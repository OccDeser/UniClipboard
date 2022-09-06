use super::super::datatype::{
    UniclipDataFrame, UniclipPayload, UNICLIP_MAGIC, UNICLIP_PROTO_VERSION,
};
use super::message;
use hex::decode;
use serde_encrypt::{shared_key::SharedKey, traits::SerdeEncryptSharedKey, EncryptedMessage};
use sha256::digest;

pub fn pwd2key(password: String) -> SharedKey {
    let val = digest(password);
    let val = decode(val).unwrap();

    let mut hash = [0u8; 32];
    for i in 0..32 {
        hash[i] = val[i];
    }
    SharedKey::new(hash)
}

pub fn pack(data: UniclipPayload, key: &SharedKey) -> Vec<u8> {
    let data_frame = UniclipDataFrame{
        magic: UNICLIP_MAGIC,
        version: UNICLIP_PROTO_VERSION,
        payload: data,
    };
    let encrypted_data = data_frame.encrypt(key).unwrap();
    let serialized_data = encrypted_data.serialize();
    serialized_data
}

pub fn unpack(data: Vec<u8>, key: &SharedKey) -> UniclipPayload {
    let encrypted_data = EncryptedMessage::deserialize(data).unwrap();
    let data_frame = UniclipDataFrame::decrypt_owned(&encrypted_data, key);
    match data_frame {
        Ok(data) => {
            if data.magic != UNICLIP_MAGIC {
                message::error(String::from("UNPACK ERROR, Invalid magic number"));
                return UniclipPayload::Error(String::from("Invalid magic number"));
            } else if data.version != UNICLIP_PROTO_VERSION {
                message::error(String::from("UNPACK ERROR, Invalid protocol version"));
                return UniclipPayload::Error(String::from("Invalid protocol version"));
            } else {
                return data.payload;
            }
        }
        Err(_) => {
            message::error(String::from("UNPACK ERROR, Unable to decrypt data frame"));
            return UniclipPayload::Error(String::from("Unable to decrypt data frame"));
        }
    }
}
