use async_trait::async_trait;
use anyhow::{anyhow, Context, Result};
use bendy::{decoding::FromBencode, encoding::ToBencode};
use rand::RngCore;
use std::collections::HashMap;
use std::marker::{Send, Sync};
use std::sync::Arc;
use std::{net::Ipv4Addr, str::FromStr};
use tokio::sync::{mpsc, RwLock};
use tokio::time::{sleep, Duration};
use tracing::{debug, error};
use tokio::net::UdpSocket;

pub const ID_LEN_BYTES: usize = 20;
pub const ID_LEN_BITS: usize = ID_LEN_BYTES * 8;
pub const ID_LEN_HEX: usize = ID_LEN_BYTES * 2;

const IPV4_LEN_BYTES: usize = 4;
const PORT_LEN_BYTES: usize = 2;
const NODE_LEN_BYTES: usize = ID_LEN_BYTES + IPV4_LEN_BYTES + PORT_LEN_BYTES;

#[derive(Clone, PartialEq, Default, Hash, Eq)]
pub struct TxId(Vec<u8>);

impl TxId {
    pub fn new(src: impl Into<Vec<u8>>) -> Self {
        Self(src.into())
    }

    pub fn from_u16(src: u16) -> Self {
        Self(src.to_ne_bytes().to_vec())
    }

    pub fn from_hex(src: &str) -> anyhow::Result<Self> {
        let bytes: Vec<u8> = hex::FromHex
            ::from_hex(src)
            .with_context(|| format!("failed to parse id: {}", src))?;

        Ok(Self(bytes))
    }

    pub fn from_str(src: &str) -> anyhow::Result<Self> {
        Ok(Self(src.as_bytes().to_vec()))
    }

    pub fn as_slice(&self) -> &[u8] {
        self.0.as_slice()
    }
}

impl std::fmt::Debug for TxId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "TxId({})", hex::encode(&self.0))
    }
}

#[derive(Clone, PartialEq)]
pub struct NodeId(Vec<u8>);

impl NodeId {
    pub fn from_hex(src: &str) -> anyhow::Result<Self> {
        let bytes: Vec<u8> = hex::FromHex
            ::from_hex(src)
            .with_context(|| format!("failed to parse id: {}", src))?;

        Ok(Self(bytes))
    }

    pub fn from_str(src: &str) -> anyhow::Result<Self> {
        Ok(Self(src.as_bytes().to_vec()))
    }

    pub fn as_slice(&self) -> &[u8] {
        self.0.as_slice()
    }

    pub fn random(length: usize) -> Self {
        let mut bytes = vec![0; length];

        rand
            ::thread_rng()
            .fill_bytes(&mut bytes);

        Self(bytes)
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn distance_to(&self, other: &NodeId) -> Distance {
        assert_eq!(self.0.len(), other.0.len());

        let value: Vec<u8> = self.0.iter()
            .zip(other.0.iter())
            .map(|(a, b)| a ^ b)
            .collect();

        Distance(value)
    }
}

impl ToString for NodeId {
    fn to_string(&self) -> String {
        hex::encode(&self.0)
    }
}

impl std::fmt::Debug for NodeId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "NodeId({})", hex::encode(&self.0))
    }
}

#[derive(Clone, PartialEq, PartialOrd)]
pub struct Distance(Vec<u8>);

impl std::fmt::Debug for Distance {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f, "Distance(LCP: {}, value: {})",
            self.lcp(), hex::encode(&self.0)
        )
    }
}

