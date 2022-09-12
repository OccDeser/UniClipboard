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

pub mod payload_type {
    use lazy_static::lazy_static;

    lazy_static! {
        pub static ref ECHO: String = "Echo".to_string();
        pub static ref ECHO_RES: String = "EchoRes".to_string();
        pub static ref PEER: String = "Peer".to_string();
        pub static ref PEER_LIST: String = "PeerList".to_string();
        pub static ref PORT: String = "Port".to_string();
        pub static ref PORT_RES: String = "PortRes".to_string();
        pub static ref UPDATE: String = "Update".to_string();
        pub static ref UPDATE_RES: String = "UpdateRes".to_string();
        pub static ref UPDATE_BIG: String = "UpdateBig".to_string();
        pub static ref UPDATE_BIG_ACK: String = "UpdateBigAck".to_string();
        pub static ref UPDATE_BIG_DATA: String = "UpdateBigData".to_string();
        pub static ref UPDATE_BIG_FINISH: String = "UpdateBigFinish".to_string();
        pub static ref QUIT: String = "Quit".to_string();
        pub static ref QUIT_RES: String = "QuitRes".to_string();
        pub static ref SHUT_DOWN: String = "ShutDown".to_string();
        pub static ref ERROR: String = "Error".to_string();
    }
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
