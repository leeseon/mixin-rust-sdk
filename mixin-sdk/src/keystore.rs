// use std::fs::File;
use std::path::Path;
// use std::io::Read;
use std::fs::read_to_string;

// use serde::{Serialize, Deserialize};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct KeyStore {
    pub client_id: String,
    pub client_secret: Option<String>,
    pub session_id: String,
    pub private_key: String,
    pub pin_token: String,
    pub scope: Option<String>,
    pub pin: u32,
  }
  
impl KeyStore {
    pub fn new(key :KeyStore) -> KeyStore {
        key
    }

    pub fn from_file(filename :&Path) -> KeyStore {
        let file_content = read_to_string(filename).expect("error reading file");
        let ks: KeyStore =serde_json::from_str(&file_content).expect("JSON was not well-formatted");
        ks
    }
}