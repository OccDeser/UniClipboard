#[derive(Debug, Serialize, Deserialize)]
pub struct RemoteClipboard {
    pub host: String,
    pub port: u16,
}

pub struct LocalClipboard {
    pub port: u16,
    pub password: String,
    pub peer: RemoteClipboard,
}

pub const UNICLIP_MAGIC: u16 = ('U' as u16) << 8 | 'C' as u16;
pub const UNICLIP_PROTO_VERSION: u8 = 1;

use serde::{Deserialize, Serialize};
use serde_encrypt::{serialize::impls::BincodeSerializer, traits::SerdeEncryptSharedKey};

#[derive(Debug, Serialize, Deserialize)]
pub enum UniclipCommand {
    Echo,
    EchoRes,

    Peer,
    PeerRes,

    Connect,
    ConnectRes,

    Update,
    UpdateContinue,
    UpdateRes,

    Quit,
    QuitRes,

    ShutDown,
    Error,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum UniclipPayload {
    Echo(u32),    // A
    EchoRes(u32), // A + 1

    Peer(u32),                          // A
    PeerRes(u32, Vec<RemoteClipboard>), // A + number of peers

    Connect(u32, RemoteClipboard), // A, local ip:prot
    ConnectRes(u32),               // A + 1

    Update(String, u32),        // data hash, data length
    UpdateContinue(String, u32, Vec<u8>), // data hash, current length, data
    UpdateRes(u32),               // received data length

    Quit(u32),    // A
    QuitRes(u32), // A + 1

    ShutDown(u32), // MAGIC "DOWN"
    Error(String), // error message
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UniclipDataFrame {
    pub magic: u16,
    pub version: u8,
    pub command: UniclipCommand,
    pub payload: UniclipPayload,
}

impl SerdeEncryptSharedKey for UniclipDataFrame {
    type S = BincodeSerializer<Self>;
}

impl RemoteClipboard {
    pub fn clone(&self) -> RemoteClipboard {
        RemoteClipboard {
            host: self.host.clone(),
            port: self.port,
        }
    }
}

impl UniclipCommand {
    pub fn clone(&self) -> UniclipCommand {
        match self {
            UniclipCommand::Echo => UniclipCommand::Echo,
            UniclipCommand::EchoRes => UniclipCommand::EchoRes,
            UniclipCommand::Peer => UniclipCommand::Peer,
            UniclipCommand::PeerRes => UniclipCommand::PeerRes,
            UniclipCommand::Connect => UniclipCommand::Connect,
            UniclipCommand::ConnectRes => UniclipCommand::ConnectRes,
            UniclipCommand::Update => UniclipCommand::Update,
            UniclipCommand::UpdateContinue => UniclipCommand::UpdateContinue,
            UniclipCommand::UpdateRes => UniclipCommand::UpdateRes,
            UniclipCommand::Quit => UniclipCommand::Quit,
            UniclipCommand::QuitRes => UniclipCommand::QuitRes,
            UniclipCommand::ShutDown => UniclipCommand::ShutDown,
            UniclipCommand::Error => UniclipCommand::Error,
        }
    }
}

impl UniclipPayload {
    pub fn clone(&self) -> UniclipPayload {
        match self {
            UniclipPayload::Echo(a) => UniclipPayload::Echo(*a),
            UniclipPayload::EchoRes(a) => UniclipPayload::EchoRes(*a),
            UniclipPayload::Peer(a) => UniclipPayload::Peer(*a),
            UniclipPayload::PeerRes(a, b) => {
                let mut new_b = Vec::new();
                for v in b.iter(){
                    new_b.push(v.clone());
                }
                UniclipPayload::PeerRes(*a, new_b)
            },
            UniclipPayload::Connect(a, b) => UniclipPayload::Connect(*a, b.clone()),
            UniclipPayload::ConnectRes(a) => UniclipPayload::ConnectRes(*a),
            UniclipPayload::Update(a, b) => UniclipPayload::Update(a.clone(), *b),
            UniclipPayload::UpdateContinue(a, b, c) => {
                UniclipPayload::UpdateContinue(a.clone(), *b, c.clone())
            }
            UniclipPayload::UpdateRes(a) => UniclipPayload::UpdateRes(*a),
            UniclipPayload::Quit(a) => UniclipPayload::Quit(*a),
            UniclipPayload::QuitRes(a) => UniclipPayload::QuitRes(*a),
            UniclipPayload::ShutDown(a) => UniclipPayload::ShutDown(*a),
            UniclipPayload::Error(a) => UniclipPayload::Error(a.clone()),
        }
    }
}

impl UniclipDataFrame{
    pub fn clone(&self) -> UniclipDataFrame {
        UniclipDataFrame {
            magic: self.magic,
            version: self.version,
            command: self.command.clone(),
            payload: self.payload.clone(),
        }
    }
}
