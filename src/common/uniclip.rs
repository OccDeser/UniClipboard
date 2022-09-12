use crate::datatype::payload_type::PEER;

use super::super::datatype::{
    LocalClipboard, payload_type, RemoteClipboard, UniclipPayload, UNICLIP_DATA_LIMIT,
};

use super::{clipboard, hotkey, message, packer};
use hotkey::{Hotkey, HotkeyManager};
use rand::prelude::*;
use serde_encrypt::shared_key::SharedKey;
use std::collections::HashMap;
use std::io::{Read, Write};
use std::mem::MaybeUninit;
use std::net::{TcpListener, TcpStream};
use std::sync::Mutex;
use std::thread;
use std::time;

struct MQItem {
    index: usize,
    data: UniclipPayload,
}

static mut PORT: u16 = 0;
static mut MQ: MaybeUninit<Mutex<HashMap<String, Vec<MQItem>>>> = MaybeUninit::uninit();
static mut PEERS: MaybeUninit<Mutex<Vec<RemoteClipboard>>> = MaybeUninit::uninit();
static mut HANDLERS: MaybeUninit<Mutex<Vec<UniclipPeerHandler>>> = MaybeUninit::uninit();

pub fn init() {
    unsafe {
        let peers = PEERS.as_mut_ptr();
        peers.write(Mutex::new(Vec::new()));
        let handlers = HANDLERS.as_mut_ptr();
        handlers.write(Mutex::new(Vec::new()));

        let mq = MQ.as_mut_ptr();
        let mut mq_data = HashMap::new();
        mq_data.insert(payload_type::PORT_RES.clone(), Vec::new());
        mq_data.insert(payload_type::ECHO_RES.clone(), Vec::new());
        mq_data.insert(payload_type::UPDATE.clone(), Vec::new());
        mq_data.insert(payload_type::UPDATE_RES.clone(), Vec::new());
        mq_data.insert(payload_type::UPDATE_BIG_ACK.clone(), Vec::new());
        mq_data.insert(payload_type::UPDATE_BIG_FINISH.clone(), Vec::new());
        mq_data.insert(payload_type::QUIT_RES.clone(), Vec::new());
        mq.write(Mutex::new(mq_data));
    }
}

fn handle(index: usize) {
    thread::spawn(move || unsafe {
        let handlers = HANDLERS.as_mut_ptr().read();
        let handler = &handlers.lock().unwrap()[index];
        let need_big = false;
        let big_size: usize = 0;
        loop {
            let data: UniclipPayload;
            if !need_big {
                data = handler.recv();
            } else {
                data = handler.recv_big(big_size);
            }
            match data {
                UniclipPayload::Echo(data) => {
                    handler.send(UniclipPayload::EchoRes(data + 1));
                }
                UniclipPayload::EchoRes(..) => {
                    insert(index, &payload_type::ECHO_RES, data);
                }
                UniclipPayload::Peer(rand_a) => {
                    let peers = PEERS.as_ptr();
                    let res =
                        UniclipPayload::PeerList(rand_a + 1, (*peers).lock().unwrap().clone());
                    handler.send(res);
                }
                UniclipPayload::PeerList(..) => {
                    insert(index, &payload_type::PEER_LIST, data);
                }
                UniclipPayload::Port(rand_a) => {
                    let port = PORT;
                    let res = UniclipPayload::PortRes(rand_a + 1, port);
                    handler.send(res);
                }
                UniclipPayload::PortRes(..) => {
                    insert(index, &payload_type::PORT_RES, data);
                }
                UniclipPayload::Update(hash, data) => {
                    let data_hash = packer::hash(&data);
                    if hash == data_hash {
                        let res = UniclipPayload::UpdateRes(data.len());
                        clipboard::set(data);
                        handler.send(res);
                    } else {
                        let res = UniclipPayload::Error("Update text hash error".to_string());
                        handler.send(res);
                    }
                }
                UniclipPayload::UpdateRes(..) => {
                    insert(index, &payload_type::UPDATE_RES, data);
                }
                _ => {
                    message::error("Invalid uniclip data.".to_string());
                    std::process::exit(-1);
                }
            }
        }
    });
}