impl Distance {
    /// Longest common previx. Used to determine the k-bucket. Longer prefix
    /// means shorter distance.
    pub fn lcp(&self) -> usize {
        let mut value: usize = 0;

        for byte in self.0.iter() {
            if *byte == 0 {
                value += 8;
                continue;
            }

            value += byte.leading_zeros() as usize;
            break;
        }

        value
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Node {
    pub id: NodeId,
    pub addr: NodeAddr,
}

#[derive(Debug, Clone, PartialEq)]
pub struct NodeAddr {
    pub ip: Ipv4Addr,
    pub port: u16,
}

impl NodeAddr {
    pub fn new(ip: Ipv4Addr, port: u16) -> Self {
        Self { ip, port }
    }

    pub fn from_compact_node_id(value: &[u8]) -> Result<Self> {
        if value.len() != IPV4_LEN_BYTES + PORT_LEN_BYTES {
            return Err(anyhow!("invalid compact node id: {:?}", value));
        }

        let (ip_bytes, port_bytes) = value.split_at(IPV4_LEN_BYTES);

        let ip =
            Ipv4Addr::new(ip_bytes[0], ip_bytes[1], ip_bytes[2], ip_bytes[3]);

        let port = {
            use byteorder::*;
            NetworkEndian::read_u16(&port_bytes)
        };

        Ok(Self { ip, port })
    }

    pub fn from_str(value: &str) -> Result<Self> {
        let parts: Vec<&str> = value.split(":").collect();

        if parts.len() != 2 {
            return Err(anyhow!("invalid ip/port: {:?}", value));
        }

        let ip = Ipv4Addr::from_str(parts[0])
            .with_context(|| format!("invalid ip: {:?}", parts[0]))?;

        let port = u16::from_str(parts[1])
            .with_context(|| format!("invalid port: {:?}", parts[1]))?;

        Ok(Self { ip, port })
    }
}

impl ToString for NodeAddr {
    fn to_string(&self) -> String {
        format!("{}:{}", self.ip.to_string(), self.port)
    }
}

#[derive(Debug, Clone)]
pub struct FindNodeRequest {
    pub tx_id: TxId,
    pub node_id_self: NodeId,
    pub node_id_target: NodeId,
}

const KEY_MESSAGE_TYPE: &[u8] = b"y";
const KEY_QUERY_ARGUMENTS: &[u8] = b"a";
const KEY_QUERY_METHOD_NAME: &[u8] = b"q";
const KEY_RETURN_VALUES: &[u8] = b"r";
const KEY_TRANSACTION_ID: &[u8] = b"t";

impl ToBencode for FindNodeRequest {
    const MAX_DEPTH: usize = 5;

    fn encode(&self, e: bendy::encoding::SingleItemEncoder) -> Result<(), bendy::encoding::Error> {
        e.emit_dict(|mut e| {
            e.emit_pair_with(KEY_QUERY_ARGUMENTS, |e| {
                e.emit_dict(|mut e| {
                    e.emit_pair_with(b"id", |e| {
                        e.emit_bytes(&self.node_id_self.as_slice())
                    })?;

                    e.emit_pair_with(b"target", |e| {
                        e.emit_bytes(&self.node_id_target.as_slice())
                    })
                })
            })?;

            e.emit_pair(KEY_QUERY_METHOD_NAME, &"find_node")?;

            e.emit_pair_with(KEY_TRANSACTION_ID, |e| {
                e.emit_bytes(self.tx_id.as_slice())
            })?;

            e.emit_pair(KEY_MESSAGE_TYPE, &"q")?;

            Ok(())
        })
    }
}

#[derive(Debug, Default, PartialEq)]
pub struct FindNodeResponse {
    pub tx_id: TxId,
    pub nodes: Vec<Node>,
}

#[derive(Debug, Default)]
struct FindNodeResponseBuilder {
    tx_id: Option<TxId>,
    message_type: Option<String>,
    node_id: Option<Vec<u8>>,
    nodes: Option<Vec<u8>>,
}

impl FindNodeResponse {
    pub fn from_bencode(src: &[u8]) -> Result<Self> {
        FindNodeResponseBuilder
            ::from_bencode(src)
            .map_err(|e| anyhow!("decoding failed: {:?}", e))?
            .build()
    }
}

impl FindNodeResponseBuilder {
    fn set_tx_id(&mut self, value: TxId) {
        self.tx_id = Some(value);
    }

    fn set_message_type(&mut self, value: String) {
        self.message_type = Some(value);
    }

    fn set_node_id(&mut self, value: Vec<u8>) {
        self.node_id = Some(value);
    }

    fn set_nodes(&mut self, value: Vec<u8>) {
        self.nodes = Some(value);
    }

    fn build(self) -> Result<FindNodeResponse> {
        let Some(tx_id) = self.tx_id else {
            return Err(anyhow!("tx_id missing"));
        };

        let Some(message_type) = self.message_type else {
            return Err(anyhow!("message_type missing"));
        };

        if message_type != "r" {
            return Err(anyhow!("wrong message type"));
        }

        let Some(compact_nodes) = self.nodes else {
            return Err(anyhow!("nodes missing"));
        };

        if compact_nodes.len() % NODE_LEN_BYTES != 0 {
            return Err(anyhow!(
                "nodes invalid length ({})",
                compact_nodes.len()
            ));
        }

        let mut nodes = vec![];

        for chunk in compact_nodes.chunks_exact(NODE_LEN_BYTES) {
            let (id, ip_and_port) = chunk.split_at(ID_LEN_BYTES);
            let id = NodeId(id.to_vec());
            let addr = NodeAddr::from_compact_node_id(ip_and_port)?;

            nodes.push(Node { id, addr });
        }

        Ok(FindNodeResponse {
            tx_id,
            nodes,
        })
    }
}

impl FromBencode for FindNodeResponseBuilder {
    fn decode_bencode_object(object: bendy::decoding::Object) -> Result<Self, bendy::decoding::Error>
    where Self: Sized {
        let mut builder = FindNodeResponseBuilder::default();
        let mut dict = object.try_into_dictionary()?;

        while let Some(pair) = dict.next_pair()? {
            match pair {
                (KEY_TRANSACTION_ID, value) => {
                    builder.set_tx_id(TxId(value.try_into_bytes()?.to_vec()));
                },

                (KEY_MESSAGE_TYPE, value) => {
                    builder.set_message_type(String::decode_bencode_object(value)?);
                },

                (KEY_RETURN_VALUES, value) => {
                    let mut dict = value.try_into_dictionary()?;

                    while let Some(pair) = dict.next_pair()? {
                        match pair {
                            (b"id", value) => {
                                builder.set_node_id(value.try_into_bytes()?.to_vec());
                            },

                            (b"nodes", value) => {
                                builder.set_nodes(value.try_into_bytes()?.to_vec());
                            },

                            _ => {},
                        }
                    }
                },

                _ => {},
            }
        }

        Ok(builder)
    }
}

#[derive(Debug, Clone)]
pub struct KrpcRequest {
    pub dst: NodeAddr,
    pub payload: FindNodeRequest,
    pub in_progress: bool,
}

#[derive(Debug)]
pub enum KrpcMessage {
    Request(TxId),
    Response(FindNodeResponse),
    ResponseTimeout(TxId),
    SendSuccess(TxId),
    SendError(TxId),
    #[allow(dead_code)] Exit,
}

#[derive(Debug)]
pub struct KrpcSender {
    pub requests: Arc<RwLock<HashMap<TxId, KrpcRequest>>>,
    pub concurrency: usize,
    pub sender_rx: mpsc::Receiver<KrpcMessage>,
    pub main_tx: mpsc::Sender<KrpcMessage>,
    pub sock: Arc<dyn KrpcSocket>,
}

impl KrpcSender {
    pub fn spawn(mut self) {
        tokio::spawn(async move { self.run().await });
    }

    async fn run(&mut self) -> Result<()> {
        while let Some(msg) = self.sender_rx.recv().await {
            self.handle_message(msg).await?
        }

        Ok(())
    }

    async fn handle_message(&self, msg: KrpcMessage) -> Result<()> {
        debug!(?msg, "sender: recv");

        let KrpcMessage::Request(tx_id) = msg else {
            return Ok(());
        };

        self.wait_until_below_concurrency_limit().await;

        let req = {
            let mut requests = self.requests.write().await;
            let req_mut = requests.get_mut(&tx_id).unwrap();
            req_mut.in_progress = true;
            req_mut.clone()
        };

        let data = FindNodeRequest::to_bencode(&req.payload).unwrap();

        match self.sock.send_to(&data, req.dst.to_string()).await {
            Ok(_) => {
                debug!(
                    dst = ?req.dst,
                    req = ?req.payload,
                    "sender: send ok"
                );

                self.main_tx
                    .send(KrpcMessage::SendSuccess(req.payload.tx_id))
                    .await
                    .unwrap();
            },

            Err(e) => {
                error!(
                    dst = ?req.dst,
                    req = ?req.payload,
                    err = ?e,
                    "sender: send error"
                );

                self.main_tx
                    .send(KrpcMessage::SendError(req.payload.tx_id))
                    .await
                    .unwrap();
            },
        }

        Ok(())
    }

    async fn wait_until_below_concurrency_limit(&self) {
        loop {
            let count = {
                self.requests
                    .read()
                    .await
                    .iter()
                    .filter(|(_k, v)| v.in_progress)
                    .count()
            };

            if count < self.concurrency {
                break;
            }

            sleep(Duration::from_millis(100)).await;
        }
    }
}

#[derive(Debug)]
pub struct KrpcReceiver {
    pub sock: Arc<dyn KrpcSocket>,
    pub main_tx: mpsc::Sender<KrpcMessage>,
}

impl KrpcReceiver {
    pub fn spawn(self) {
        tokio::spawn(async move { self.run().await });
    }

    async fn run(&self) -> Result<()> {
        let mut buf = [0; 4096];

        loop {
            let (len, addr) = self.sock.recv_from(&mut buf).await.unwrap();
            let data = &buf[0..len];
            let data_hex = hex::encode(&data);

            debug!(src = ?addr, ?len, ?data_hex, "receiver: recv");

            self.handle_data(&data).await?;
        }
    }

    async fn handle_data(&self, data: &[u8]) -> Result<()> {
        let Ok(res) = FindNodeResponse::from_bencode(data) else {
            return Ok(()); // only this message type is supported
        };

        self.main_tx.send(KrpcMessage::Response(res)).await?;

        Ok(())
    }
}


#[async_trait]
pub trait KrpcSocket: std::fmt::Debug + Send + Sync + 'static{
    async fn recv_from(&self, buf: &mut [u8]) -> tokio::io::Result<(usize, core::net::SocketAddr)>;
    async fn send_to(&self, buf: &[u8], target: String) -> tokio::io::Result<usize>;
}

#[derive(Debug)]
pub struct KrpcSocketImpl(pub UdpSocket);

#[async_trait]
impl KrpcSocket for KrpcSocketImpl {
    async fn recv_from(&self, buf: &mut [u8]) -> tokio::io::Result<(usize, core::net::SocketAddr)> {
        self.0.recv_from(buf).await
    }

