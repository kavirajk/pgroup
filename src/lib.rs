use crossbeam_channel::Receiver;
use std::collections::HashMap;
use std::net::{SocketAddr, UdpSocket};

struct Group {
    me: Node,
    peers: HashMap<String, Node>,

    // ack for specific ping's seq_no.
    ack_handlers: HashMap<usize, Receiver<Packet>>,
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
            Packet::Ping => {}
            Packet::Ack => {}
            Packet::PingReq => {}
            Packet::IndirectAck => {}
            _ => {}
        }

        Ok(())
    }
}

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

enum NodeState {
    Alive,
    Dead,
    Suspect,
}

enum Packet {
    Ping,
    Ack,
    PingReq,
    IndirectAck,
    Alive,
    Joined,
    Left,
    Failed,
}

fn encode_packet(pkt: Packet) -> Result<Vec<u8>, String> {
    unimplemented!()
}

fn decode_packet(buf: &[u8]) -> Result<Packet, String> {
    unimplemented!()
}
