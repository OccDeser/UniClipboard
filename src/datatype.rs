use serde::{Deserialize, Serialize};
use serde_encrypt::{serialize::impls::BincodeSerializer, traits::SerdeEncryptSharedKey};

#[derive(Debug, Serialize, Deserialize, Clone)]
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

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum UniclipBig {
    Text,
    Picture,
    File,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum UniclipPayload {
    Echo(u32),      // random number A
    EchoRes(u32),   // A + 1

    Peer(u32),                          // random number A
    PeerRes(u32, Vec<RemoteClipboard>), // A + 1, peers

    Connect(u32, u16),      // random number A, local prot
    ConnectRes(u32),        // A + 1

    Update(String, Vec<u8>),        // data hash, data
    UpdateRes(u32),                 // received data length

    UpdateBig(String, UniclipBig, u32),     // data hash, data type, data frame size
    UpdateBigAck(String, u32),              // data hash, data frame size
    UpdateBigData(Vec<u8>),                 // data
    UpdateBigFinish(u32),                   // data length

    Quit(u32),    // A
    QuitRes(u32), // A + 1

    ShutDown,      // no payload
    Error(String), // error message
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
