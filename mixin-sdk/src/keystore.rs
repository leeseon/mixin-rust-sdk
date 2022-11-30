#[derive(Debug)]
pub struct Keystore {
    pub client_id: String,
    pub client_secret: String,
    pub session_id: String,
    pub private_key: String,
    pub pin_token: String,
    pub scope: String,
    pub pin: String,
  }
  
impl Keystore {
    pub fn new(key :Keystore) -> Keystore {
        key
    }
}