use super::super::datatype::{LocalClipboard, RemoteClipboard, UniclipPayload, UNICLIP_DATA_LIMIT};

use super::{clipboard, hotkey, message, packer};
use hotkey::{Hotkey, HotkeyManager};
use rand::prelude::*;
use serde_encrypt::shared_key::SharedKey;
use std::io::{Read, Write};
use std::mem::MaybeUninit;
use std::net::{TcpListener, TcpStream};
use std::sync::Mutex;
use std::thread;

pub struct UniclipPeerHandler {
    key: SharedKey,
    stream: TcpStream,
}

impl UniclipPeerHandler {
    pub fn new(key: SharedKey, remote: RemoteClipboard) -> UniclipPeerHandler {
        let stream = TcpStream::connect(format!("{}:{}", remote.host, remote.port));
        let stream = match stream {
            Ok(stream) => stream,
            Err(error) => {
                message::error(format!("{}", error));
                std::process::exit(-1);
            }
        };
        UniclipPeerHandler { key, stream }
    }

    pub fn send(&self, data: UniclipPayload) {
        let key = &self.key;
        let mut stream = &self.stream;

        let buf = packer::pack(data, key);
        let buffer = buf.as_slice();

        let res = stream.write(&buffer);

        match res {
            Ok(_) => (),
            Err(error) => {
                message::error(format!("{}", error));
            }
        }
    }

    pub fn recv(&self) -> UniclipPayload {
        let key = &self.key;
        let mut stream = &self.stream;

        let mut buffer = [0; UNICLIP_DATA_LIMIT];
        let res = stream.read(&mut buffer);

        match res {
            Ok(_) => (),
            Err(error) => {
                message::error(format!("{}", error));
            }
        }

        let data = packer::unpack(Vec::from(buffer), key);
        data
    }

    pub fn recv_big(&self, big_size: usize) -> UniclipPayload {
        let key = &self.key;
        let mut stream = &self.stream;

        let mut buffer = vec![0u8; big_size + UNICLIP_DATA_LIMIT];
        let res = stream.read(&mut buffer);

        match res {
            Ok(_) => (),
            Err(error) => {
                message::error(format!("{}", error));
            }
        }

        let data = packer::unpack(Vec::from(buffer), key);
        data
    }

    pub fn address(&self) -> RemoteClipboard {
        let host = self.stream.peer_addr().unwrap().ip().to_string();

        let rand_a: u32 = random();
        self.send(UniclipPayload::Port(rand_a));
        let data = self.recv();
        let port: u16 = match data {
            UniclipPayload::PortRes(rand_b, port) => {
                if rand_a + 1 == rand_b {
                    port
                } else {
                    0
                }
            }
            _ => {
                message::error("Invalid uniclip data.".to_string());
                std::process::exit(-1);
            }
        };

        RemoteClipboard { host, port }
    }
}

static mut PEERS: MaybeUninit<Mutex<Vec<RemoteClipboard>>> = MaybeUninit::uninit();
static mut HANDLERS: MaybeUninit<Mutex<Vec<UniclipPeerHandler>>> = MaybeUninit::uninit();

pub fn init() {
    unsafe {
        let peers = PEERS.as_mut_ptr();
        peers.write(Mutex::new(Vec::new()));
        let handlers = HANDLERS.as_mut_ptr();
        handlers.write(Mutex::new(Vec::new()));
    }
}

fn handle(index: usize) {
    thread::spawn(move || unsafe {
        let handlers = HANDLERS.as_mut_ptr().read();
        let handler = &handlers.lock().unwrap()[index];
        loop {
            let data = handler.recv();
            match data {
                UniclipPayload::Echo(data) => {
                    handler.send(UniclipPayload::EchoRes(data + 1));
                }
                UniclipPayload::Peer(rand_a) => {
                    let peers = PEERS.as_ptr();
                    let res =
                        UniclipPayload::PeerList(rand_a + 1, (*peers).lock().unwrap().clone());
                    handler.send(res);
                }
                _ => {
                    message::error("Invalid uniclip data.".to_string());
                    std::process::exit(-1);
                }
            }
        }
    });
}

fn add_handler(handler: UniclipPeerHandler) {
    unsafe {
        let handlers = HANDLERS.as_mut_ptr();
        let mut handlers = (*handlers).lock().unwrap();
        let handler_ind = handlers.len();
        handlers.push(handler);
        handle(handler_ind);
    }
}

fn add_peer(key: &SharedKey, remote: &RemoteClipboard) {
    unsafe {
        let peers = PEERS.as_mut_ptr();
        (*peers).lock().unwrap().push(remote.clone());
        add_handler(UniclipPeerHandler::new(key.clone(), remote.clone()));
    }
}

fn get_peers() {
    unsafe {
        let peers = PEERS.as_mut_ptr();
        let peers = (*peers).lock().unwrap();
        let handlers = HANDLERS.as_mut_ptr();
        let handlers = (*handlers).lock().unwrap();
        if handlers.len() > 0 {
            let handler = handlers.last().unwrap();
            let rand_a: u32 = random();
            handler.send(UniclipPayload::Peer(rand_a));
            let data = handler.recv();
            let peer_list = match data {
                UniclipPayload::PeerList(rand_b, peer_list) => {
                    if rand_a + 1 == rand_b {
                        peer_list
                    } else {
                        message::error("Uniclip: PeerList error".to_string());
                        return;
                    }
                }
                _ => {
                    message::error("Uniclip: PeerList error".to_string());
                    return;
                }
            };
            for p in peer_list.iter() {
                if !peers.contains(p) {
                    add_peer(&handler.key, p);
                }
            }
        }
    }
}

pub struct Uniclip {
    port: u16,
    key: SharedKey,
    hotkey: Vec<hotkey::Keycode>,
}

impl Uniclip {
    pub fn new(local_clip: &LocalClipboard, hotkey: Vec<hotkey::Keycode>) -> Uniclip {
        let key = packer::pwd2key(local_clip.password.clone());

        if local_clip.peer.port != 0 {
            add_peer(&key, &local_clip.peer);
            get_peers();
        }

        Uniclip {
            port: local_clip.port,
            key,
            hotkey,
        }
    }

    fn listen_hotkey() {
        println!("RECEIVED HOTKEY");
    }

    fn listen_port(&self) {
        let key = self.key.clone();
        let listener = TcpListener::bind(format!("127.0.0.1:{}", self.port));

        thread::spawn(move || unsafe {
            let peers = PEERS.as_mut_ptr();
            let handlers = HANDLERS.as_mut_ptr();

            // 判断是否有新的连接
            for stream in listener.unwrap().incoming() {
                match stream {
                    Ok(stream) => {
                        let handler = UniclipPeerHandler {
                            key: key.clone(),
                            stream,
                        };
                        (*peers).lock().unwrap().push(handler.address());
                        (*handlers).lock().unwrap().push(handler);
                    }
                    Err(error) => {
                        message::error(format!("{}", error));
                    }
                }
            }
        });
    }

    pub fn start(&mut self) {
        let mut hk_manager = HotkeyManager::new();
        let hk = Hotkey::new(self.hotkey.clone(), Self::listen_hotkey);
        hk_manager.register(hk);
        hk_manager.listen();

        self.listen_port();
    }
}
