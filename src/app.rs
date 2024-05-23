use anyhow::{anyhow, Result};
use clap::Parser;
use itertools::Itertools;
use std::collections::HashMap;
use std::sync::Arc;
use std::sync::atomic::{Ordering, AtomicU16};
use tokio::net::UdpSocket;
use tokio::sync::{mpsc, RwLock};
use tokio::time::{sleep, Duration};
use tracing::{debug, error};

use crate::krpc::*;

/// Joins the Bittorrent DHT network and looks up the closest nodes to itself.
#[derive(Debug, Parser, Clone)]
struct AppArgs {
    /// How many nodes to keep per k-bucket.
    #[arg(short = 'k', default_value_t = 8)]
    k: usize,

    /// How many requests to send at once.
    #[arg(short = 'j', long, default_value_t = 3)]
    concurrency: usize,

    /// Timeout for requests, in seconds.
    #[arg(long, default_value_t = 5_000)]
    timeout_ms: u64,

    /// Bootstrap node in ipv4:port format. Can be specified multiple times.
    #[arg(short, long, required = true, num_args = 1..)]
    bootstrap_node: Vec<String>,

    /// UDP address to bind to, in ipv4:port format. Use port 0 for random
    /// port.
    #[arg(long)]
    bind: String,

    /// ID to use, instead of randomly generating a new one. As hex string.
    #[arg(long)]
    id: Option<String>,
}

#[derive(Debug)]
pub struct App {
    k: usize,
    concurrency: usize,
    timeout_ms: u64,
    id: NodeId,
    bootstrap_nodes: Vec<NodeAddr>,
    sock: Arc<dyn KrpcSocket>,
    requests: Arc<RwLock<HashMap<TxId, KrpcRequest>>>,
    next_tx_id: AtomicU16,
}

impl App {
    pub async fn main() -> Result<()> {
        let args = AppArgs::parse();

        debug!(?args);

        let id = Self::parse_id(&args)?;
        let bootstrap_nodes = Self::parse_bootstrap_nodes(&args.bootstrap_node)?;
        let bind_addr = NodeAddr::from_str(&args.bind)?;
        let sock = UdpSocket::bind(&bind_addr.to_string()).await?;

        debug!(bind_addr = ?sock.local_addr());

        let app = App {
            k: args.k,
            concurrency: args.concurrency,
            timeout_ms: args.timeout_ms,
            id,
            bootstrap_nodes,
            sock: Arc::new(KrpcSocketImpl(sock)),
            requests: Arc::new(RwLock::new(HashMap::new())),
            next_tx_id: AtomicU16::new(0x6161),
        };

        let (sender_tx, sender_rx) = mpsc::channel::<KrpcMessage>(1024);
        let (main_tx, main_rx) = mpsc::channel::<KrpcMessage>(1024);

        app.main_internal(sender_tx, sender_rx, main_tx, main_rx).await
    }

    fn parse_id(args: &AppArgs) -> Result<NodeId> {
        let Some(s) = &args.id else {
            return Ok(NodeId::random(ID_LEN_BYTES));
        };

        let id = NodeId::from_hex(s)?;

        if id.len() != ID_LEN_BYTES {
            return Err(anyhow!(
                "id is of invalid length (expected {}, got {})",
                ID_LEN_BYTES, id.len()
            ));
        }

        Ok(id)
    }

    fn parse_bootstrap_nodes(values: &[String]) -> Result<Vec<NodeAddr>> {
        let mut nodes = vec![];

        for value in values.iter() {
            nodes.push(NodeAddr::from_str(value)?);
        }

        Ok(nodes)
    }

