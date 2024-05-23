# kademliar

This is a partial implementation of Kademlia used in Bittorrent DHT based on the [spec](http://www.bittorrent.org/beps/bep_0005.html). 

It performs a network join through the specified bootstrap nodes and recursively searches for the nodes closest to itself. Once no more closer nodes are found, it dumps the assembled k-buckets and exits. 

It does not respond to any queries and only accepts responses to the `find_node` queries it sends.

## How to use

```
./e2e-public-network.sh <address>
```

where `address` is in the format of `1.2.3.4:5678`. The IP is either your
public IP or your LAN IP if you're behind NAT. You can use port `0` for a
random port.

Output will look something like this

```
2024-05-23T20:44:20.808488Z DEBUG kademliar::app: args=AppArgs { k: 8, concurrency: 3, timeout_ms: 5000, bootstrap_node: ["67.215.246.10:6881", "82.221.103.244:6881", "212.129.33.59:6881"], bind: "192.168.133.20:0", id: None }
2024-05-23T20:44:20.808618Z DEBUG kademliar::app: bind_addr=Ok(192.168.133.20:55611)
2024-05-23T20:44:20.808638Z DEBUG kademliar::app: self=App { k: 8, concurrency: 3, timeout_ms: 5000, id: NodeId(a63a91e78eff828189c60e7bed79d212c1832f06), bootstrap_nodes: [NodeAddr { ip: 67.215.246.10, port: 6881 }, NodeAddr { ip: 82.221.103.244, port: 6881 }, NodeAddr { ip: 212.129.33.59, port: 6881 }], sock: KrpcSocketImpl(PollEvented { io: Some(UdpSocket { addr: 192.168.133.20:55611, fd: 9 }) }), requests: RwLock { data: {} }, next_tx_id: 24929 }
2024-05-23T20:44:20.808712Z DEBUG kademliar::krpc: sender: recv msg=Request(TxId(6161))
2024-05-23T20:44:20.808802Z DEBUG kademliar::krpc: sender: send ok dst=NodeAddr { ip: 67.215.246.10, port: 6881 } req=FindNodeRequest { tx_id: TxId(6161), node_id_self: NodeId(a63a91e78eff828189c60e7bed79d212c1832f06), node_id_target: NodeId(a63a91e78eff828189c60e7bed79d212c1832f06) }
2024-05-23T20:44:20.808828Z DEBUG kademliar::krpc: sender: recv msg=Request(TxId(6261))
2024-05-23T20:44:20.808834Z DEBUG kademliar::app: main: recv msg=SendSuccess(TxId(6161))
2024-05-23T20:44:20.808861Z DEBUG kademliar::krpc: sender: send ok dst=NodeAddr { ip: 82.221.103.244, port: 6881 } req=FindNodeRequest { tx_id: TxId(6261), node_id_self: NodeId(a63a91e78eff828189c60e7bed79d212c1832f06), node_id_target: NodeId(a63a91e78eff828189c60e7bed79d212c1832f06) }
2024-05-23T20:44:20.808883Z DEBUG kademliar::app: main: recv msg=SendSuccess(TxId(6261))
2024-05-23T20:44:20.808881Z DEBUG kademliar::krpc: sender: recv msg=Request(TxId(6361))
2024-05-23T20:44:20.808922Z DEBUG kademliar::krpc: sender: send ok dst=NodeAddr { ip: 212.129.33.59, port: 6881 } req=FindNodeRequest { tx_id: TxId(6361), node_id_self: NodeId(a63a91e78eff828189c60e7bed79d212c1832f06), node_id_target: NodeId(a63a91e78eff828189c60e7bed79d212c1832f06) }
2024-05-23T20:44:20.808944Z DEBUG kademliar::app: main: recv msg=SendSuccess(TxId(6361))
2024-05-23T20:44:20.985548Z DEBUG kademliar::krpc: receiver: recv src=67.215.246.10:6881 len=486 data_hex="64323a6970363a54f80421d93b313a7264323a696432303a32f54e697351ff4aec29cdbaabf2fbe3467cc267353a6e6f6465733431363a129791afac2ff0ffaf2d96818e30affad62eefd797f02427615881e5906b5bf0d9b9badbda50cc60612049ecfba3b17c9a2a8b01c9c07155effad392af5cd0e5125ad7fd298133bbbad8b7e357646d49c19da949e86261b4dd0a9c377802076b3195b1b50363529cb454d76d0fcb67f70a239fa3ecce2c2d58c4bf50b09149143e68ad9a0269161cc04abb3f1c7584961565fdec1a72b0bed1f90c4e4e4c42b9154bcc193819750bd0be623220761a96ac3ba1401168d2ef43179e0fc8e0f398fdb91973d2d555735694b52c81424ab24077169214e1cee4ce59063a74b6e3c7d5418a7d29696f94a346c1587ad970ec6f8e044bae7605556984c2eb3368bea4d8161ae1fa9bece9c949623fb4772488439011db769cf4d8b127560c786d2b5779bbb23c29bfb258e51a6c66c9bf0d95760fb57c86fe33b79ace994c70d4165087e60a1763bb25a2a2f70a4d5207a0e01ae1b8815434b1e68b07e2b6cddf5ee18f5a84e1469b57436f3e042ba3bd6ecf5c1b227798eca92c35bc1b3e1b57fbd08049aa419a204caa4d322d478549f43cd9766df9c8430a638ad14d16b432cc1565313a74323a6161313a79313a7265"
2024-05-23T20:44:20.985682Z DEBUG kademliar::app: main: recv msg=Response(FindNodeResponse { tx_id: TxId(6161), nodes: [Node { id: NodeId(129791afac2ff0ffaf2d96818e30affad62eefd7), addr: NodeAddr { ip: 151.240.36.39, port: 24920 } }, Node { id: NodeId(81e5906b5bf0d9b9badbda50cc60612049ecfba3), addr: NodeAddr { ip: 177.124.154.42, port: 35585 } }, Node { id: NodeId(c9c07155effad392af5cd0e5125ad7fd298133bb), addr: NodeAddr { ip: 186.216.183.227, port: 22372 } }, Node { id: NodeId(6d49c19da949e86261b4dd0a9c377802076b3195), addr: NodeAddr { ip: 177.181.3.99, port: 21148 } }, Node { id: NodeId(b454d76d0fcb67f70a239fa3ecce2c2d58c4bf50), addr: NodeAddr { ip: 176.145.73.20, port: 15976 } }, Node { id: NodeId(ad9a0269161cc04abb3f1c7584961565fdec1a72), addr: NodeAddr { ip: 176.190.209.249, port: 3150 } }, Node { id: NodeId(4e4c42b9154bcc193819750bd0be623220761a96), addr: NodeAddr { ip: 172.59.161.64, port: 4456 } }, Node { id: NodeId(d2ef43179e0fc8e0f398fdb91973d2d555735694), addr: NodeAddr { ip: 181.44.129.66, port: 19122 } }, Node { id: NodeId(4077169214e1cee4ce59063a74b6e3c7d5418a7d), addr: NodeAddr { ip: 41.105.111.148, port: 41798 } }, Node { id: NodeId(c1587ad970ec6f8e044bae7605556984c2eb3368), addr: NodeAddr { ip: 190.164.216.22, port: 6881 } }, Node { id: NodeId(fa9bece9c949623fb4772488439011db769cf4d8), addr: NodeAddr { ip: 177.39.86.12, port: 30829 } }, Node { id: NodeId(2b5779bbb23c29bfb258e51a6c66c9bf0d95760f), addr: NodeAddr { ip: 181.124.134.254, port: 13239 } }, Node { id: NodeId(9ace994c70d4165087e60a1763bb25a2a2f70a4d), addr: NodeAddr { ip: 82.7.160.224, port: 6881 } }, Node { id: NodeId(b8815434b1e68b07e2b6cddf5ee18f5a84e1469b), addr: NodeAddr { ip: 87.67.111.62, port: 1067 } }, Node { id: NodeId(a3bd6ecf5c1b227798eca92c35bc1b3e1b57fbd0), addr: NodeAddr { ip: 128.73.170.65, port: 39456 } }, Node { id: NodeId(4caa4d322d478549f43cd9766df9c8430a638ad1), addr: NodeAddr { ip: 77.22.180.50, port: 52245 } }] })
2024-05-23T20:44:20.985741Z DEBUG kademliar::app: response tx_id=TxId(6161) res=FindNodeResponse { tx_id: TxId(6161), nodes: [Node { id: NodeId(129791afac2ff0ffaf2d96818e30affad62eefd7), addr: NodeAddr { ip: 151.240.36.39, port: 24920 } }, Node { id: NodeId(81e5906b5bf0d9b9badbda50cc60612049ecfba3), addr: NodeAddr { ip: 177.124.154.42, port: 35585 } }, Node { id: NodeId(c9c07155effad392af5cd0e5125ad7fd298133bb), addr: NodeAddr { ip: 186.216.183.227, port: 22372 } }, Node { id: NodeId(6d49c19da949e86261b4dd0a9c377802076b3195), addr: NodeAddr { ip: 177.181.3.99, port: 21148 } }, Node { id: NodeId(b454d76d0fcb67f70a239fa3ecce2c2d58c4bf50), addr: NodeAddr { ip: 176.145.73.20, port: 15976 } }, Node { id: NodeId(ad9a0269161cc04abb3f1c7584961565fdec1a72), addr: NodeAddr { ip: 176.190.209.249, port: 3150 } }, Node { id: NodeId(4e4c42b9154bcc193819750bd0be623220761a96), addr: NodeAddr { ip: 172.59.161.64, port: 4456 } }, Node { id: NodeId(d2ef43179e0fc8e0f398fdb91973d2d555735694), addr: NodeAddr { ip: 181.44.129.66, port: 19122 } }, Node { id: NodeId(4077169214e1cee4ce59063a74b6e3c7d5418a7d), addr: NodeAddr { ip: 41.105.111.148, port: 41798 } }, Node { id: NodeId(c1587ad970ec6f8e044bae7605556984c2eb3368), addr: NodeAddr { ip: 190.164.216.22, port: 6881 } }, Node { id: NodeId(fa9bece9c949623fb4772488439011db769cf4d8), addr: NodeAddr { ip: 177.39.86.12, port: 30829 } }, Node { id: NodeId(2b5779bbb23c29bfb258e51a6c66c9bf0d95760f), addr: NodeAddr { ip: 181.124.134.254, port: 13239 } }, Node { id: NodeId(9ace994c70d4165087e60a1763bb25a2a2f70a4d), addr: NodeAddr { ip: 82.7.160.224, port: 6881 } }, Node { id: NodeId(b8815434b1e68b07e2b6cddf5ee18f5a84e1469b), addr: NodeAddr { ip: 87.67.111.62, port: 1067 } }, Node { id: NodeId(a3bd6ecf5c1b227798eca92c35bc1b3e1b57fbd0), addr: NodeAddr { ip: 128.73.170.65, port: 39456 } }, Node { id: NodeId(4caa4d322d478549f43cd9766df9c8430a638ad1), addr: NodeAddr { ip: 77.22.180.50, port: 52245 } }] }
2024-05-23T20:44:20.985781Z DEBUG kademliar::app: node tx_id=TxId(6161) distance=Distance(LCP: 0, value: b4ad004822d0727e26eb98fa63497de817adc0d1) node=Node { id: NodeId(129791afac2ff0ffaf2d96818e30affad62eefd7), addr: NodeAddr { ip: 151.240.36.39, port: 24920 } }
2024-05-23T20:44:20.985800Z DEBUG kademliar::app: node is closer --> query tx_id=TxId(6161)
2024-05-23T20:44:20.985823Z DEBUG kademliar::app: node tx_id=TxId(6161) distance=Distance(LCP: 2, value: 27df018cd50f5b38331dd42b2119b332886fd4a5) node=Node { id: NodeId(81e5906b5bf0d9b9badbda50cc60612049ecfba3), addr: NodeAddr { ip: 177.124.154.42, port: 35585 } }
2024-05-23T20:44:20.985837Z DEBUG kademliar::app: node is closer --> query tx_id=TxId(6161)
2024-05-23T20:44:20.985850Z DEBUG kademliar::app: node tx_id=TxId(6161) distance=Distance(LCP: 1, value: 6ffae0b261055113269ade9eff2305efe8021cbd) node=Node { id: NodeId(c9c07155effad392af5cd0e5125ad7fd298133bb), addr: NodeAddr { ip: 186.216.183.227, port: 22372 } }
2024-05-23T20:44:20.985853Z DEBUG kademliar::krpc: sender: recv msg=Request(TxId(6461))
2024-05-23T20:44:20.985864Z DEBUG kademliar::app: node tx_id=TxId(6161) distance=Distance(LCP: 0, value: cb73507a27b66ae3e872d371714eaa10c6e81e93) node=Node { id: NodeId(6d49c19da949e86261b4dd0a9c377802076b3195), addr: NodeAddr { ip: 177.181.3.99, port: 21148 } }
2024-05-23T20:44:20.985876Z DEBUG kademliar::app: node tx_id=TxId(6161) distance=Distance(LCP: 3, value: 126e468a8134e57683e591d801b7fe3f99479056) node=Node { id: NodeId(b454d76d0fcb67f70a239fa3ecce2c2d58c4bf50), addr: NodeAddr { ip: 176.145.73.20, port: 15976 } }
2024-05-23T20:44:20.985889Z DEBUG kademliar::app: node is closer --> query tx_id=TxId(6161)
2024-05-23T20:44:20.985899Z DEBUG kademliar::app: node tx_id=TxId(6161) distance=Distance(LCP: 4, value: 0ba0938e98e342cb32f9120e69efc7773c6f3574) node=Node { id: NodeId(ad9a0269161cc04abb3f1c7584961565fdec1a72), addr: NodeAddr { ip: 176.190.209.249, port: 3150 } }
2024-05-23T20:44:20.985912Z DEBUG kademliar::app: node is closer --> query tx_id=TxId(6161)
2024-05-23T20:44:20.985921Z DEBUG kademliar::app: node tx_id=TxId(6161) distance=Distance(LCP: 0, value: e876d35e9bb44e98b1df7b703dc7b020e1f53590) node=Node { id: NodeId(4e4c42b9154bcc193819750bd0be623220761a96), addr: NodeAddr { ip: 172.59.161.64, port: 4456 } }
2024-05-23T20:44:20.985933Z DEBUG kademliar::app: node tx_id=TxId(6161) distance=Distance(LCP: 1, value: 74d5d2f010f04a617a5ef3c2f40a00c794f07992) node=Node { id: NodeId(d2ef43179e0fc8e0f398fdb91973d2d555735694), addr: NodeAddr { ip: 181.44.129.66, port: 19122 } }
2024-05-23T20:44:20.985934Z DEBUG kademliar::krpc: sender: send ok dst=NodeAddr { ip: 151.240.36.39, port: 24920 } req=FindNodeRequest { tx_id: TxId(6461), node_id_self: NodeId(a63a91e78eff828189c60e7bed79d212c1832f06), node_id_target: NodeId(a63a91e78eff828189c60e7bed79d212c1832f06) }
2024-05-23T20:44:20.985945Z DEBUG kademliar::app: node tx_id=TxId(6161) distance=Distance(LCP: 0, value: e64d87759a1e4c65479f084199cf31d514c2a57b) node=Node { id: NodeId(4077169214e1cee4ce59063a74b6e3c7d5418a7d), addr: NodeAddr { ip: 41.105.111.148, port: 41798 } }
2024-05-23T20:44:20.985959Z DEBUG kademliar::app: node tx_id=TxId(6161) distance=Distance(LCP: 1, value: 6762eb3efe13ed0f8d8da00de82cbb9603681c6e) node=Node { id: NodeId(c1587ad970ec6f8e044bae7605556984c2eb3368), addr: NodeAddr { ip: 190.164.216.22, port: 6881 } }
2024-05-23T20:44:20.985960Z DEBUG kademliar::krpc: sender: recv msg=Request(TxId(6561))
2024-05-23T20:44:20.985970Z DEBUG kademliar::app: node tx_id=TxId(6161) distance=Distance(LCP: 1, value: 5ca17d0e47b6e0be3db12af3aee9c3c9b71fdbde) node=Node { id: NodeId(fa9bece9c949623fb4772488439011db769cf4d8), addr: NodeAddr { ip: 177.39.86.12, port: 30829 } }
2024-05-23T20:44:20.985982Z DEBUG kademliar::app: node tx_id=TxId(6161) distance=Distance(LCP: 0, value: 8d6de85c3cc3ab3e3b9eeb61811f1badcc165909) node=Node { id: NodeId(2b5779bbb23c29bfb258e51a6c66c9bf0d95760f), addr: NodeAddr { ip: 181.124.134.254, port: 13239 } }
2024-05-23T20:44:20.985994Z DEBUG kademliar::app: node tx_id=TxId(6161) distance=Distance(LCP: 2, value: 3cf408abfe2b94d10e20046c8ec2f7b06374254b) node=Node { id: NodeId(9ace994c70d4165087e60a1763bb25a2a2f70a4d), addr: NodeAddr { ip: 82.7.160.224, port: 6881 } }
2024-05-23T20:44:20.986006Z DEBUG kademliar::app: node tx_id=TxId(6161) distance=Distance(LCP: 3, value: 1ebbc5d33f1909866b70c3a4b3985d484562699d) node=Node { id: NodeId(b8815434b1e68b07e2b6cddf5ee18f5a84e1469b), addr: NodeAddr { ip: 87.67.111.62, port: 1067 } }
2024-05-23T20:44:20.986018Z DEBUG kademliar::app: node tx_id=TxId(6161) distance=Distance(LCP: 5, value: 0587ff28d2e4a0f6112aa757d8c5c92cdad4d4d6) node=Node { id: NodeId(a3bd6ecf5c1b227798eca92c35bc1b3e1b57fbd0), addr: NodeAddr { ip: 128.73.170.65, port: 39456 } }
2024-05-23T20:44:20.986031Z DEBUG kademliar::app: node is closer --> query tx_id=TxId(6161)
2024-05-23T20:44:20.986041Z DEBUG kademliar::app: node tx_id=TxId(6161) distance=Distance(LCP: 0, value: ea90dcd5a3b807c87dfad70d80801a51cbe0a5d7) node=Node { id: NodeId(4caa4d322d478549f43cd9766df9c8430a638ad1), addr: NodeAddr { ip: 77.22.180.50, port: 52245 } }
2024-05-23T20:44:20.986058Z DEBUG kademliar::app: main: recv msg=SendSuccess(TxId(6461))
2024-05-23T20:44:21.149875Z DEBUG kademliar::krpc: receiver: recv src=151.240.36.39:24920 len=278 data_hex="64323a6970363a54f80421d93b313a7264323a696432303a129791afac2ff0ffaf2d96818e30affad62eefd7353a6e6f6465733230383a06f3893be940a94020cd7fb6d08cf9bdcebcf2f55f199484bdf306f22bd6ae529049f1f1bbe9ebb3a6db3c870ce1cf263834e0f506f31c3918a3fea966266fa7b8887d2197428b202fcadb6ec1bc06f3ac3b8c7fe94ebfe79733bc0722415e4432a4b160748cc491093d7a781d719ece8a20f1f2043c3e65dd5e0b245e1cd6edb3a906f3a686b17d4072d1948afc44ce9948305b2809c38e461ab07106f3a232b5393f7d193af9d8e692416c6bd495d6ade1f3c9959206f6c723cfe936584cf991a241e4297e78836a3d594003c2da0f65313a74323a6461313a79313a7265"
2024-05-23T20:44:21.149991Z DEBUG kademliar::app: main: recv msg=Response(FindNodeResponse { tx_id: TxId(6461), nodes: [Node { id: NodeId(06f3893be940a94020cd7fb6d08cf9bdcebcf2f5), addr: NodeAddr { ip: 95.25.148.132, port: 48627 } }, Node { id: NodeId(06f22bd6ae529049f1f1bbe9ebb3a6db3c870ce1), addr: NodeAddr { ip: 207.38.56.52, port: 57589 } }, Node { id: NodeId(06f31c3918a3fea966266fa7b8887d2197428b20), addr: NodeAddr { ip: 47.202.219.110, port: 49596 } }, Node { id: NodeId(06f3ac3b8c7fe94ebfe79733bc0722415e4432a4), addr: NodeAddr { ip: 177.96.116.140, port: 50321 } }, Node { id: NodeId(093d7a781d719ece8a20f1f2043c3e65dd5e0b24), addr: NodeAddr { ip: 94.28.214.237, port: 45993 } }, Node { id: NodeId(06f3a686b17d4072d1948afc44ce9948305b2809), addr: NodeAddr { ip: 195.142.70.26, port: 45169 } }, Node { id: NodeId(06f3a232b5393f7d193af9d8e692416c6bd495d6), addr: NodeAddr { ip: 173.225.243.201, port: 38290 } }, Node { id: NodeId(06f6c723cfe936584cf991a241e4297e78836a3d), addr: NodeAddr { ip: 89.64.3.194, port: 55823 } }] })
2024-05-23T20:44:21.150035Z DEBUG kademliar::app: response tx_id=TxId(6461) res=FindNodeResponse { tx_id: TxId(6461), nodes: [Node { id: NodeId(06f3893be940a94020cd7fb6d08cf9bdcebcf2f5), addr: NodeAddr { ip: 95.25.148.132, port: 48627 } }, Node { id: NodeId(06f22bd6ae529049f1f1bbe9ebb3a6db3c870ce1), addr: NodeAddr { ip: 207.38.56.52, port: 57589 } }, Node { id: NodeId(06f31c3918a3fea966266fa7b8887d2197428b20), addr: NodeAddr { ip: 47.202.219.110, port: 49596 } }, Node { id: NodeId(06f3ac3b8c7fe94ebfe79733bc0722415e4432a4), addr: NodeAddr { ip: 177.96.116.140, port: 50321 } }, Node { id: NodeId(093d7a781d719ece8a20f1f2043c3e65dd5e0b24), addr: NodeAddr { ip: 94.28.214.237, port: 45993 } }, Node { id: NodeId(06f3a686b17d4072d1948afc44ce9948305b2809), addr: NodeAddr { ip: 195.142.70.26, port: 45169 } }, Node { id: NodeId(06f3a232b5393f7d193af9d8e692416c6bd495d6), addr: NodeAddr { ip: 173.225.243.201, port: 38290 } }, Node { id: NodeId(06f6c723cfe936584cf991a241e4297e78836a3d), addr: NodeAddr { ip: 89.64.3.194, port: 55823 } }] }
2024-05-23T20:44:21.150063Z DEBUG kademliar::app: node tx_id=TxId(6461) distance=Distance(LCP: 0, value: a0c918dc67bf2bc1a90b71cd3df52baf0f3fddf3) node=Node { id: NodeId(06f3893be940a94020cd7fb6d08cf9bdcebcf2f5), addr: NodeAddr { ip: 95.25.148.132, port: 48627 } }
2024-05-23T20:44:21.150081Z DEBUG kademliar::app: node tx_id=TxId(6461) distance=Distance(LCP: 0, value: a0c8ba3120ad12c87837b59206ca74c9fd0423e7) node=Node { id: NodeId(06f22bd6ae529049f1f1bbe9ebb3a6db3c870ce1), addr: NodeAddr { ip: 207.38.56.52, port: 57589 } }
2024-05-23T20:44:21.150093Z DEBUG kademliar::app: node tx_id=TxId(6461) distance=Distance(LCP: 0, value: a0c98dde965c7c28efe061dc55f1af3356c1a426) node=Node { id: NodeId(06f31c3918a3fea966266fa7b8887d2197428b20), addr: NodeAddr { ip: 47.202.219.110, port: 49596 } }
2024-05-23T20:44:21.150107Z DEBUG kademliar::app: k-bucket is full --> ignore tx_id=TxId(6461)
2024-05-23T20:44:21.150114Z DEBUG kademliar::app: node tx_id=TxId(6461) distance=Distance(LCP: 0, value: a0c93ddc02806bcf36219948517ef0539fc71da2) node=Node { id: NodeId(06f3ac3b8c7fe94ebfe79733bc0722415e4432a4), addr: NodeAddr { ip: 177.96.116.140, port: 50321 } }
2024-05-23T20:44:21.150125Z DEBUG kademliar::app: k-bucket is full --> ignore tx_id=TxId(6461)
2024-05-23T20:44:21.150131Z DEBUG kademliar::app: node tx_id=TxId(6461) distance=Distance(LCP: 0, value: af07eb9f938e1c4f03e6ff89e945ec771cdd2422) node=Node { id: NodeId(093d7a781d719ece8a20f1f2043c3e65dd5e0b24), addr: NodeAddr { ip: 94.28.214.237, port: 45993 } }
2024-05-23T20:44:21.150141Z DEBUG kademliar::app: k-bucket is full --> ignore tx_id=TxId(6461)
2024-05-23T20:44:21.150148Z DEBUG kademliar::app: node tx_id=TxId(6461) distance=Distance(LCP: 0, value: a0c937613f82c2f358528487a9b74b5af1d8070f) node=Node { id: NodeId(06f3a686b17d4072d1948afc44ce9948305b2809), addr: NodeAddr { ip: 195.142.70.26, port: 45169 } }
2024-05-23T20:44:21.150158Z DEBUG kademliar::app: k-bucket is full --> ignore tx_id=TxId(6461)
2024-05-23T20:44:21.150165Z DEBUG kademliar::app: node tx_id=TxId(6461) distance=Distance(LCP: 0, value: a0c933d53bc6bdfc90fcf7a30beb937eaa57bad0) node=Node { id: NodeId(06f3a232b5393f7d193af9d8e692416c6bd495d6), addr: NodeAddr { ip: 173.225.243.201, port: 38290 } }
2024-05-23T20:44:21.150175Z DEBUG kademliar::app: k-bucket is full --> ignore tx_id=TxId(6461)
2024-05-23T20:44:21.150181Z DEBUG kademliar::app: node tx_id=TxId(6461) distance=Distance(LCP: 0, value: a0cc56c44116b4d9c53f9fd9ac9dfb6cb900453b) node=Node { id: NodeId(06f6c723cfe936584cf991a241e4297e78836a3d), addr: NodeAddr { ip: 89.64.3.194, port: 55823 } }
2024-05-23T20:44:21.150192Z DEBUG kademliar::app: k-bucket is full --> ignore tx_id=TxId(6461)
2024-05-23T20:44:21.187206Z DEBUG kademliar::krpc: sender: send ok dst=NodeAddr { ip: 177.124.154.42, port: 35585 } req=FindNodeRequest { tx_id: TxId(6561), node_id_self: NodeId(a63a91e78eff828189c60e7bed79d212c1832f06), node_id_target: NodeId(a63a91e78eff828189c60e7bed79d212c1832f06) }
2024-05-23T20:44:21.187261Z DEBUG kademliar::krpc: sender: recv msg=Request(TxId(6661))
2024-05-23T20:44:21.187274Z DEBUG kademliar::app: main: recv msg=SendSuccess(TxId(6561))
2024-05-23T20:44:25.809833Z DEBUG kademliar::app: main: recv msg=ResponseTimeout(TxId(6361))
2024-05-23T20:44:25.809890Z ERROR kademliar::app: timeout --> remove tx_id=TxId(6361)
2024-05-23T20:44:25.809903Z DEBUG kademliar::app: main: recv msg=ResponseTimeout(TxId(6161))
2024-05-23T20:44:25.809914Z DEBUG kademliar::app: main: recv msg=ResponseTimeout(TxId(6261))
2024-05-23T20:44:25.809922Z ERROR kademliar::app: timeout --> remove tx_id=TxId(6261)
2024-05-23T20:44:25.842053Z DEBUG kademliar::krpc: sender: send ok dst=NodeAddr { ip: 176.145.73.20, port: 15976 } req=FindNodeRequest { tx_id: TxId(6661), node_id_self: NodeId(a63a91e78eff828189c60e7bed79d212c1832f06), node_id_target: NodeId(a63a91e78eff828189c60e7bed79d212c1832f06) }
2024-05-23T20:44:25.842119Z DEBUG kademliar::krpc: sender: recv msg=Request(TxId(6761))
2024-05-23T20:44:25.842135Z DEBUG kademliar::app: main: recv msg=SendSuccess(TxId(6661))
2024-05-23T20:44:25.842161Z DEBUG kademliar::krpc: sender: send ok dst=NodeAddr { ip: 176.190.209.249, port: 3150 } req=FindNodeRequest { tx_id: TxId(6761), node_id_self: NodeId(a63a91e78eff828189c60e7bed79d212c1832f06), node_id_target: NodeId(a63a91e78eff828189c60e7bed79d212c1832f06) }
2024-05-23T20:44:25.842177Z DEBUG kademliar::app: main: recv msg=SendSuccess(TxId(6761))
2024-05-23T20:44:25.842179Z DEBUG kademliar::krpc: sender: recv msg=Request(TxId(6861))
2024-05-23T20:44:25.987650Z DEBUG kademliar::app: main: recv msg=ResponseTimeout(TxId(6461))
2024-05-23T20:44:26.188360Z DEBUG kademliar::app: main: recv msg=ResponseTimeout(TxId(6561))
2024-05-23T20:44:26.188410Z ERROR kademliar::app: timeout --> remove tx_id=TxId(6561)
2024-05-23T20:44:26.246581Z DEBUG kademliar::krpc: sender: send ok dst=NodeAddr { ip: 128.73.170.65, port: 39456 } req=FindNodeRequest { tx_id: TxId(6861), node_id_self: NodeId(a63a91e78eff828189c60e7bed79d212c1832f06), node_id_target: NodeId(a63a91e78eff828189c60e7bed79d212c1832f06) }
2024-05-23T20:44:26.246657Z DEBUG kademliar::app: main: recv msg=SendSuccess(TxId(6861))
2024-05-23T20:44:26.337400Z DEBUG kademliar::krpc: receiver: recv src=128.73.170.65:39456 len=297 data_hex="64323a6970363a54f80421d93b313a7264323a696432303aa3bd6ecf5c1b227798eca92c35bc1b3e1b57fbd0353a6e6f6465733230383aa4598d10d22804c4b8b6e1df94d895b67a25ec54598e11645135a4088d9787f3ba49594adc10e8c9bbf7bf03ee8b8ec4dfe81ae1a6ba06650959bb0610b9536a51c4775bf06a46a62e1638862490a501dedb1a053996b57738912f916e5590dcd05e2da1edd0150da7bddaf7ceef09f5765501d8ae49950a8a9b284a72227cf73a47a72f931aa0053b9efb418e3982d10a6bd1e5fbe35d7c50143d6ba678e2c349d39235dda2c7ea02a5a7cf05c8fd11b2d5c21830bea4ba8c75198bd1deee026aa11b307029850c10e50512635237de313a706935353631316565313a74323a6861313a76343a4c54012e313a79313a7265"
2024-05-23T20:44:26.337525Z DEBUG kademliar::app: main: recv msg=Response(FindNodeResponse { tx_id: TxId(6861), nodes: [Node { id: NodeId(a4598d10d22804c4b8b6e1df94d895b67a25ec54), addr: NodeAddr { ip: 89.142.17.100, port: 20789 } }, Node { id: NodeId(a4088d9787f3ba49594adc10e8c9bbf7bf03ee8b), addr: NodeAddr { ip: 142.196.223.232, port: 6881 } }, Node { id: NodeId(a6ba06650959bb0610b9536a51c4775bf06a46a6), addr: NodeAddr { ip: 46.22.56.134, port: 9360 } }, Node { id: NodeId(a501dedb1a053996b57738912f916e5590dcd05e), addr: NodeAddr { ip: 45.161.237.208, port: 5389 } }, Node { id: NodeId(a7bddaf7ceef09f5765501d8ae49950a8a9b284a), addr: NodeAddr { ip: 114.34.124.247, port: 14919 } }, Node { id: NodeId(a72f931aa0053b9efb418e3982d10a6bd1e5fbe3), addr: NodeAddr { ip: 93.124.80.20, port: 15723 } }, Node { id: NodeId(a678e2c349d39235dda2c7ea02a5a7cf05c8fd11), addr: NodeAddr { ip: 178.213.194.24, port: 12478 } }, Node { id: NodeId(a4ba8c75198bd1deee026aa11b307029850c10e5), addr: NodeAddr { ip: 5.18.99.82, port: 14302 } }] })
2024-05-23T20:44:26.337570Z DEBUG kademliar::app: response tx_id=TxId(6861) res=FindNodeResponse { tx_id: TxId(6861), nodes: [Node { id: NodeId(a4598d10d22804c4b8b6e1df94d895b67a25ec54), addr: NodeAddr { ip: 89.142.17.100, port: 20789 } }, Node { id: NodeId(a4088d9787f3ba49594adc10e8c9bbf7bf03ee8b), addr: NodeAddr { ip: 142.196.223.232, port: 6881 } }, Node { id: NodeId(a6ba06650959bb0610b9536a51c4775bf06a46a6), addr: NodeAddr { ip: 46.22.56.134, port: 9360 } }, Node { id: NodeId(a501dedb1a053996b57738912f916e5590dcd05e), addr: NodeAddr { ip: 45.161.237.208, port: 5389 } }, Node { id: NodeId(a7bddaf7ceef09f5765501d8ae49950a8a9b284a), addr: NodeAddr { ip: 114.34.124.247, port: 14919 } }, Node { id: NodeId(a72f931aa0053b9efb418e3982d10a6bd1e5fbe3), addr: NodeAddr { ip: 93.124.80.20, port: 15723 } }, Node { id: NodeId(a678e2c349d39235dda2c7ea02a5a7cf05c8fd11), addr: NodeAddr { ip: 178.213.194.24, port: 12478 } }, Node { id: NodeId(a4ba8c75198bd1deee026aa11b307029850c10e5), addr: NodeAddr { ip: 5.18.99.82, port: 14302 } }] }
2024-05-23T20:44:26.337596Z DEBUG kademliar::app: node tx_id=TxId(6861) distance=Distance(LCP: 6, value: 02631cf75cd786453170efa479a147a4bba6c352) node=Node { id: NodeId(a4598d10d22804c4b8b6e1df94d895b67a25ec54), addr: NodeAddr { ip: 89.142.17.100, port: 20789 } }
2024-05-23T20:44:26.337611Z DEBUG kademliar::app: node is closer --> query tx_id=TxId(6861)
2024-05-23T20:44:26.337635Z DEBUG kademliar::app: node tx_id=TxId(6861) distance=Distance(LCP: 6, value: 02321c70090c38c8d08cd26b05b069e57e80c18d) node=Node { id: NodeId(a4088d9787f3ba49594adc10e8c9bbf7bf03ee8b), addr: NodeAddr { ip: 142.196.223.232, port: 6881 } }
2024-05-23T20:44:26.337646Z DEBUG kademliar::app: node is closer --> query tx_id=TxId(6861)
2024-05-23T20:44:26.337656Z DEBUG kademliar::app: node tx_id=TxId(6861) distance=Distance(LCP: 8, value: 0080978287a63987997f5d11bcbda54931e969a0) node=Node { id: NodeId(a6ba06650959bb0610b9536a51c4775bf06a46a6), addr: NodeAddr { ip: 46.22.56.134, port: 9360 } }
2024-05-23T20:44:26.337656Z DEBUG kademliar::krpc: sender: recv msg=Request(TxId(6961))
2024-05-23T20:44:26.337672Z DEBUG kademliar::app: node is closer --> query tx_id=TxId(6861)
2024-05-23T20:44:26.337682Z DEBUG kademliar::app: node tx_id=TxId(6861) distance=Distance(LCP: 6, value: 033b4f3c94fabb173cb136eac2e8bc47515fff58) node=Node { id: NodeId(a501dedb1a053996b57738912f916e5590dcd05e), addr: NodeAddr { ip: 45.161.237.208, port: 5389 } }
2024-05-23T20:44:26.337693Z DEBUG kademliar::app: node tx_id=TxId(6861) distance=Distance(LCP: 7, value: 01874b1040108b74ff930fa3433047184b18074c) node=Node { id: NodeId(a7bddaf7ceef09f5765501d8ae49950a8a9b284a), addr: NodeAddr { ip: 114.34.124.247, port: 14919 } }
2024-05-23T20:44:26.337704Z DEBUG kademliar::app: node tx_id=TxId(6861) distance=Distance(LCP: 7, value: 011502fd2efab91f728780426fa8d8791066d4e5) node=Node { id: NodeId(a72f931aa0053b9efb418e3982d10a6bd1e5fbe3), addr: NodeAddr { ip: 93.124.80.20, port: 15723 } }
2024-05-23T20:44:26.337715Z DEBUG kademliar::app: node tx_id=TxId(6861) distance=Distance(LCP: 9, value: 00427324c72c10b45464c991efdc75ddc44bd217) node=Node { id: NodeId(a678e2c349d39235dda2c7ea02a5a7cf05c8fd11), addr: NodeAddr { ip: 178.213.194.24, port: 12478 } }
2024-05-23T20:44:26.337725Z DEBUG kademliar::app: node is closer --> query tx_id=TxId(6861)
2024-05-23T20:44:26.337734Z DEBUG kademliar::app: node tx_id=TxId(6861) distance=Distance(LCP: 6, value: 02801d929774535f67c464daf649a23b448f3fe3) node=Node { id: NodeId(a4ba8c75198bd1deee026aa11b307029850c10e5), addr: NodeAddr { ip: 5.18.99.82, port: 14302 } }
2024-05-23T20:44:26.337726Z DEBUG kademliar::krpc: sender: send ok dst=NodeAddr { ip: 89.142.17.100, port: 20789 } req=FindNodeRequest { tx_id: TxId(6961), node_id_self: NodeId(a63a91e78eff828189c60e7bed79d212c1832f06), node_id_target: NodeId(a63a91e78eff828189c60e7bed79d212c1832f06) }
2024-05-23T20:44:26.337750Z DEBUG kademliar::app: main: recv msg=SendSuccess(TxId(6961))
2024-05-23T20:44:26.337751Z DEBUG kademliar::krpc: sender: recv msg=Request(TxId(6a61))
2024-05-23T20:44:26.399038Z DEBUG kademliar::krpc: receiver: recv src=89.142.17.100:20789 len=287 data_hex="64323a6970363a54f80421d93b313a7264323a696432303aa4598d10d22804c4b8b6e1df94d895b67a25ec54353a6e6f6465733230383aa60bac34097afa35108dbc9088a33ab39aac7105598604598f12a652f592387dbc9eb8f38617199c3401ad846c2f4675883586fda6a7d7bee6f36a9affe6201402c8f46ac7688ac2c90fb97ac2f5a6c98ecff95b40cc2778f7cafda349acbfdb15dd2d98d254c1eba71f7bd6ae529049f1f1bbe9ebb3a6db3c870ce1c98d67e0c7b4a76750d6ae529049f1f1bbe9ebb3a6db3c870ce1d5169587a3eba7b7e6170025d15813a79582d0d9c24b68b9aaeebce6fc10c8d5a7ed5ad60dc2d5e16b956cb1c5a1287a4ce3cc1ea23751dbc35065313a74323a6961313a76343a5554b7ec313a79313a7265"
2024-05-23T20:44:26.399282Z DEBUG kademliar::app: main: recv msg=Response(FindNodeResponse { tx_id: TxId(6961), nodes: [Node { id: NodeId(a60bac34097afa35108dbc9088a33ab39aac7105), addr: NodeAddr { ip: 89.134.4.89, port: 36626 } }, Node { id: NodeId(a652f592387dbc9eb8f38617199c3401ad846c2f), addr: NodeAddr { ip: 70.117.136.53, port: 34557 } }, Node { id: NodeId(a6a7d7bee6f36a9affe6201402c8f46ac7688ac2), addr: NodeAddr { ip: 201.15.185.122, port: 49909 } }, Node { id: NodeId(a6c98ecff95b40cc2778f7cafda349acbfdb15dd), addr: NodeAddr { ip: 45.152.210.84, port: 49643 } }, Node { id: NodeId(a71f7bd6ae529049f1f1bbe9ebb3a6db3c870ce1), addr: NodeAddr { ip: 201.141.103.224, port: 51124 } }, Node { id: NodeId(a76750d6ae529049f1f1bbe9ebb3a6db3c870ce1), addr: NodeAddr { ip: 213.22.149.135, port: 41963 } }, Node { id: NodeId(a7b7e6170025d15813a79582d0d9c24b68b9aaee), addr: NodeAddr { ip: 188.230.252.16, port: 51413 } }, Node { id: NodeId(a7ed5ad60dc2d5e16b956cb1c5a1287a4ce3cc1e), addr: NodeAddr { ip: 162.55.81.219, port: 50000 } }] })
2024-05-23T20:44:26.399356Z DEBUG kademliar::app: response tx_id=TxId(6961) res=FindNodeResponse { tx_id: TxId(6961), nodes: [Node { id: NodeId(a60bac34097afa35108dbc9088a33ab39aac7105), addr: NodeAddr { ip: 89.134.4.89, port: 36626 } }, Node { id: NodeId(a652f592387dbc9eb8f38617199c3401ad846c2f), addr: NodeAddr { ip: 70.117.136.53, port: 34557 } }, Node { id: NodeId(a6a7d7bee6f36a9affe6201402c8f46ac7688ac2), addr: NodeAddr { ip: 201.15.185.122, port: 49909 } }, Node { id: NodeId(a6c98ecff95b40cc2778f7cafda349acbfdb15dd), addr: NodeAddr { ip: 45.152.210.84, port: 49643 } }, Node { id: NodeId(a71f7bd6ae529049f1f1bbe9ebb3a6db3c870ce1), addr: NodeAddr { ip: 201.141.103.224, port: 51124 } }, Node { id: NodeId(a76750d6ae529049f1f1bbe9ebb3a6db3c870ce1), addr: NodeAddr { ip: 213.22.149.135, port: 41963 } }, Node { id: NodeId(a7b7e6170025d15813a79582d0d9c24b68b9aaee), addr: NodeAddr { ip: 188.230.252.16, port: 51413 } }, Node { id: NodeId(a7ed5ad60dc2d5e16b956cb1c5a1287a4ce3cc1e), addr: NodeAddr { ip: 162.55.81.219, port: 50000 } }] }
2024-05-23T20:44:26.399394Z DEBUG kademliar::app: node tx_id=TxId(6961) distance=Distance(LCP: 10, value: 00313dd3878578b4994bb2eb65dae8a15b2f5e03) node=Node { id: NodeId(a60bac34097afa35108dbc9088a33ab39aac7105), addr: NodeAddr { ip: 89.134.4.89, port: 36626 } }
2024-05-23T20:44:26.399419Z DEBUG kademliar::app: node is closer --> query tx_id=TxId(6961)
2024-05-23T20:44:26.399444Z DEBUG kademliar::app: node tx_id=TxId(6961) distance=Distance(LCP: 9, value: 00686475b6823e1f3135886cf4e5e6136c074329) node=Node { id: NodeId(a652f592387dbc9eb8f38617199c3401ad846c2f), addr: NodeAddr { ip: 70.117.136.53, port: 34557 } }
2024-05-23T20:44:26.399461Z DEBUG kademliar::app: node tx_id=TxId(6961) distance=Distance(LCP: 8, value: 009d4659680ce81b76202e6fefb1267806eba5c4) node=Node { id: NodeId(a6a7d7bee6f36a9affe6201402c8f46ac7688ac2), addr: NodeAddr { ip: 201.15.185.122, port: 49909 } }
2024-05-23T20:44:26.399476Z DEBUG kademliar::app: node tx_id=TxId(6961) distance=Distance(LCP: 8, value: 00f31f2877a4c24daebef9b110da9bbe7e583adb) node=Node { id: NodeId(a6c98ecff95b40cc2778f7cafda349acbfdb15dd), addr: NodeAddr { ip: 45.152.210.84, port: 49643 } }
2024-05-23T20:44:26.399490Z DEBUG kademliar::app: node tx_id=TxId(6961) distance=Distance(LCP: 7, value: 0125ea3120ad12c87837b59206ca74c9fd0423e7) node=Node { id: NodeId(a71f7bd6ae529049f1f1bbe9ebb3a6db3c870ce1), addr: NodeAddr { ip: 201.141.103.224, port: 51124 } }
2024-05-23T20:44:26.399504Z DEBUG kademliar::app: node tx_id=TxId(6961) distance=Distance(LCP: 7, value: 015dc13120ad12c87837b59206ca74c9fd0423e7) node=Node { id: NodeId(a76750d6ae529049f1f1bbe9ebb3a6db3c870ce1), addr: NodeAddr { ip: 213.22.149.135, port: 41963 } }
2024-05-23T20:44:26.399517Z DEBUG kademliar::app: node tx_id=TxId(6961) distance=Distance(LCP: 7, value: 018d77f08eda53d99a619bf93da01059a93a85e8) node=Node { id: NodeId(a7b7e6170025d15813a79582d0d9c24b68b9aaee), addr: NodeAddr { ip: 188.230.252.16, port: 51413 } }
2024-05-23T20:44:26.399533Z DEBUG kademliar::app: node tx_id=TxId(6961) distance=Distance(LCP: 7, value: 01d7cb31833d5760e25362ca28d8fa688d60e318) node=Node { id: NodeId(a7ed5ad60dc2d5e16b956cb1c5a1287a4ce3cc1e), addr: NodeAddr { ip: 162.55.81.219, port: 50000 } }
2024-05-23T20:44:26.438532Z DEBUG kademliar::krpc: sender: send ok dst=NodeAddr { ip: 142.196.223.232, port: 6881 } req=FindNodeRequest { tx_id: TxId(6a61), node_id_self: NodeId(a63a91e78eff828189c60e7bed79d212c1832f06), node_id_target: NodeId(a63a91e78eff828189c60e7bed79d212c1832f06) }
2024-05-23T20:44:26.438613Z DEBUG kademliar::krpc: sender: recv msg=Request(TxId(6b61))
2024-05-23T20:44:26.438631Z DEBUG kademliar::app: main: recv msg=SendSuccess(TxId(6a61))
2024-05-23T20:44:26.588427Z DEBUG kademliar::krpc: receiver: recv src=142.196.223.232:6881 len=297 data_hex="64323a6970363a54f80421d93b313a7264323a696432303aa4088d9787f3ba49594adc10e8c9bbf7bf03ee8b353a6e6f6465733230383aa7dfb52ae4b77b86c28b72f232719f4ec95723d93b17a2a77fbda661bd5c78695608ccc3a49f89d46d2cdbf51133dda08673de5da667d5191430cbb351b02bf0a01522d241a54d2fad2fd13c1ae9a7c25608c4864979c6fb3159a6316a711eccae16bca5f68ce064a6fbc457db9db6c4f84494f3fef4822af93281a65fa8a81d75f0a6f15ccc6a20833731f3c76bcfa64d4542892e96d420d4b72c05a70e9d95f27e9ee549e3729dc7bb33bf71ca24d2d380297d6a04a7c3ca05f9d7ff6fd2441363a7efce3e31433f134eb68e79958a313a706935353631316565313a74323a6a61313a76343a4c540209313a79313a7265"
2024-05-23T20:44:26.588546Z DEBUG kademliar::app: main: recv msg=Response(FindNodeResponse { tx_id: TxId(6a61), nodes: [Node { id: NodeId(a7dfb52ae4b77b86c28b72f232719f4ec95723d9), addr: NodeAddr { ip: 59.23.162.167, port: 32701 } }, Node { id: NodeId(a661bd5c78695608ccc3a49f89d46d2cdbf51133), addr: NodeAddr { ip: 221.160.134.115, port: 56925 } }, Node { id: NodeId(a667d5191430cbb351b02bf0a01522d241a54d2f), addr: NodeAddr { ip: 173.47.209.60, port: 6889 } }, Node { id: NodeId(a7c25608c4864979c6fb3159a6316a711eccae16), addr: NodeAddr { ip: 188.165.246.140, port: 57444 } }, Node { id: NodeId(a6fbc457db9db6c4f84494f3fef4822af93281a6), addr: NodeAddr { ip: 95.168.168.29, port: 30192 } }, Node { id: NodeId(a6f15ccc6a20833731f3c76bcfa64d4542892e96), addr: NodeAddr { ip: 212.32.212.183, port: 11269 } }, Node { id: NodeId(a70e9d95f27e9ee549e3729dc7bb33bf71ca24d2), addr: NodeAddr { ip: 211.128.41.125, port: 27140 } }, Node { id: NodeId(a7c3ca05f9d7ff6fd2441363a7efce3e31433f13), addr: NodeAddr { ip: 78.182.142.121, port: 38282 } }] })
2024-05-23T20:44:26.588590Z DEBUG kademliar::app: response tx_id=TxId(6a61) res=FindNodeResponse { tx_id: TxId(6a61), nodes: [Node { id: NodeId(a7dfb52ae4b77b86c28b72f232719f4ec95723d9), addr: NodeAddr { ip: 59.23.162.167, port: 32701 } }, Node { id: NodeId(a661bd5c78695608ccc3a49f89d46d2cdbf51133), addr: NodeAddr { ip: 221.160.134.115, port: 56925 } }, Node { id: NodeId(a667d5191430cbb351b02bf0a01522d241a54d2f), addr: NodeAddr { ip: 173.47.209.60, port: 6889 } }, Node { id: NodeId(a7c25608c4864979c6fb3159a6316a711eccae16), addr: NodeAddr { ip: 188.165.246.140, port: 57444 } }, Node { id: NodeId(a6fbc457db9db6c4f84494f3fef4822af93281a6), addr: NodeAddr { ip: 95.168.168.29, port: 30192 } }, Node { id: NodeId(a6f15ccc6a20833731f3c76bcfa64d4542892e96), addr: NodeAddr { ip: 212.32.212.183, port: 11269 } }, Node { id: NodeId(a70e9d95f27e9ee549e3729dc7bb33bf71ca24d2), addr: NodeAddr { ip: 211.128.41.125, port: 27140 } }, Node { id: NodeId(a7c3ca05f9d7ff6fd2441363a7efce3e31433f13), addr: NodeAddr { ip: 78.182.142.121, port: 38282 } }] }
2024-05-23T20:44:26.588617Z DEBUG kademliar::app: node tx_id=TxId(6a61) distance=Distance(LCP: 7, value: 01e524cd6a48f9074b4d7c89df084d5c08d40cdf) node=Node { id: NodeId(a7dfb52ae4b77b86c28b72f232719f4ec95723d9), addr: NodeAddr { ip: 59.23.162.167, port: 32701 } }
2024-05-23T20:44:26.588633Z DEBUG kademliar::app: node tx_id=TxId(6a61) distance=Distance(LCP: 9, value: 005b2cbbf696d4894505aae464adbf3e1a763e35) node=Node { id: NodeId(a661bd5c78695608ccc3a49f89d46d2cdbf51133), addr: NodeAddr { ip: 221.160.134.115, port: 56925 } }
2024-05-23T20:44:26.588645Z DEBUG kademliar::app: node tx_id=TxId(6a61) distance=Distance(LCP: 9, value: 005d44fe9acf4932d876258b4d6cf0c080266229) node=Node { id: NodeId(a667d5191430cbb351b02bf0a01522d241a54d2f), addr: NodeAddr { ip: 173.47.209.60, port: 6889 } }
2024-05-23T20:44:26.588657Z DEBUG kademliar::app: node tx_id=TxId(6a61) distance=Distance(LCP: 7, value: 01f8c7ef4a79cbf84f3d3f224b48b863df4f8110) node=Node { id: NodeId(a7c25608c4864979c6fb3159a6316a711eccae16), addr: NodeAddr { ip: 188.165.246.140, port: 57444 } }
2024-05-23T20:44:26.588668Z DEBUG kademliar::app: node tx_id=TxId(6a61) distance=Distance(LCP: 8, value: 00c155b05562344571829a88138d503838b1aea0) node=Node { id: NodeId(a6fbc457db9db6c4f84494f3fef4822af93281a6), addr: NodeAddr { ip: 95.168.168.29, port: 30192 } }
2024-05-23T20:44:26.588680Z DEBUG kademliar::app: node tx_id=TxId(6a61) distance=Distance(LCP: 8, value: 00cbcd2be4df01b6b835c91022df9f57830a0190) node=Node { id: NodeId(a6f15ccc6a20833731f3c76bcfa64d4542892e96), addr: NodeAddr { ip: 212.32.212.183, port: 11269 } }
2024-05-23T20:44:26.588698Z DEBUG kademliar::app: node tx_id=TxId(6a61) distance=Distance(LCP: 7, value: 01340c727c811c64c0257ce62ac2e1adb0490bd4) node=Node { id: NodeId(a70e9d95f27e9ee549e3729dc7bb33bf71ca24d2), addr: NodeAddr { ip: 211.128.41.125, port: 27140 } }
2024-05-23T20:44:26.588709Z DEBUG kademliar::app: k-bucket is full --> ignore tx_id=TxId(6a61)
2024-05-23T20:44:26.588716Z DEBUG kademliar::app: node tx_id=TxId(6a61) distance=Distance(LCP: 7, value: 01f95be277287dee5b821d184a961c2cf0c01015) node=Node { id: NodeId(a7c3ca05f9d7ff6fd2441363a7efce3e31433f13), addr: NodeAddr { ip: 78.182.142.121, port: 38282 } }
2024-05-23T20:44:26.588726Z DEBUG kademliar::app: k-bucket is full --> ignore tx_id=TxId(6a61)
2024-05-23T20:44:26.642388Z DEBUG kademliar::krpc: sender: send ok dst=NodeAddr { ip: 46.22.56.134, port: 9360 } req=FindNodeRequest { tx_id: TxId(6b61), node_id_self: NodeId(a63a91e78eff828189c60e7bed79d212c1832f06), node_id_target: NodeId(a63a91e78eff828189c60e7bed79d212c1832f06) }
2024-05-23T20:44:26.642456Z DEBUG kademliar::krpc: sender: recv msg=Request(TxId(6c61))
2024-05-23T20:44:26.642470Z DEBUG kademliar::app: main: recv msg=SendSuccess(TxId(6b61))
2024-05-23T20:44:30.844108Z DEBUG kademliar::app: main: recv msg=ResponseTimeout(TxId(6761))
2024-05-23T20:44:30.844204Z ERROR kademliar::app: timeout --> remove tx_id=TxId(6761)
2024-05-23T20:44:30.844239Z DEBUG kademliar::app: main: recv msg=ResponseTimeout(TxId(6661))
2024-05-23T20:44:30.844267Z ERROR kademliar::app: timeout --> remove tx_id=TxId(6661)
2024-05-23T20:44:30.890578Z DEBUG kademliar::krpc: sender: send ok dst=NodeAddr { ip: 178.213.194.24, port: 12478 } req=FindNodeRequest { tx_id: TxId(6c61), node_id_self: NodeId(a63a91e78eff828189c60e7bed79d212c1832f06), node_id_target: NodeId(a63a91e78eff828189c60e7bed79d212c1832f06) }
2024-05-23T20:44:30.890692Z DEBUG kademliar::krpc: sender: recv msg=Request(TxId(6d61))
2024-05-23T20:44:30.890710Z DEBUG kademliar::app: main: recv msg=SendSuccess(TxId(6c61))
2024-05-23T20:44:30.890754Z DEBUG kademliar::krpc: sender: send ok dst=NodeAddr { ip: 89.134.4.89, port: 36626 } req=FindNodeRequest { tx_id: TxId(6d61), node_id_self: NodeId(a63a91e78eff828189c60e7bed79d212c1832f06), node_id_target: NodeId(a63a91e78eff828189c60e7bed79d212c1832f06) }
2024-05-23T20:44:30.890792Z DEBUG kademliar::app: main: recv msg=SendSuccess(TxId(6d61))
2024-05-23T20:44:30.923041Z DEBUG kademliar::krpc: receiver: recv src=178.213.194.24:12478 len=287 data_hex="64323a6970363a54f80421d93b313a7264323a696432303aa678e2c349d39235dda2c7ea02a5a7cf05c8fd11353a6e6f6465733230383aa63c3223a4d5d7021786e0ce429b27b1b530df7605e452fd209da633f1f8cd7c19b53998e62d763266042ba695187afe03e8599fa62b37c84a8af54c4ab9a4edfd1c930838210bd30510315c1ae2a62274660472ffdc1e7bedfbcd39dc96d108566d26afc835f99aa61e95b3a6db3c870c3e99245e0d1c06b747dee95f1851eec48aa6121718c4d0a23e1637866cc4892258275447595cf0cb727d46a60e07f87f973b70c4883f8c8b5dacfde6b0fbf4de798b1fc8d5a60688f2c5f8c89497348bf0511dfa5d711594655e8daa13bf6965313a74323a6c61313a76343a5554b7ec313a79313a7265"
2024-05-23T20:44:30.923221Z DEBUG kademliar::app: main: recv msg=Response(FindNodeResponse { tx_id: TxId(6c61), nodes: [Node { id: NodeId(a63c3223a4d5d7021786e0ce429b27b1b530df76), addr: NodeAddr { ip: 5.228.82.253, port: 8349 } }, Node { id: NodeId(a633f1f8cd7c19b53998e62d763266042ba69518), addr: NodeAddr { ip: 122.254.3.232, port: 22943 } }, Node { id: NodeId(a62b37c84a8af54c4ab9a4edfd1c930838210bd3), addr: NodeAddr { ip: 5.16.49.92, port: 6882 } }, Node { id: NodeId(a62274660472ffdc1e7bedfbcd39dc96d108566d), addr: NodeAddr { ip: 38.175.200.53, port: 63898 } }, Node { id: NodeId(a61e95b3a6db3c870c3e99245e0d1c06b747dee9), addr: NodeAddr { ip: 95.24.81.238, port: 50314 } }, Node { id: NodeId(a6121718c4d0a23e1637866cc489225827544759), addr: NodeAddr { ip: 92.240.203.114, port: 32070 } }, Node { id: NodeId(a60e07f87f973b70c4883f8c8b5dacfde6b0fbf4), addr: NodeAddr { ip: 222.121.139.31, port: 51413 } }, Node { id: NodeId(a60688f2c5f8c89497348bf0511dfa5d71159465), addr: NodeAddr { ip: 94.141.170.19, port: 49001 } }] })
2024-05-23T20:44:30.923297Z DEBUG kademliar::app: response tx_id=TxId(6c61) res=FindNodeResponse { tx_id: TxId(6c61), nodes: [Node { id: NodeId(a63c3223a4d5d7021786e0ce429b27b1b530df76), addr: NodeAddr { ip: 5.228.82.253, port: 8349 } }, Node { id: NodeId(a633f1f8cd7c19b53998e62d763266042ba69518), addr: NodeAddr { ip: 122.254.3.232, port: 22943 } }, Node { id: NodeId(a62b37c84a8af54c4ab9a4edfd1c930838210bd3), addr: NodeAddr { ip: 5.16.49.92, port: 6882 } }, Node { id: NodeId(a62274660472ffdc1e7bedfbcd39dc96d108566d), addr: NodeAddr { ip: 38.175.200.53, port: 63898 } }, Node { id: NodeId(a61e95b3a6db3c870c3e99245e0d1c06b747dee9), addr: NodeAddr { ip: 95.24.81.238, port: 50314 } }, Node { id: NodeId(a6121718c4d0a23e1637866cc489225827544759), addr: NodeAddr { ip: 92.240.203.114, port: 32070 } }, Node { id: NodeId(a60e07f87f973b70c4883f8c8b5dacfde6b0fbf4), addr: NodeAddr { ip: 222.121.139.31, port: 51413 } }, Node { id: NodeId(a60688f2c5f8c89497348bf0511dfa5d71159465), addr: NodeAddr { ip: 94.141.170.19, port: 49001 } }] }
2024-05-23T20:44:30.923348Z DEBUG kademliar::app: node tx_id=TxId(6c61) distance=Distance(LCP: 13, value: 0006a3c42a2a55839e40eeb5afe2f5a374b3f070) node=Node { id: NodeId(a63c3223a4d5d7021786e0ce429b27b1b530df76), addr: NodeAddr { ip: 5.228.82.253, port: 8349 } }
2024-05-23T20:44:30.923398Z DEBUG kademliar::app: node is closer --> query tx_id=TxId(6c61)
2024-05-23T20:44:30.923451Z DEBUG kademliar::app: node tx_id=TxId(6c61) distance=Distance(LCP: 12, value: 0009601f43839b34b05ee8569b4bb416ea25ba1e) node=Node { id: NodeId(a633f1f8cd7c19b53998e62d763266042ba69518), addr: NodeAddr { ip: 122.254.3.232, port: 22943 } }
2024-05-23T20:44:30.923487Z DEBUG kademliar::krpc: sender: recv msg=Request(TxId(6e61))
2024-05-23T20:44:30.923496Z DEBUG kademliar::app: node tx_id=TxId(6c61) distance=Distance(LCP: 11, value: 0011a62fc47577cdc37faa961065411af9a224d5) node=Node { id: NodeId(a62b37c84a8af54c4ab9a4edfd1c930838210bd3), addr: NodeAddr { ip: 5.16.49.92, port: 6882 } }
2024-05-23T20:44:30.923563Z DEBUG kademliar::app: node tx_id=TxId(6c61) distance=Distance(LCP: 11, value: 0018e5818a8d7d5d97bde38020400e84108b796b) node=Node { id: NodeId(a62274660472ffdc1e7bedfbcd39dc96d108566d), addr: NodeAddr { ip: 38.175.200.53, port: 63898 } }
2024-05-23T20:44:30.923598Z DEBUG kademliar::app: node tx_id=TxId(6c61) distance=Distance(LCP: 10, value: 002404542824be0685f8975fb374ce1476c4f1ef) node=Node { id: NodeId(a61e95b3a6db3c870c3e99245e0d1c06b747dee9), addr: NodeAddr { ip: 95.24.81.238, port: 50314 } }
2024-05-23T20:44:30.923628Z DEBUG kademliar::app: node tx_id=TxId(6c61) distance=Distance(LCP: 10, value: 002886ff4a2f20bf9ff1881729f0f04ae6d7685f) node=Node { id: NodeId(a6121718c4d0a23e1637866cc489225827544759), addr: NodeAddr { ip: 92.240.203.114, port: 32070 } }
2024-05-23T20:44:30.923621Z DEBUG kademliar::krpc: sender: send ok dst=NodeAddr { ip: 5.228.82.253, port: 8349 } req=FindNodeRequest { tx_id: TxId(6e61), node_id_self: NodeId(a63a91e78eff828189c60e7bed79d212c1832f06), node_id_target: NodeId(a63a91e78eff828189c60e7bed79d212c1832f06) }
2024-05-23T20:44:30.923660Z DEBUG kademliar::app: node tx_id=TxId(6c61) distance=Distance(LCP: 10, value: 0034961ff168b9f14d4e31f766247eef2733d4f2) node=Node { id: NodeId(a60e07f87f973b70c4883f8c8b5dacfde6b0fbf4), addr: NodeAddr { ip: 222.121.139.31, port: 51413 } }
2024-05-23T20:44:30.923697Z DEBUG kademliar::app: node tx_id=TxId(6c61) distance=Distance(LCP: 10, value: 003c19154b074a151ef2858bbc64284fb096bb63) node=Node { id: NodeId(a60688f2c5f8c89497348bf0511dfa5d71159465), addr: NodeAddr { ip: 94.141.170.19, port: 49001 } }
2024-05-23T20:44:30.923735Z DEBUG kademliar::app: main: recv msg=SendSuccess(TxId(6e61))
2024-05-23T20:44:30.962124Z DEBUG kademliar::krpc: receiver: recv src=5.228.82.253:8349 len=287 data_hex="64323a6970363a54f80421d93b313a7264323a696432303aa63c3223a4d5d7021786e0ce429b27b1b530df76353a6e6f6465733230383aa63ac27341cea96590d91adf98431228cb7d17ad55aecac609bea63a6751a60af38449321e7e071c950b83b3f071a213384621d1a63bc797622178897c43b51bc8cf33d0d1022fd35f18805b7429a63b2049f1f1bbe9ebb3a6db3c870c3e99245e528713b9138e02a63883b8ec3616a90cbfd021d3d81d00d51caee8d43b6a700d9fa63821c1a51325626e93321ef44b453f1c1ed33757bb1ec91ae1a639e9dcadb343574e9058883ccb8113f44273fc5bd7f955051aa63913ce89c9ea1f303da3449ab4051b28f59de67317fd93806165313a74323a6e61313a76343a5554b7ec313a79313a7265"
2024-05-23T20:44:30.962317Z DEBUG kademliar::app: main: recv msg=Response(FindNodeResponse { tx_id: TxId(6e61), nodes: [Node { id: NodeId(a63ac27341cea96590d91adf98431228cb7d17ad), addr: NodeAddr { ip: 85.174.202.198, port: 2494 } }, Node { id: NodeId(a63a6751a60af38449321e7e071c950b83b3f071), addr: NodeAddr { ip: 162.19.56.70, port: 8657 } }, Node { id: NodeId(a63bc797622178897c43b51bc8cf33d0d1022fd3), addr: NodeAddr { ip: 95.24.128.91, port: 29737 } }, Node { id: NodeId(a63b2049f1f1bbe9ebb3a6db3c870c3e99245e52), addr: NodeAddr { ip: 135.19.185.19, port: 36354 } }, Node { id: NodeId(a63883b8ec3616a90cbfd021d3d81d00d51caee8), addr: NodeAddr { ip: 212.59.106.112, port: 3487 } }, Node { id: NodeId(a63821c1a51325626e93321ef44b453f1c1ed337), addr: NodeAddr { ip: 87.187.30.201, port: 6881 } }, Node { id: NodeId(a639e9dcadb343574e9058883ccb8113f44273fc), addr: NodeAddr { ip: 91.215.249.85, port: 1306 } }, Node { id: NodeId(a63913ce89c9ea1f303da3449ab4051b28f59de6), addr: NodeAddr { ip: 115.23.253.147, port: 32865 } }] })
2024-05-23T20:44:30.962381Z DEBUG kademliar::app: response tx_id=TxId(6e61) res=FindNodeResponse { tx_id: TxId(6e61), nodes: [Node { id: NodeId(a63ac27341cea96590d91adf98431228cb7d17ad), addr: NodeAddr { ip: 85.174.202.198, port: 2494 } }, Node { id: NodeId(a63a6751a60af38449321e7e071c950b83b3f071), addr: NodeAddr { ip: 162.19.56.70, port: 8657 } }, Node { id: NodeId(a63bc797622178897c43b51bc8cf33d0d1022fd3), addr: NodeAddr { ip: 95.24.128.91, port: 29737 } }, Node { id: NodeId(a63b2049f1f1bbe9ebb3a6db3c870c3e99245e52), addr: NodeAddr { ip: 135.19.185.19, port: 36354 } }, Node { id: NodeId(a63883b8ec3616a90cbfd021d3d81d00d51caee8), addr: NodeAddr { ip: 212.59.106.112, port: 3487 } }, Node { id: NodeId(a63821c1a51325626e93321ef44b453f1c1ed337), addr: NodeAddr { ip: 87.187.30.201, port: 6881 } }, Node { id: NodeId(a639e9dcadb343574e9058883ccb8113f44273fc), addr: NodeAddr { ip: 91.215.249.85, port: 1306 } }, Node { id: NodeId(a63913ce89c9ea1f303da3449ab4051b28f59de6), addr: NodeAddr { ip: 115.23.253.147, port: 32865 } }] }
2024-05-23T20:44:30.962428Z DEBUG kademliar::app: node tx_id=TxId(6e61) distance=Distance(LCP: 17, value: 00005394cf312be4191f14a4753ac03a0afe38ab) node=Node { id: NodeId(a63ac27341cea96590d91adf98431228cb7d17ad), addr: NodeAddr { ip: 85.174.202.198, port: 2494 } }
2024-05-23T20:44:30.962470Z DEBUG kademliar::app: node is closer --> query tx_id=TxId(6e61)
2024-05-23T20:44:30.962506Z DEBUG kademliar::app: node tx_id=TxId(6e61) distance=Distance(LCP: 16, value: 0000f6b628f57105c0f41005ea6547194230df77) node=Node { id: NodeId(a63a6751a60af38449321e7e071c950b83b3f071), addr: NodeAddr { ip: 162.19.56.70, port: 8657 } }
2024-05-23T20:44:30.962531Z DEBUG kademliar::app: node tx_id=TxId(6e61) distance=Distance(LCP: 15, value: 00015670ecdefa08f585bb6025b6e1c2108100d5) node=Node { id: NodeId(a63bc797622178897c43b51bc8cf33d0d1022fd3), addr: NodeAddr { ip: 95.24.128.91, port: 29737 } }
2024-05-23T20:44:30.962554Z DEBUG kademliar::app: node tx_id=TxId(6e61) distance=Distance(LCP: 15, value: 0001b1ae7f0e39686275a8a0d1fede2c58a77154) node=Node { id: NodeId(a63b2049f1f1bbe9ebb3a6db3c870c3e99245e52), addr: NodeAddr { ip: 135.19.185.19, port: 36354 } }
2024-05-23T20:44:30.962575Z DEBUG kademliar::app: node tx_id=TxId(6e61) distance=Distance(LCP: 14, value: 0002125f62c994288579de5a3ea1cf12149f81ee) node=Node { id: NodeId(a63883b8ec3616a90cbfd021d3d81d00d51caee8), addr: NodeAddr { ip: 212.59.106.112, port: 3487 } }
2024-05-23T20:44:30.962599Z DEBUG kademliar::app: node tx_id=TxId(6e61) distance=Distance(LCP: 14, value: 0002b0262beca7e3e7553c651932972ddd9dfc31) node=Node { id: NodeId(a63821c1a51325626e93321ef44b453f1c1ed337), addr: NodeAddr { ip: 87.187.30.201, port: 6881 } }
2024-05-23T20:44:30.962621Z DEBUG kademliar::app: node tx_id=TxId(6e61) distance=Distance(LCP: 14, value: 0003783b234cc1d6c75656f3d1b2530135c15cfa) node=Node { id: NodeId(a639e9dcadb343574e9058883ccb8113f44273fc), addr: NodeAddr { ip: 91.215.249.85, port: 1306 } }
2024-05-23T20:44:30.962643Z DEBUG kademliar::app: node tx_id=TxId(6e61) distance=Distance(LCP: 14, value: 000382290736689eb9fbad3f77cdd709e976b2e0) node=Node { id: NodeId(a63913ce89c9ea1f303da3449ab4051b28f59de6), addr: NodeAddr { ip: 115.23.253.147, port: 32865 } }
2024-05-23T20:44:30.962716Z DEBUG kademliar::krpc: sender: recv msg=Request(TxId(6f61))
2024-05-23T20:44:30.962828Z DEBUG kademliar::krpc: sender: send ok dst=NodeAddr { ip: 85.174.202.198, port: 2494 } req=FindNodeRequest { tx_id: TxId(6f61), node_id_self: NodeId(a63a91e78eff828189c60e7bed79d212c1832f06), node_id_target: NodeId(a63a91e78eff828189c60e7bed79d212c1832f06) }
2024-05-23T20:44:30.962920Z DEBUG kademliar::app: main: recv msg=SendSuccess(TxId(6f61))
2024-05-23T20:44:30.966687Z DEBUG kademliar::krpc: receiver: recv src=89.134.4.89:36626 len=297 data_hex="64323a6970363a54f80421d93b313a7264323a696432303aa60bac34097afa35108dbc9088a33ab39aac7105353a6e6f6465733230383aa631a2c14daa48055af399ff150773514442d9035abf6c9e9838a63674accc165ccfe2d48154991997b72be7488dba13cc4e1ae1a62df25be51ab32b296f8e4fdd2eff32d5aa9da73b1000a0ca1fa621e45c6ee9d2ba0a33da361da6bc181eed4ec55aa46005b598a63fb869d99477b201252975f8770b0a13562dc034cb3d971ae0a630d6f15772d484971d8de7390ac010075dbcf58bda60ef72c3a621716b5d2e8b21a90bc1bbe2665fe0661117687ae72fef41f1a63b5b6693985b193fcac992a0417c7de2d5b5e3d9c5bd271ae1313a706935353631316565313a74323a6d61313a76343a4c54012f313a79313a7265"
2024-05-23T20:44:30.966965Z DEBUG kademliar::app: main: recv msg=Response(FindNodeResponse { tx_id: TxId(6d61), nodes: [Node { id: NodeId(a631a2c14daa48055af399ff150773514442d903), addr: NodeAddr { ip: 90.191.108.158, port: 38968 } }, Node { id: NodeId(a63674accc165ccfe2d48154991997b72be7488d), addr: NodeAddr { ip: 186.19.204.78, port: 6881 } }, Node { id: NodeId(a62df25be51ab32b296f8e4fdd2eff32d5aa9da7), addr: NodeAddr { ip: 59.16.0.160, port: 51743 } }, Node { id: NodeId(a621e45c6ee9d2ba0a33da361da6bc181eed4ec5), addr: NodeAddr { ip: 90.164.96.5, port: 46488 } }, Node { id: NodeId(a63fb869d99477b201252975f8770b0a13562dc0), addr: NodeAddr { ip: 52.203.61.151, port: 6880 } }, Node { id: NodeId(a630d6f15772d484971d8de7390ac010075dbcf5), addr: NodeAddr { ip: 139.218.96.239, port: 29379 } }, Node { id: NodeId(a621716b5d2e8b21a90bc1bbe2665fe066111768), addr: NodeAddr { ip: 122.231.47.239, port: 16881 } }, Node { id: NodeId(a63b5b6693985b193fcac992a0417c7de2d5b5e3), addr: NodeAddr { ip: 217.197.189.39, port: 6881 } }] })
2024-05-23T20:44:30.967038Z DEBUG kademliar::app: response tx_id=TxId(6d61) res=FindNodeResponse { tx_id: TxId(6d61), nodes: [Node { id: NodeId(a631a2c14daa48055af399ff150773514442d903), addr: NodeAddr { ip: 90.191.108.158, port: 38968 } }, Node { id: NodeId(a63674accc165ccfe2d48154991997b72be7488d), addr: NodeAddr { ip: 186.19.204.78, port: 6881 } }, Node { id: NodeId(a62df25be51ab32b296f8e4fdd2eff32d5aa9da7), addr: NodeAddr { ip: 59.16.0.160, port: 51743 } }, Node { id: NodeId(a621e45c6ee9d2ba0a33da361da6bc181eed4ec5), addr: NodeAddr { ip: 90.164.96.5, port: 46488 } }, Node { id: NodeId(a63fb869d99477b201252975f8770b0a13562dc0), addr: NodeAddr { ip: 52.203.61.151, port: 6880 } }, Node { id: NodeId(a630d6f15772d484971d8de7390ac010075dbcf5), addr: NodeAddr { ip: 139.218.96.239, port: 29379 } }, Node { id: NodeId(a621716b5d2e8b21a90bc1bbe2665fe066111768), addr: NodeAddr { ip: 122.231.47.239, port: 16881 } }, Node { id: NodeId(a63b5b6693985b193fcac992a0417c7de2d5b5e3), addr: NodeAddr { ip: 217.197.189.39, port: 6881 } }] }
2024-05-23T20:44:30.967093Z DEBUG kademliar::app: node tx_id=TxId(6d61) distance=Distance(LCP: 12, value: 000b3326c355ca84d3359784f87ea14385c1f605) node=Node { id: NodeId(a631a2c14daa48055af399ff150773514442d903), addr: NodeAddr { ip: 90.191.108.158, port: 38968 } }
2024-05-23T20:44:30.967122Z DEBUG kademliar::app: node tx_id=TxId(6d61) distance=Distance(LCP: 12, value: 000ce54b42e9de4e6b128f2f746045a5ea64678b) node=Node { id: NodeId(a63674accc165ccfe2d48154991997b72be7488d), addr: NodeAddr { ip: 186.19.204.78, port: 6881 } }
2024-05-23T20:44:30.967146Z DEBUG kademliar::app: node tx_id=TxId(6d61) distance=Distance(LCP: 11, value: 001763bc6be531aaa0a9803430572d201429b2a1) node=Node { id: NodeId(a62df25be51ab32b296f8e4fdd2eff32d5aa9da7), addr: NodeAddr { ip: 59.16.0.160, port: 51743 } }
2024-05-23T20:44:30.967168Z DEBUG kademliar::app: node tx_id=TxId(6d61) distance=Distance(LCP: 11, value: 001b75bbe016503b83f5d44df0df6e0adf6e61c3) node=Node { id: NodeId(a621e45c6ee9d2ba0a33da361da6bc181eed4ec5), addr: NodeAddr { ip: 90.164.96.5, port: 46488 } }
2024-05-23T20:44:30.967190Z DEBUG kademliar::app: node tx_id=TxId(6d61) distance=Distance(LCP: 13, value: 0005298e576bf53388e3270e150ed918d2d502c6) node=Node { id: NodeId(a63fb869d99477b201252975f8770b0a13562dc0), addr: NodeAddr { ip: 52.203.61.151, port: 6880 } }
2024-05-23T20:44:30.967213Z DEBUG kademliar::app: node tx_id=TxId(6d61) distance=Distance(LCP: 12, value: 000a4716d98d56051edb839cd4731202c6de93f3) node=Node { id: NodeId(a630d6f15772d484971d8de7390ac010075dbcf5), addr: NodeAddr { ip: 139.218.96.239, port: 29379 } }
2024-05-23T20:44:30.967234Z DEBUG kademliar::app: node tx_id=TxId(6d61) distance=Distance(LCP: 11, value: 001be08cd3d109a020cdcfc00f1f8df2a792386e) node=Node { id: NodeId(a621716b5d2e8b21a90bc1bbe2665fe066111768), addr: NodeAddr { ip: 122.231.47.239, port: 16881 } }
2024-05-23T20:44:30.967256Z DEBUG kademliar::app: node tx_id=TxId(6d61) distance=Distance(LCP: 15, value: 0001ca811d67d998b60cc7e94d38ae6f23569ae5) node=Node { id: NodeId(a63b5b6693985b193fcac992a0417c7de2d5b5e3), addr: NodeAddr { ip: 217.197.189.39, port: 6881 } }
2024-05-23T20:44:31.247510Z DEBUG kademliar::app: main: recv msg=ResponseTimeout(TxId(6861))
2024-05-23T20:44:31.338884Z DEBUG kademliar::app: main: recv msg=ResponseTimeout(TxId(6961))
2024-05-23T20:44:31.439199Z DEBUG kademliar::app: main: recv msg=ResponseTimeout(TxId(6a61))
2024-05-23T20:44:31.643674Z DEBUG kademliar::app: main: recv msg=ResponseTimeout(TxId(6b61))
2024-05-23T20:44:31.643742Z ERROR kademliar::app: timeout --> remove tx_id=TxId(6b61)
2024-05-23T20:44:35.891296Z DEBUG kademliar::app: main: recv msg=ResponseTimeout(TxId(6d61))
2024-05-23T20:44:35.891346Z DEBUG kademliar::app: main: recv msg=ResponseTimeout(TxId(6c61))
2024-05-23T20:44:35.924504Z DEBUG kademliar::app: main: recv msg=ResponseTimeout(TxId(6e61))
2024-05-23T20:44:35.964653Z DEBUG kademliar::app: main: recv msg=ResponseTimeout(TxId(6f61))
2024-05-23T20:44:35.964699Z ERROR kademliar::app: timeout --> remove tx_id=TxId(6f61)
2024-05-23T20:44:35.964713Z DEBUG kademliar::app: done
k-bucket 0: [
    Node {
        id: NodeId(129791afac2ff0ffaf2d96818e30affad62eefd7),
        addr: NodeAddr {
            ip: 151.240.36.39,
            port: 24920,
        },
    },
    Node {
        id: NodeId(6d49c19da949e86261b4dd0a9c377802076b3195),
        addr: NodeAddr {
            ip: 177.181.3.99,
            port: 21148,
        },
    },
    Node {
        id: NodeId(4e4c42b9154bcc193819750bd0be623220761a96),
        addr: NodeAddr {
            ip: 172.59.161.64,
            port: 4456,
        },
    },
    Node {
        id: NodeId(4077169214e1cee4ce59063a74b6e3c7d5418a7d),
        addr: NodeAddr {
            ip: 41.105.111.148,
            port: 41798,
        },
    },
    Node {
        id: NodeId(2b5779bbb23c29bfb258e51a6c66c9bf0d95760f),
        addr: NodeAddr {
            ip: 181.124.134.254,
            port: 13239,
        },
    },
    Node {
        id: NodeId(4caa4d322d478549f43cd9766df9c8430a638ad1),
        addr: NodeAddr {
            ip: 77.22.180.50,
            port: 52245,
        },
    },
    Node {
        id: NodeId(06f3893be940a94020cd7fb6d08cf9bdcebcf2f5),
        addr: NodeAddr {
            ip: 95.25.148.132,
            port: 48627,
        },
    },
    Node {
        id: NodeId(06f22bd6ae529049f1f1bbe9ebb3a6db3c870ce1),
        addr: NodeAddr {
            ip: 207.38.56.52,
            port: 57589,
        },
    },
]
k-bucket 1: [
    Node {
        id: NodeId(c9c07155effad392af5cd0e5125ad7fd298133bb),
        addr: NodeAddr {
            ip: 186.216.183.227,
            port: 22372,
        },
    },
    Node {
        id: NodeId(d2ef43179e0fc8e0f398fdb91973d2d555735694),
        addr: NodeAddr {
            ip: 181.44.129.66,
            port: 19122,
        },
    },
    Node {
        id: NodeId(c1587ad970ec6f8e044bae7605556984c2eb3368),
        addr: NodeAddr {
            ip: 190.164.216.22,
            port: 6881,
        },
    },
    Node {
        id: NodeId(fa9bece9c949623fb4772488439011db769cf4d8),
        addr: NodeAddr {
            ip: 177.39.86.12,
            port: 30829,
        },
    },
]
k-bucket 2: [
    Node {
        id: NodeId(81e5906b5bf0d9b9badbda50cc60612049ecfba3),
        addr: NodeAddr {
            ip: 177.124.154.42,
            port: 35585,
        },
    },
    Node {
        id: NodeId(9ace994c70d4165087e60a1763bb25a2a2f70a4d),
        addr: NodeAddr {
            ip: 82.7.160.224,
            port: 6881,
        },
    },
]
k-bucket 3: [
    Node {
        id: NodeId(b454d76d0fcb67f70a239fa3ecce2c2d58c4bf50),
        addr: NodeAddr {
            ip: 176.145.73.20,
            port: 15976,
        },
    },
    Node {
        id: NodeId(b8815434b1e68b07e2b6cddf5ee18f5a84e1469b),
        addr: NodeAddr {
            ip: 87.67.111.62,
            port: 1067,
        },
    },
]
k-bucket 4: [
    Node {
        id: NodeId(ad9a0269161cc04abb3f1c7584961565fdec1a72),
        addr: NodeAddr {
            ip: 176.190.209.249,
            port: 3150,
        },
    },
]
k-bucket 5: [
    Node {
        id: NodeId(a3bd6ecf5c1b227798eca92c35bc1b3e1b57fbd0),
        addr: NodeAddr {
            ip: 128.73.170.65,
            port: 39456,
        },
    },
]
k-bucket 6: [
    Node {
        id: NodeId(a4598d10d22804c4b8b6e1df94d895b67a25ec54),
        addr: NodeAddr {
            ip: 89.142.17.100,
            port: 20789,
        },
    },
    Node {
        id: NodeId(a4088d9787f3ba49594adc10e8c9bbf7bf03ee8b),
        addr: NodeAddr {
            ip: 142.196.223.232,
            port: 6881,
        },
    },
    Node {
        id: NodeId(a501dedb1a053996b57738912f916e5590dcd05e),
        addr: NodeAddr {
            ip: 45.161.237.208,
            port: 5389,
        },
    },
    Node {
        id: NodeId(a4ba8c75198bd1deee026aa11b307029850c10e5),
        addr: NodeAddr {
            ip: 5.18.99.82,
            port: 14302,
        },
    },
]
k-bucket 7: [
    Node {
        id: NodeId(a7bddaf7ceef09f5765501d8ae49950a8a9b284a),
        addr: NodeAddr {
            ip: 114.34.124.247,
            port: 14919,
        },
    },
    Node {
        id: NodeId(a72f931aa0053b9efb418e3982d10a6bd1e5fbe3),
        addr: NodeAddr {
            ip: 93.124.80.20,
            port: 15723,
        },
    },
    Node {
        id: NodeId(a71f7bd6ae529049f1f1bbe9ebb3a6db3c870ce1),
        addr: NodeAddr {
            ip: 201.141.103.224,
            port: 51124,
        },
    },
    Node {
        id: NodeId(a76750d6ae529049f1f1bbe9ebb3a6db3c870ce1),
        addr: NodeAddr {
            ip: 213.22.149.135,
            port: 41963,
        },
    },
    Node {
        id: NodeId(a7b7e6170025d15813a79582d0d9c24b68b9aaee),
        addr: NodeAddr {
            ip: 188.230.252.16,
            port: 51413,
        },
    },
    Node {
        id: NodeId(a7ed5ad60dc2d5e16b956cb1c5a1287a4ce3cc1e),
        addr: NodeAddr {
            ip: 162.55.81.219,
            port: 50000,
        },
    },
    Node {
        id: NodeId(a7dfb52ae4b77b86c28b72f232719f4ec95723d9),
        addr: NodeAddr {
            ip: 59.23.162.167,
            port: 32701,
        },
    },
    Node {
        id: NodeId(a7c25608c4864979c6fb3159a6316a711eccae16),
        addr: NodeAddr {
            ip: 188.165.246.140,
            port: 57444,
        },
    },
]
k-bucket 8: [
    Node {
        id: NodeId(a6ba06650959bb0610b9536a51c4775bf06a46a6),
        addr: NodeAddr {
            ip: 46.22.56.134,
            port: 9360,
        },
    },
    Node {
        id: NodeId(a6a7d7bee6f36a9affe6201402c8f46ac7688ac2),
        addr: NodeAddr {
            ip: 201.15.185.122,
            port: 49909,
        },
    },
    Node {
        id: NodeId(a6c98ecff95b40cc2778f7cafda349acbfdb15dd),
        addr: NodeAddr {
            ip: 45.152.210.84,
            port: 49643,
        },
    },
    Node {
        id: NodeId(a6fbc457db9db6c4f84494f3fef4822af93281a6),
        addr: NodeAddr {
            ip: 95.168.168.29,
            port: 30192,
        },
    },
    Node {
        id: NodeId(a6f15ccc6a20833731f3c76bcfa64d4542892e96),
        addr: NodeAddr {
            ip: 212.32.212.183,
            port: 11269,
        },
    },
]
k-bucket 9: [
    Node {
        id: NodeId(a678e2c349d39235dda2c7ea02a5a7cf05c8fd11),
        addr: NodeAddr {
            ip: 178.213.194.24,
            port: 12478,
        },
    },
    Node {
        id: NodeId(a652f592387dbc9eb8f38617199c3401ad846c2f),
        addr: NodeAddr {
            ip: 70.117.136.53,
            port: 34557,
        },
    },
    Node {
        id: NodeId(a661bd5c78695608ccc3a49f89d46d2cdbf51133),
        addr: NodeAddr {
            ip: 221.160.134.115,
            port: 56925,
        },
    },
    Node {
        id: NodeId(a667d5191430cbb351b02bf0a01522d241a54d2f),
        addr: NodeAddr {
            ip: 173.47.209.60,
            port: 6889,
        },
    },
]
k-bucket 10: [
    Node {
        id: NodeId(a60bac34097afa35108dbc9088a33ab39aac7105),
        addr: NodeAddr {
            ip: 89.134.4.89,
            port: 36626,
        },
    },
    Node {
        id: NodeId(a61e95b3a6db3c870c3e99245e0d1c06b747dee9),
        addr: NodeAddr {
            ip: 95.24.81.238,
            port: 50314,
        },
    },
    Node {
        id: NodeId(a6121718c4d0a23e1637866cc489225827544759),
        addr: NodeAddr {
            ip: 92.240.203.114,
            port: 32070,
        },
    },
    Node {
        id: NodeId(a60e07f87f973b70c4883f8c8b5dacfde6b0fbf4),
        addr: NodeAddr {
            ip: 222.121.139.31,
            port: 51413,
        },
    },
    Node {
        id: NodeId(a60688f2c5f8c89497348bf0511dfa5d71159465),
        addr: NodeAddr {
            ip: 94.141.170.19,
            port: 49001,
        },
    },
]
k-bucket 11: [
    Node {
        id: NodeId(a62b37c84a8af54c4ab9a4edfd1c930838210bd3),
        addr: NodeAddr {
            ip: 5.16.49.92,
            port: 6882,
        },
    },
    Node {
        id: NodeId(a62274660472ffdc1e7bedfbcd39dc96d108566d),
        addr: NodeAddr {
            ip: 38.175.200.53,
            port: 63898,
        },
    },
    Node {
        id: NodeId(a62df25be51ab32b296f8e4fdd2eff32d5aa9da7),
        addr: NodeAddr {
            ip: 59.16.0.160,
            port: 51743,
        },
    },
    Node {
        id: NodeId(a621e45c6ee9d2ba0a33da361da6bc181eed4ec5),
        addr: NodeAddr {
            ip: 90.164.96.5,
            port: 46488,
        },
    },
    Node {
        id: NodeId(a621716b5d2e8b21a90bc1bbe2665fe066111768),
        addr: NodeAddr {
            ip: 122.231.47.239,
            port: 16881,
        },
    },
]
k-bucket 12: [
    Node {
        id: NodeId(a633f1f8cd7c19b53998e62d763266042ba69518),
        addr: NodeAddr {
            ip: 122.254.3.232,
            port: 22943,
        },
    },
    Node {
        id: NodeId(a631a2c14daa48055af399ff150773514442d903),
        addr: NodeAddr {
            ip: 90.191.108.158,
            port: 38968,
        },
    },
    Node {
        id: NodeId(a63674accc165ccfe2d48154991997b72be7488d),
        addr: NodeAddr {
            ip: 186.19.204.78,
            port: 6881,
        },
    },
    Node {
        id: NodeId(a630d6f15772d484971d8de7390ac010075dbcf5),
        addr: NodeAddr {
            ip: 139.218.96.239,
            port: 29379,
        },
    },
]
k-bucket 13: [
    Node {
        id: NodeId(a63c3223a4d5d7021786e0ce429b27b1b530df76),
        addr: NodeAddr {
            ip: 5.228.82.253,
            port: 8349,
        },
    },
    Node {
        id: NodeId(a63fb869d99477b201252975f8770b0a13562dc0),
        addr: NodeAddr {
            ip: 52.203.61.151,
            port: 6880,
        },
    },
]
k-bucket 14: [
    Node {
        id: NodeId(a63883b8ec3616a90cbfd021d3d81d00d51caee8),
        addr: NodeAddr {
            ip: 212.59.106.112,
            port: 3487,
        },
    },
    Node {
        id: NodeId(a63821c1a51325626e93321ef44b453f1c1ed337),
        addr: NodeAddr {
            ip: 87.187.30.201,
            port: 6881,
        },
    },
    Node {
        id: NodeId(a639e9dcadb343574e9058883ccb8113f44273fc),
        addr: NodeAddr {
            ip: 91.215.249.85,
            port: 1306,
        },
    },
    Node {
        id: NodeId(a63913ce89c9ea1f303da3449ab4051b28f59de6),
        addr: NodeAddr {
            ip: 115.23.253.147,
            port: 32865,
        },
    },
]
k-bucket 15: [
    Node {
        id: NodeId(a63bc797622178897c43b51bc8cf33d0d1022fd3),
        addr: NodeAddr {
            ip: 95.24.128.91,
            port: 29737,
        },
    },
    Node {
        id: NodeId(a63b2049f1f1bbe9ebb3a6db3c870c3e99245e52),
        addr: NodeAddr {
            ip: 135.19.185.19,
            port: 36354,
        },
    },
    Node {
        id: NodeId(a63b5b6693985b193fcac992a0417c7de2d5b5e3),
        addr: NodeAddr {
            ip: 217.197.189.39,
            port: 6881,
        },
    },
]
k-bucket 16: [
    Node {
        id: NodeId(a63a6751a60af38449321e7e071c950b83b3f071),
        addr: NodeAddr {
            ip: 162.19.56.70,
            port: 8657,
        },
    },
]
k-bucket 17: [
    Node {
        id: NodeId(a63ac27341cea96590d91adf98431228cb7d17ad),
        addr: NodeAddr {
            ip: 85.174.202.198,
            port: 2494,
        },
    },
]
```
