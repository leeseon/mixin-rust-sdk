use serde::{Serialize, Deserialize};
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
}