fn insert(index: usize, mtype: &String, data: UniclipPayload) {
    unsafe {
        let mq = MQ.as_mut_ptr();
        let mut mq = (*mq).lock().unwrap();
        mq.get_mut(mtype).unwrap().push(MQItem { index, data });
    }
}

fn acquire(index: usize, mtype: &String) -> UniclipPayload {
    unsafe {
        let mq = MQ.as_mut_ptr();
        let mut mq = (*mq).lock().unwrap();
        loop {
            let queue = mq.get_mut(mtype).unwrap();
            for i in 0..queue.len() {
                if queue[i].index == index {
                    let data = queue.remove(i);
                    return data.data;
                }
            }
            thread::sleep(time::Duration::from_millis(10));
        }
    };
}

fn add_handler(mut handler: UniclipPeerHandler) -> usize {
    unsafe {
        let handlers = HANDLERS.as_mut_ptr();
        let mut handlers = (*handlers).lock().unwrap();
        let handler_ind = handlers.len();
        handler.set_index(handler_ind);
        handlers.push(handler);
        handle(handler_ind);
        handler_ind
    }
}

fn add_peer(key: &SharedKey, remote: &RemoteClipboard) {
    message::success(
        "success".to_string(),
        format!("Connected to {}", remote.host),
    );
    unsafe {
        let peers = PEERS.as_mut_ptr();
        (*peers).lock().unwrap().push(remote.clone());
        add_handler(UniclipPeerHandler::new(key.clone(), remote.clone()));
    }
}

fn add_stream(key: &SharedKey, stream: TcpStream) {
    let host = stream.peer_addr().unwrap().ip().to_string();
    let handler = UniclipPeerHandler::from(key.clone(), stream);
    let rand_a: u32 = random();
    handler.send(UniclipPayload::Port(rand_a));
    let index = add_handler(handler);
    let data = acquire(index, &payload_type::PORT_RES);
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
    let address = RemoteClipboard { host, port };
    unsafe {
        let peers = PEERS.as_mut_ptr().read();
        peers.lock().unwrap().push(address);
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
            let data = acquire(handler.index, &payload_type::PEER_LIST);
            println!("DATA: {:?}", data);
            let peer_list = match data {
                UniclipPayload::PeerList(rand_b, peer_list) => {
                    if rand_a + 1 == rand_b {
                        peer_list
                    } else {
                        message::error("Uniclip: PeerList rand error".to_string());
                        return;
                    }
                }
                _ => {
                    message::error("Uniclip: PeerList parse error".to_string());
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

fn broadcast(data: UniclipPayload) {
    unsafe {
        let handlers = HANDLERS.as_mut_ptr();
        let handlers = (*handlers).lock().unwrap();
        for handler in handlers.iter() {
            handler.send(data.clone());
        }
    }
}

pub struct UniclipPeerHandler {
    key: SharedKey,
    index: usize,
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
        UniclipPeerHandler {
            key,
            index: 0,
            stream,
        }
    }

    pub fn from(key: SharedKey, stream: TcpStream) -> UniclipPeerHandler {
        let handler = UniclipPeerHandler {
            key,
            index: 0,
            stream,
        };
        handler
    }

    pub fn set_index(&mut self, index: usize) {
        self.index = index;
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

        let size = match res {
            Ok(s) => s,
            Err(error) => {
                message::error(format!("{}", error));
                0
            }
        };

        let buf = &buffer[0..size];
        let data = packer::unpack(Vec::from(buf), key);
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

        unsafe { PORT = local_clip.port.clone() };

        Uniclip {
            port: local_clip.port,
            key,
            hotkey,
        }
    }

    fn listen_hotkey() {
        let text = clipboard::get();
        // message::info(format!("debug: Read clipboard data: {}", text));
        println!("debug: Read clipboard data: {}", text);
        let hash = packer::hash(&text);
        let data = UniclipPayload::Update(hash, text);
        broadcast(data);
    }

    fn listen_port(&self) {
        let key = self.key.clone();
        let listener = TcpListener::bind(format!("0.0.0.0:{}", self.port));

        thread::spawn(move || {
            // 判断是否有新的连接
            for stream in listener.unwrap().incoming() {
                match stream {
                    Ok(stream) => {
                        add_stream(&key, stream);
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
