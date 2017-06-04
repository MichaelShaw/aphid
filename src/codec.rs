
use serde_json;
use bincode;


use serde::Serialize;
use serde::de::DeserializeOwned;

use std::str;


#[derive(Debug)]
pub enum CodecError {
    CouldntCreateString(str::Utf8Error),
    BinCodeError(bincode::Error), // Box<bincode::ErrorKind>
    JsonError(serde_json::error::Error),
}

pub trait SerializeCodec<E> where E: Serialize {
    fn serialize(e: &E, bytes: &mut Vec<u8>) -> Result<(), CodecError>;
}

pub trait DeserializeCodec<E> where E : DeserializeOwned {
    fn deserialize(bytes: &[u8]) -> Result<E, CodecError>;
}

pub trait AsymmetricCodec<IE, OE> where OE : Serialize, IE : DeserializeOwned { // for client <-> server use
    fn serialize_outgoing(oe: &OE, bytes: &mut Vec<u8>) -> Result<(), CodecError>;
    fn deserialize_incoming(bytes: &[u8]) -> Result<IE, CodecError>;
}

use std::io::Write;

pub struct JsonCodec;

pub fn serialize_json<E>(e: &E, bytes: &mut Vec<u8>) -> Result<(), CodecError> where E : Serialize {
    match serde_json::to_string(e) {
        Ok(string) => {
            bytes.write(string.as_bytes()).expect("this isn't fallible");
            Ok(())
        },
        Err(e) => {
            Err(CodecError::JsonError(e))
        }
    }
}

pub fn deserialize_json<E>(bytes: &[u8]) -> Result<E, CodecError> where E: DeserializeOwned {
    match str::from_utf8(bytes) {
        Ok(as_str) => {
            match serde_json::from_str::<E>(as_str) {
                Ok(incoming_event) => Ok(incoming_event),
                Err(e) => {
                    Err(CodecError::JsonError(e))
                },
            }
        },
        Err(e) => {
            Err(CodecError::CouldntCreateString(e))
        }
    }
}

impl<E> SerializeCodec<E> for JsonCodec where E: Serialize {
    fn serialize(e: &E, bytes: &mut Vec<u8>) -> Result<(), CodecError> {
        serialize_json(e, bytes)
    }
}

impl<E> DeserializeCodec<E> for JsonCodec where E : DeserializeOwned {
    fn deserialize(bytes: &[u8]) -> Result<E, CodecError> {
        deserialize_json(bytes)
    }
}

impl<IE, OE> AsymmetricCodec<IE, OE> for JsonCodec where OE : Serialize, IE : DeserializeOwned {
    fn serialize_outgoing(oe: &OE, bytes: &mut Vec<u8>) -> Result<(), CodecError> {
        JsonCodec::serialize(oe, bytes)
    }

    fn deserialize_incoming(bytes: &[u8]) -> Result<IE, CodecError> {
        JsonCodec::deserialize(bytes)
    }
}


pub struct BincodeCodec;

pub fn serialize_bincode<E>(e: &E, bytes: &mut Vec<u8>) -> Result<(), CodecError> where E : Serialize {
    match bincode::serialize(e, bincode::Infinite) {
        Ok(bb) => {
            bytes.write(&bb).expect("byte writing worked"); // i didn't relize
            Ok(())
        }
        Err(e) => {
            Err(CodecError::BinCodeError(e))
        }
    }
}

pub fn deserialize_bincode<E>(bytes: &[u8]) -> Result<E, CodecError> where E : DeserializeOwned {
    match bincode::deserialize::<E>(bytes) {
        Ok(event) => Ok(event),
        Err(e) => Err(CodecError::BinCodeError(e)),
    }
}

impl<E> SerializeCodec<E> for BincodeCodec where E : Serialize {
    fn serialize(e: &E, bytes: &mut Vec<u8>) -> Result<(), CodecError> {
        serialize_bincode(e, bytes)
    }
}

impl<E> DeserializeCodec<E> for BincodeCodec where E : DeserializeOwned {
    fn deserialize(bytes: &[u8]) -> Result<E, CodecError> {
        deserialize_bincode(bytes)
    }
}

impl<IE, OE> AsymmetricCodec<IE, OE> for BincodeCodec where OE : Serialize, IE : DeserializeOwned {
    fn serialize_outgoing(oe: &OE, bytes: &mut Vec<u8>) -> Result<(), CodecError> {
        serialize_bincode(oe, bytes)
    }

    fn deserialize_incoming(bytes: &[u8]) -> Result<IE, CodecError> {
        deserialize_bincode(bytes)
    }
}