    async fn send_to(&self, buf: &[u8], target: String) -> tokio::io::Result<usize> {
        self.0.send_to(buf, target).await
    }
}

#[derive(Debug)]
pub struct KrpcSocketStub;

#[async_trait]
impl KrpcSocket for KrpcSocketStub {
    async fn recv_from(&self, _buf: &mut [u8]) -> tokio::io::Result<(usize, core::net::SocketAddr)> {
        futures::future::pending().await
    }

    async fn send_to(&self, buf: &[u8], _target: String) -> tokio::io::Result<usize> {
        Ok(buf.len())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn calculates_distance() {
        assert!(Distance(vec![1, 1, 1]) > Distance(vec![1, 0, 0]));
        assert!(Distance(vec![1, 1, 0]) > Distance(vec![1, 0, 0]));
        assert!(Distance(vec![1, 0, 1]) > Distance(vec![1, 0, 0]));

        assert!(Distance(vec![1, 0, 0]) == Distance(vec![1, 0, 0]));
        assert!(Distance(vec![0, 1, 0]) == Distance(vec![0, 1, 0]));

        assert!(Distance(vec![0, 1, 1]) < Distance(vec![1, 0, 0]));
        assert!(Distance(vec![0, 1, 0]) < Distance(vec![1, 0, 0]));
        assert!(Distance(vec![0, 0, 1]) < Distance(vec![1, 0, 0]));
        assert!(Distance(vec![0, 0, 0]) < Distance(vec![1, 0, 0]));
    }

    #[test]
    fn calculates_kongest_common_prefix() {
        assert_eq!(Distance(vec![0b00000000, 0b00000000]).lcp(), 16);
        assert_eq!(Distance(vec![0b00000000, 0b00000001]).lcp(), 15);
        assert_eq!(Distance(vec![0b00000000, 0b00000010]).lcp(), 14);
        assert_eq!(Distance(vec![0b00000000, 0b00000100]).lcp(), 13);
        assert_eq!(Distance(vec![0b00000000, 0b00001000]).lcp(), 12);
        assert_eq!(Distance(vec![0b00000000, 0b00010000]).lcp(), 11);
        assert_eq!(Distance(vec![0b00000000, 0b00100000]).lcp(), 10);
        assert_eq!(Distance(vec![0b00000000, 0b01000000]).lcp(), 9);
        assert_eq!(Distance(vec![0b00000000, 0b10000000]).lcp(), 8);
        assert_eq!(Distance(vec![0b00000001, 0b00000000]).lcp(), 7);
        assert_eq!(Distance(vec![0b00000010, 0b00000000]).lcp(), 6);
        assert_eq!(Distance(vec![0b00000100, 0b00000000]).lcp(), 5);
        assert_eq!(Distance(vec![0b00001000, 0b00000000]).lcp(), 4);
        assert_eq!(Distance(vec![0b00010000, 0b00000000]).lcp(), 3);
        assert_eq!(Distance(vec![0b00100000, 0b00000000]).lcp(), 2);
        assert_eq!(Distance(vec![0b01000000, 0b00000000]).lcp(), 1);
        assert_eq!(Distance(vec![0b10000000, 0b00000000]).lcp(), 0);

        assert_eq!(Distance(vec![0b00011111, 0b11111111]).lcp(), 3);
        assert_eq!(Distance(vec![0b11111111, 0b11111111]).lcp(), 0);
    }

    // NOTE: While real KRPC messages may not stringify properly because they
    // contain raw byte data, it's simpler for testing purposes to use messages
    // that only contain ASCII. They're still valid messages, as per the spec:
    //
    // http://www.bittorrent.org/beps/bep_0005.html

    #[test]
    fn encodes_find_node_request() {
        // Query = {
        //   "t": "aa"
        //   "y": "q",
        //   "q": "find_node",
        //   "a": {
        //     "id": "Aihoi6iC6Oowo0quor1j",
        //     "target": "Viefohchaog3shoh7qui"
        //   }
        // }

        let req = FindNodeRequest {
            tx_id: TxId::from_str("aa").unwrap(),
            node_id_self: NodeId::from_str("Aihoi6iC6Oowo0quor1j").unwrap(),
            node_id_target: NodeId::from_str("Viefohchaog3shoh7qui").unwrap(),
        };

        similar_asserts::assert_eq!(
            String::from_utf8(
                req.to_bencode().unwrap()
            ).unwrap(),
            "d1:ad2:id20:Aihoi6iC6Oowo0quor1j6:target20:Viefohchaog3shoh7quie1:q9:find_node1:t2:aa1:y1:qe".to_string()
        );
    }

    #[test]
    fn decodes_find_node_response_with_single_node() {
        // Response = {
        //   "t": "aa"
        //   "y": "r",
        //   "r": {
        //     "id": "Viefohchaog3shoh7qui",
        //
        //     // id    goo8quuireJohQuooseh
        //     // ip    97.98.99.100
        //     // port  25958 dec (6566 hex)
        //
        //     "nodes": "goo8quuireJohQuoosehabcdef"
        //   }
        // }

        similar_asserts::assert_eq!(
            FindNodeResponse::from_bencode(
                "d1:rd2:id20:Viefohchaog3shoh7qui5:nodes26:goo8quuireJohQuoosehabcdefe1:t2:aa1:y1:re".as_bytes()
            ).unwrap(),

            FindNodeResponse {
                tx_id: TxId::from_str("aa").unwrap(),
                nodes: vec![
                    Node {
                        id: NodeId::from_str("goo8quuireJohQuooseh").unwrap(),
                        addr: NodeAddr::from_str("97.98.99.100:25958").unwrap()
                    }
                ],
            }
        );
    }

    #[test]
    fn decodes_find_node_response_with_two_nodes() {
        // Response = {
        //   "t": "aa"
        //   "y": "r",
        //   "r": {
        //     "id": "Viefohchaog3shoh7qui",
        //
        //     // id    goo8quuireJohQuooseh
        //     // ip    97.98.99.100
        //     // port  25958 dec (6566 hex)
        //
        //     // id    raiRac8heb3Ye3naingi
        //     // ip    98.99.100.101
        //     // port  26215 dec (6667 hex)
        //
        //     "nodes": "goo8quuireJohQuoosehabcdefraiRac8heb3Ye3naingibcdefg"
        //   }
        // }

        similar_asserts::assert_eq!(
            FindNodeResponse::from_bencode(
                "d1:rd2:id20:Viefohchaog3shoh7qui5:nodes52:goo8quuireJohQuoosehabcdefraiRac8heb3Ye3naingibcdefge1:t2:aa1:y1:re".as_bytes()
            ).unwrap(),

            FindNodeResponse {
                tx_id: TxId::from_str("aa").unwrap(),
                nodes: vec![
                    Node {
                        id: NodeId::from_str("goo8quuireJohQuooseh").unwrap(),
                        addr: NodeAddr::from_str("97.98.99.100:25958").unwrap()
                    },
                    Node {
                        id: NodeId::from_str("raiRac8heb3Ye3naingi").unwrap(),
                        addr: NodeAddr::from_str("98.99.100.101:26215").unwrap()
                    },
                ],
            }
        );
    }
}
