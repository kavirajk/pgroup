use bincode;
use crossbeam_channel::Receiver;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::net::{SocketAddr, ToSocketAddrs, UdpSocket};

#[derive(Debug)]
struct Group {
    me: Node,
    peers: HashMap<String, Node>,

    // ack for specific ping's seq_no.
    ack_handlers: HashMap<u32, Receiver<Packet>>,
}

impl Group {
    fn members(&self) -> Vec<Node> {
        unimplemented!()
    }

    fn new(me: Node, seed_peers: &[Node]) -> Self {
        unimplemented!()
    }

    fn probe_peers(&self) {
        unimplemented!()
    }

    fn probe(&self, node: &Node) {
        unimplemented!()
    }

    fn packet_listener(&self) -> Result<(), String> {
        let mut buf: Vec<u8> = vec![0; 1024];

        self.me.sock.recv(&mut buf).unwrap();

        let pkt = decode_packet(&buf).unwrap();

        match pkt {
            Packet::Ping { from, seq_no } => if self.ack_handlers.contains_key(&seq_no) {},
            Packet::Ack { from, seq_no } => {}
            Packet::PingReq => {}
            Packet::IndirectAck => {}
            _ => {}
        }

        Ok(())
    }

    fn send<T: ToSocketAddrs>(sock: &UdpSocket, msg: Vec<u8>, to: T) -> std::io::Result<usize> {
        sock.send_to(&msg, to)
    }

    fn encode_and_send() {}
}

#[derive(Debug)]
struct Node {
    name: String,
    seq_no: u64,
    incar_no: u64,
    addr: SocketAddr,
    sock: UdpSocket,
    state: NodeState,
}

impl Node {
    fn next_seq_no(&self) {
        unimplemented!()
    }

    fn next_incar_no(&self) {
        unimplemented!()
    }
}

#[derive(Debug)]
enum NodeState {
    Alive,
    Dead,
    Suspect,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
enum Packet {
    Ping { from: String, seq_no: u32 },
    Ack { from: String, seq_no: u32 },
    PingReq,
    IndirectAck,
    Alive,
    Joined,
    Left,
    Failed,
}

fn encode_packet(pkt: &Packet) -> Result<Vec<u8>, String> {
    let buf = bincode::serialize(pkt).unwrap();
    Ok(buf)
}

fn decode_packet(buf: &[u8]) -> Result<Packet, String> {
    let pkt: Packet = bincode::deserialize(buf).unwrap();
    Ok(pkt)
}

#[test]
fn test_encode_decode() {
    let before = Packet::Ping {
        from: "me".to_owned(),
        seq_no: 1234,
    };

    let buf = encode_packet(&before).unwrap();

    let after = decode_packet(&buf).unwrap();

    assert_eq!(before, after);
}
