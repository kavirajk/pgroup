use crossbeam_channel::{after, select, tick, unbounded, Receiver, Sender};
use std::env;
use std::net::{Ipv4Addr, ToSocketAddrs, UdpSocket};
use std::sync::Arc;
use std::thread;
use std::time::Duration;

fn main() -> std::io::Result<()> {
    let mut args: Vec<String> = env::args().skip(1).collect();

    if args.len() < 2 {
        panic!("should run with atleast two args(listener and atleast single peer)")
    }

    let listen = args[0].clone();

    let peers = args[1..].to_vec();

    let socket = UdpSocket::bind(listen)?;

    let sock = Arc::new(socket);

    let (tx, rx) = unbounded::<Vec<u8>>();

    let sender = sock.clone();
    thread::spawn(move || loop {
        let ticker = tick(Duration::from_secs(1));
        loop {
            select! {
            recv(ticker) -> _ => probe_all_peers(&sender, &rx, &peers).unwrap(),
            }
        }
    });

    println!("listening on {}", sock.local_addr()?);
    loop {
        let mut buf = vec![0; 1024];
        let (size, sender) = sock.recv_from(&mut buf)?;

        let msg = std::str::from_utf8(&buf[..size]).unwrap();

        match msg {
            "Ping" => {
                send(&sock, b"Ack".to_vec(), sender)?;
            }
            "Ack" => {
                tx.send(msg.as_bytes().to_vec()).unwrap();
            }
            _ => {}
        }
    }
}

fn send<T: ToSocketAddrs>(sock: &UdpSocket, msg: Vec<u8>, to: T) -> std::io::Result<usize> {
    sock.send_to(&msg, to)
}

fn probe_all_peers<T: ToSocketAddrs>(
    sock: &UdpSocket,
    rx: &Receiver<Vec<u8>>,
    peers: &Vec<T>,
) -> std::io::Result<()> {
    for peer in peers {
        probeNode(sock, peer, rx)?
    }
    Ok(())
}

// probe is a single failure detector round.
fn probeNode<T: ToSocketAddrs>(
    sock: &UdpSocket,
    addr: T,
    rx: &Receiver<Vec<u8>>,
) -> std::io::Result<()> {
    // 1. Choose Mj and send Ping, and start the timer.
    // 2. If no repsonse from Mj within timer, send PingReq(Mj) to K other peers and starts timer.
    // 3. If no response from Mj within timer, mark Mj as failed
    // 4. If got response from either Ping or PingReq(Mj), then return success

    // ping
    send(sock, "Ping".as_bytes().to_vec(), addr)?;

    let timeout = after(Duration::from_secs(1));

    // wait for ack
    select! {
    recv(timeout)-> _ => println!("timout in pinging the peer. sorry!"),
    recv(rx) -> _ => println!("received ack")
    }

    Ok(())
}