    async fn main_internal(
        &self,
        sender_tx: mpsc::Sender<KrpcMessage>,
        sender_rx: mpsc::Receiver<KrpcMessage>,
        main_tx: mpsc::Sender<KrpcMessage>,
        mut main_rx: mpsc::Receiver<KrpcMessage>,
    ) -> Result<()> {
        debug!(?self);

        let sender = KrpcSender {
            requests: self.requests.clone(),
            concurrency: self.concurrency,
            sock: self.sock.clone(),
            sender_rx,
            main_tx: main_tx.clone(),
        };

        let receiver = KrpcReceiver {
            sock: self.sock.clone(),
            main_tx: main_tx.clone(),
        };

        sender.spawn();
        receiver.spawn();

        for node_addr in self.bootstrap_nodes.iter() {
            self.request_closest_nodes_to_self(
                node_addr.clone(),
                sender_tx.clone()
            ).await?;
        }

        let mut closest_distance: Option<Distance> = None;
        let mut k_buckets: HashMap<usize, Vec<Node>> = HashMap::new();

        while let Some(msg) = main_rx.recv().await {
            debug!(?msg, "main: recv");

            match msg {
                KrpcMessage::Response(res) => {
                    let Some(_req) = self.requests
                        .write()
                        .await
                        .remove(&res.tx_id) else {
                            error!(
                                ?res,
                                "response with no matching tx_id --> ignore"
                            );

                            continue;
                        };

                    // TODO: Check that the response came from the node we sent
                    // the request to?

                    debug!(tx_id = ?res.tx_id, ?res, "response");

                    for node in res.nodes.into_iter() {
                        let distance = node.id.distance_to(&self.id);
                        let k_bucket_index = distance.lcp();

                        debug!(tx_id = ?res.tx_id, ?distance, ?node, "node");

                        let k_bucket = match k_buckets.get_mut(&k_bucket_index) {
                            Some(x) => x,
                            None => {
                                k_buckets.insert(k_bucket_index, vec![]);
                                k_buckets.get_mut(&k_bucket_index).unwrap()
                            },
                        };

                        if k_bucket.len() >= self.k {
                            debug!(
                                tx_id = ?res.tx_id,
                                "k-bucket is full --> ignore"
                            );

                            continue;
                        }

                        let distance = node.id.distance_to(&self.id);
                        let is_closer = match &closest_distance {
                            Some(value) => distance < *value,
                            None => true,
                        };

                        if is_closer {
                            debug!(
                                tx_id = ?res.tx_id,
                                "node is closer --> query"
                            );

                            closest_distance = Some(distance);

                            self.request_closest_nodes_to_self(
                                node.addr.clone(),
                                sender_tx.clone()
                            ).await?;
                        }

                        if k_bucket.iter().find(|x| **x == node).is_none() {
                            k_bucket.push(node);
                        }
                    }
                },

                KrpcMessage::SendSuccess(tx_id) => {
                    let timeout_ms = self.timeout_ms;
                    let main_tx = main_tx.clone();

                    tokio::spawn(async move {
                        sleep(Duration::from_millis(timeout_ms)).await;

                        main_tx
                            .send(KrpcMessage::ResponseTimeout(tx_id))
                            .await
                            .unwrap();
                    });
                },

                KrpcMessage::SendError(tx_id) => {
                    if let Some(_req) = self.requests
                        .write()
                        .await
                        .remove(&tx_id)
                    {
                        error!(?tx_id, "send error --> remove");
                    }
                },

                KrpcMessage::ResponseTimeout(tx_id) => {
                    if let Some(_req) = self.requests
                        .write()
                        .await
                        .remove(&tx_id)
                    {
                        error!(?tx_id, "timeout --> remove");
                    }
                },

                KrpcMessage::Exit => {
                    debug!("exit");
                    return Ok(());
                },

                _ => {},
            }

            if self.requests.read().await.is_empty() {
                debug!("done");

                for key in k_buckets.keys().sorted() {
                    let bucket = k_buckets.get(key).unwrap();
                    println!("k-bucket {}: {:#?}", key, bucket);
                }

                return Ok(());
            }
        }

        Ok(())
    }

    async fn request_closest_nodes_to_self(
        &self,
        dst: NodeAddr,
        sender_tx: mpsc::Sender<KrpcMessage>,
    ) -> Result<()> {
        let tx_id = self.create_tx_id();
        let req = KrpcRequest {
            dst,
            payload: FindNodeRequest {
                tx_id: tx_id.clone(),
                node_id_self: self.id.clone(),
                node_id_target: self.id.clone(),
            },
            in_progress: false,
        };

        self
            .requests
            .write()
            .await
            .insert(tx_id.clone(), req);

        sender_tx.send(KrpcMessage::Request(tx_id)).await?;

        Ok(())
    }

    fn create_tx_id(&self) -> TxId {
        TxId::from_u16(
            self.next_tx_id.fetch_add(1, Ordering::Relaxed)
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug, Default)]
    struct Setup {
        bootstrap_nodes: Option<Vec<NodeAddr>>,
        node_id: Option<NodeId>,
    }

    #[allow(dead_code)]
    #[derive(Debug)]
    struct SetupResult {
        requests: Arc<RwLock<HashMap<TxId, KrpcRequest>>>,
        task: tokio::task::JoinHandle<Result<()>>,
        sender_rx: mpsc::Receiver<KrpcMessage>,
        main_tx: mpsc::Sender<KrpcMessage>,
    }

    impl Setup {
        fn new() -> Self {
            Self::default()
        }

        fn bootstrap_nodes(mut self, value: Vec<NodeAddr>) -> Self {
            self.bootstrap_nodes = Some(value);
            self
        }

        fn node_id(mut self, value: NodeId) -> Self {
            self.node_id = Some(value);
            self
        }

