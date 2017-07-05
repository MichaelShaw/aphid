
extern crate fnv;
extern crate serde;
extern crate serde_json;
extern crate bincode;
extern crate bytes;

pub mod codec;
pub mod resource;

use std::hash::Hash;
use std::collections::hash_map::Entry::*;
use std::io;
use std::io::Write;
use std::io::Read;
use std::path::Path;
use std::fs::File;

pub type Seconds = f64;

pub type Milliseconds = u64;

pub type Nanoseconds = u64;

use codec::{SerializeCodec, DeserializeCodec};

pub type HashMap<K, V> = fnv::FnvHashMap<K, V>;
pub type HashSet<K> = fnv::FnvHashSet<K>;

pub fn group_by<T, K, F>(items: Vec<T>, f: F) -> HashMap<K, Vec<T>> where F : Fn(&T) -> K, K : Eq + Hash {
    let mut map : HashMap<K, Vec<T>> = HashMap::default();

    for item in items.into_iter() {
        let k = f(&item);
        match map.entry(k) {
            Occupied(mut cl) => {
                (cl.get_mut()).push(item);
            },
            Vacant(ve) => { 
                ve.insert(vec![item]);
            },
        }
    }

    map
}

#[derive(Debug)]
pub enum AphidError {
    IO(io::Error),
    CodecError(codec::CodecError),
}


impl From<io::Error> for AphidError {
    fn from(err: io::Error) -> Self {
        AphidError::IO(err)
    }
}

impl From<codec::CodecError> for AphidError {
    fn from(err: codec::CodecError) -> Self {
        AphidError::CodecError(err)
    }
}


pub type AphidResult<T> = Result<T, AphidError>;



pub fn serialize_to_json_file<T, P : AsRef<Path>, C>(ele: &T, path: P) -> AphidResult<()> where C : SerializeCodec<T>, T : serde::Serialize {
    let mut bytes = Vec::new();

    C::serialize(ele, &mut bytes)?;

    let mut f = File::create(path)?;
    f.write_all(&bytes)?;
    f.sync_all().map_err(AphidError::IO)?;

    Ok(())
}

pub fn deserialize_from_json_file<T, P : AsRef<Path>, C>(path: P) -> AphidResult<T> where C : DeserializeCodec<T>, T : serde::de::DeserializeOwned {
    let mut f = File::open(path)?;

    let mut bytes = Vec::new();
    f.read_to_end(&mut bytes)?;
    
    C::deserialize(&bytes).map_err(AphidError::CodecError)
}

pub fn contains<T, F>(opt: Option<T>, f: F) -> bool where F: Fn(&T) -> bool {
    opt.iter().any(f)
}

#[macro_export]
macro_rules! hashset {
    ($($val: expr ),*) => {{
         let mut set = HashSet::default();
         $( set.insert( $val); )*
         set
    }}
}

#[macro_export]
macro_rules! hashmap {
    ($( $key: expr => $val: expr ),*) => {{
         let mut map = HashMap::default();
         $( map.insert($key, $val); )*
         map
    }}
}