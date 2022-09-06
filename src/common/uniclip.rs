use super::super::datatype::{LocalClipboard, RemoteClipboard, UniclipPayload, UNICLIP_DATA_LIMIT};

use super::{clipboard, hotkey, message, packer};
use hotkey::Hotkey;
use rand::prelude::*;
use serde_encrypt::shared_key::SharedKey;
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::sync::{Arc, Mutex};
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
}

#[derive(Debug, Clone)]
pub struct Uniclip {
    port: u16,
    key: SharedKey,
    remote: RemoteClipboard,
    copy_keys: Vec<hotkey::Keycode>,
    paste_keys: Vec<hotkey::Keycode>,
}

impl Uniclip {
    pub fn new(
        local_clip: LocalClipboard,
        copy_keys: Vec<hotkey::Keycode>,
        paste_keys: Vec<hotkey::Keycode>,
    ) -> Uniclip {
        Uniclip {
            port: local_clip.port,
            key: packer::pwd2key(local_clip.password),
            remote: local_clip.peer,
            copy_keys,
            paste_keys,
        }
    }
}

fn send(data: UniclipPayload, handler: &UniclipPeerHandler) {
    let key = &handler.key;
    let stream = &handler.stream;

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

fn recv(handler: &UniclipPeerHandler) -> UniclipPayload {
    let key = &handler.key;
    let stream = &handler.stream;

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

fn recv_big(handler: &UniclipPeerHandler, big_size: usize) -> UniclipPayload {
    let key = &handler.key;
    let stream = &handler.stream;

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

fn get_peers(remote: RemoteClipboard, key: &SharedKey) -> Arc<Mutex<Vec<UniclipPeerHandler>>> {
    let mut peers = Vec::new();
    peers.push(UniclipPeerHandler::new(key.clone(), remote));

    // request peer list
    let rand_a: u32 = random();
    let data = UniclipPayload::Peer(rand_a);
    send(data, &peers[0]);

    // receive peer list
    let data = recv(&peers[0]);
    let peers = match data {
        UniclipPayload::PeerList(rand_b, peer_list) => {
            if rand_a + 1 == rand_b {
                for peer in peer_list {
                    peers.push(UniclipPeerHandler::new(key.clone(), peer));
                }
                peers
            } else {
                Vec::new()
            }
        }
        _ => {
            message::error("Invalid peer list".to_string());
            std::process::exit(-1);
        }
    };

    Arc::new(Mutex::new(peers))
}

fn listen_peers(uniclip: Uniclip, peers: Arc<Mutex<Vec<UniclipPeerHandler>>>) {
    let listener = TcpListener::bind(format!("127.0.0.1:{}", uniclip.port)).unwrap();
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                let handler = UniclipPeerHandler {
                    key: uniclip.key.clone(),
                    stream,
                };
                peers.lock().unwrap().push(handler);
            }
            Err(error) => {
                message::error(format!("{}", error));
            }
        }
    }
}

fn listen_copy() {}

fn listen_paste() {}

pub fn run(uniclip: Uniclip) {
    let mut hk_manager = hotkey::HotkeyManager::new();
    hk_manager.register(Hotkey::new(uniclip.copy_keys.clone(), listen_copy));
    hk_manager.register(Hotkey::new(uniclip.paste_keys.clone(), listen_paste));
    hk_manager.listen();

    let peers = get_peers(uniclip.remote.clone(), &uniclip.key);
    thread::spawn(move || listen_peers(uniclip, peers));
}