        fn execute(self) -> SetupResult {
            with_tracing();

            let (sender_tx, sender_rx) = mpsc::channel::<KrpcMessage>(1024);
            let (_, dummy_sender_rx) = mpsc::channel::<KrpcMessage>(1024);
            let (main_tx, main_rx) = mpsc::channel::<KrpcMessage>(1024);
            let main_tx_cloned = main_tx.clone();

            let requests = Arc::new(RwLock::new(HashMap::new()));
            let requests_cloned = requests.clone();

            let bootstrap_nodes = self.bootstrap_nodes.unwrap_or(vec![
                NodeAddr::from_str("127.0.0.1:1000").unwrap()
            ]);

            let node_id = self.node_id.unwrap_or_else(||
                NodeId::random(ID_LEN_BYTES)
            );

            let task = tokio::spawn(async move {
                let app = App {
                    k: 8,
                    concurrency: 999, // doesn't matter
                    timeout_ms: 1_000,
                    id: node_id,
                    bootstrap_nodes,
                    sock: Arc::new(KrpcSocketStub),
                    requests: requests_cloned,
                    next_tx_id: AtomicU16::new(0),
                };

                app.main_internal(
                    sender_tx,
                    dummy_sender_rx,
                    main_tx_cloned,
                    main_rx
                ).await
            });

            SetupResult {
                requests,
                task,
                sender_rx,
                main_tx,
            }
        }
    }

    fn with_tracing() {
        use tracing_subscriber::FmtSubscriber;

        let subscriber = FmtSubscriber::builder()
            .with_max_level(tracing::Level::DEBUG)
            .finish();

        let _ = tracing::subscriber::set_global_default(subscriber);
    }

    #[tokio::test]
    async fn stops_after_bootstrapping_when_all_requests_failed() -> Result<()> {
        let mut s = Setup::new()
            .bootstrap_nodes(vec![
                NodeAddr::from_str("127.0.0.1:1000").unwrap(),
                NodeAddr::from_str("127.0.0.1:1001").unwrap(),
            ])
            .execute();

        let mut requests_sent = vec![];

        while let Some(msg) = s.sender_rx.recv().await {
            match msg {
                KrpcMessage::Request(tx_id) => {
                    requests_sent.push(tx_id.clone());
                    s.main_tx.send(KrpcMessage::SendError(tx_id)).await?;
                },
                _ => {},
            }
        }

        assert_eq!(2, requests_sent.len());

        let _ = s.task.await?;
        Ok(())
    }

    #[tokio::test]
    async fn queries_closer_nodes_on_response() -> Result<()> {
        let mut s = Setup::new()
            .node_id(NodeId::from_hex("ffffffffffffffffffffffffffffffffffffffff").unwrap())
            .execute();

        let mut requests_sent = vec![];

        while let Some(msg) = s.sender_rx.recv().await {
            match msg {
                KrpcMessage::Request(tx_id) if tx_id == TxId::from_u16(0) => {
                    requests_sent.push(tx_id.clone());

                    s.main_tx.send(KrpcMessage::SendSuccess(tx_id.clone())).await?;
                    s.main_tx.send(KrpcMessage::Response(FindNodeResponse {
                        tx_id,
                        nodes: vec![Node {
                            id: NodeId::from_hex("ffffffffff000000000000000000000000000000").unwrap(),
                            addr: NodeAddr::from_str("127.0.0.1:2000").unwrap(),
                        }]
                    })).await?;
                },

                KrpcMessage::Request(tx_id) if tx_id == TxId::from_u16(1) => {
                    requests_sent.push(tx_id.clone());

                    s.main_tx.send(KrpcMessage::SendSuccess(tx_id.clone())).await?;
                    s.main_tx.send(KrpcMessage::Response(FindNodeResponse {
                        tx_id,
                        nodes: vec![Node {
                            id: NodeId::from_hex("ffffffffffffffffffff00000000000000000000").unwrap(),
                            addr: NodeAddr::from_str("127.0.0.1:2001").unwrap(),
                        }]
                    })).await?;
                },

                KrpcMessage::Request(tx_id) if tx_id == TxId::from_u16(2) => {
                    requests_sent.push(tx_id.clone());

                    // not closer, so this won't lead to another query

                    s.main_tx.send(KrpcMessage::SendSuccess(tx_id.clone())).await?;
                    s.main_tx.send(KrpcMessage::Response(FindNodeResponse {
                        tx_id,
                        nodes: vec![Node {
                            id: NodeId::from_hex("fffffffffffffff0000000000000000000000000").unwrap(),
                            addr: NodeAddr::from_str("127.0.0.1:2002").unwrap(),
                        }]
                    })).await?;
                },

                KrpcMessage::Request(tx_id) => {
                    requests_sent.push(tx_id.clone());
                    s.main_tx.send(KrpcMessage::SendError(tx_id.clone())).await?;
                },

                _ => {},
            }
        }

        assert_eq!(3, requests_sent.len());

        // TODO: Assert the k-buckets. Either grap the k_buckets directly from
        // App or add a callback (listener) for when the main loop exits (where
        // it currently dumps the k_buckets to stdout).

        let _ = s.task.await?;
        Ok(())
    }
}
