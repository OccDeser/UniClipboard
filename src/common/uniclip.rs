use super::super::datatype::{LocalClipboard, RemoteClipboard, UniclipDataFrame};
use super::clipboard;
use super::packer;
use serde_encrypt::shared_key::SharedKey;
use std::net::UdpSocket;

pub struct Uniclip {
    port: u16,
    key: SharedKey,
    peers: Vec<RemoteClipboard>,
    server: UdpSocket,
}

impl Uniclip {
    pub fn new(local_clip: LocalClipboard) -> Uniclip {
        let mut uniclip = Uniclip {
            port: local_clip.port,
            peers: Vec::new(),
            key: packer::pwd2key(local_clip.password),
            server: UdpSocket::bind(format!("127.0.0.1:{}" , local_clip.port)).unwrap(),
        };

        if local_clip.peer.port != 0 {
            uniclip.peers.push(local_clip.peer);
        }

        uniclip
    }

    fn send_to_peer(&self, peer: &RemoteClipboard, data: UniclipDataFrame) {
        let socket = UdpSocket::bind(format!("127.0.0.1:0")).unwrap();
        socket
            .connect(format!("{}:{}", peer.host, peer.port))
            .unwrap();

        let buf = packer::pack(data, &self.key);
        let buf = buf.as_slice();
        socket.send_to(buf, format!("{}:{}", peer.host, peer.port)).unwrap();
    }

    fn send_to_peers(&self, data: UniclipDataFrame) {
        for peer in &self.peers {
            self.send_to_peer(peer, data.clone());
        }
    }

    pub fn start(&mut self) {

    }
}
