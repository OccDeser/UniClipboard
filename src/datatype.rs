use serde::{Deserialize, Serialize};
use serde_encrypt::{serialize::impls::BincodeSerializer, traits::SerdeEncryptSharedKey};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RemoteClipboard {
    pub host: String,
    pub port: u16,
}

impl PartialEq for RemoteClipboard {
    fn eq(&self, other: &Self) -> bool {
        self.host == other.host && self.port == other.port
    }
}

pub struct LocalClipboard {
    pub port: u16,
    pub password: String,
    pub peer: RemoteClipboard,
}

pub const UNICLIP_MAGIC: u16 = ('U' as u16) << 8 | 'C' as u16;
pub const UNICLIP_PROTO_VERSION: u8 = 1;
pub const UNICLIP_DATA_LIMIT: usize = 5 * 1024;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum UniclipBig {
    Text,
    Picture,
    File,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum UniclipPayload {
    Echo(u32),    // random number A
    EchoRes(u32), // A + 1

    Peer(u32),                           // random number A
    PeerList(u32, Vec<RemoteClipboard>), // A + 1, peers

    Port(u32),         // random number A
    PortRes(u32, u16), // A + 1, port

    Update(String, String), // data hash, data
    UpdateRes(usize),       // received data length

    UpdateBig(String, UniclipBig, u32), // data hash, data type, data frame size
    UpdateBigAck(String, u32),          // data hash, data frame size
    UpdateBigData(Vec<u8>),             // data
    UpdateBigFinish(usize),             // data length

    Quit(u32),    // A
    QuitRes(u32), // A + 1

    ShutDown,      // no payload
    Error(String), // error message
}

pub mod PayloadType {
    pub const ECHO: &str = "Echo";
    pub const ECHO_RES: &str = "EchoRes";
    pub const PEER: &str = "Peer";
    pub const PEER_LIST: &str = "PeerList";
    pub const PORT: &str = "Port";
    pub const PORT_RES: &str = "PortRes";
    pub const UPDATE: &str = "Update";
    pub const UPDATE_RES: &str = "UpdateRes";
    pub const UPDATE_BIG: &str = "UpdateBig";
    pub const UPDATE_BIG_ACK: &str = "UpdateBigAck";
    pub const UPDATE_BIG_DATA: &str = "UpdateBigData";
    pub const UPDATE_BIG_FINISH: &str = "UpdateBigFinish";
    pub const QUIT: &str = "Quit";
    pub const QUIT_RES: &str = "QuitRes";
    pub const SHUT_DOWN: &str = "ShutDown";
    pub const ERROR: &str = "Error";
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UniclipDataFrame {
    pub magic: u16,
    pub version: u8,
    pub payload: UniclipPayload,
}

impl SerdeEncryptSharedKey for UniclipDataFrame {
    type S = BincodeSerializer<Self>;
}
