
extern crate fnv;
extern crate serde;
extern crate serde_json;

use fnv::FnvHasher;
use std::collections::{HashMap as StdHashMap, HashSet as StdHashSet};
use std::hash::BuildHasherDefault;
use std::hash::Hash;
use std::collections::hash_map::Entry::*;
use std::io;


pub type HashMap<K, V> = StdHashMap<K, V, BuildHasherDefault<FnvHasher>>;
pub type HashSet<V> = StdHashSet<V, BuildHasherDefault<FnvHasher>>;

// let bullshit = vec![1,3,2,5,6,1,23,5,6,72];
// let as_a_map = group_by(bullshit, |e| e % 2);
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

use std::path::Path;
use std::fs::File;
use std::io::Write;
use std::io::Read;

#[derive(Debug)]
pub enum AphidError {
	IO(io::Error),
	SerdeJson(serde_json::Error),
}


impl From<io::Error> for AphidError {
    fn from(err: io::Error) -> Self {
        AphidError::IO(err)
    }
}

impl From<serde_json::Error> for AphidError {
    fn from(err: serde_json::Error) -> Self {
        AphidError::SerdeJson(err)
    }
}


pub type AphidResult<T> = Result<T, AphidError>;

pub fn serialize_to_json_file<T, P : AsRef<Path>>(ele: &T, path: P) -> AphidResult<()> where T : serde::Serialize {
	let serialized = serde_json::to_string(ele)?;
	let serialized_bytes = serialized.into_bytes();

	let mut f = File::create(path)?;
	f.write_all(&serialized_bytes)?;
	f.sync_all().map_err(AphidError::IO)?;

	Ok(())
}

pub fn deserialize_from_json_file<T, P : AsRef<Path>>(path: P) -> AphidResult<T> where T : serde::Deserialize {
	let mut f = File::open(path)?;
	let mut str_buffer = String::new();
	f.read_to_string(&mut str_buffer)?;
	serde_json::from_str(&str_buffer).map_err(AphidError::SerdeJson)
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