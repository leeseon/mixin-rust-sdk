pub mod keystore;
pub mod http;
pub mod authorization;

use keystore::KeyStore;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::error;

#[derive(Debug)]
pub struct Client {
    keystore: KeyStore,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    user_id: String,
    identity_number: String,
    phone: String,
    full_name: String,
    biography: String,
    avatar_url: String,
    relationship: String,
    mute_until: String,
    created_at: String,
    is_verified: bool,
    is_scam: bool,

    #[serde(default)]
    #[serde(flatten)]
    _unknow_fields_: Option<HashMap<String, toml::Value>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Me {
    #[serde(flatten)]
    user: User,
    session_id: String,
    pin_token_base64: String,
    code_id: String,
    code_url: String,
    device_status: String,
    has_pin: bool,
    has_emergency_contact: bool,
    receive_message_source: String,
    accept_conversation_source: String,
    accept_search_source: String,
    fiat_currency: String,
    transfer_notification_threshold: f64,
    transfer_confirmation_threshold: f64,

    #[serde(default)]
    #[serde(flatten)]
    _unknow_fields_: Option<HashMap<String, toml::Value>>,
}

impl Client {
    pub fn new(ks: KeyStore) -> Client {
        Client {keystore: ks}
    }

    pub fn me() -> Result<Me, Box<dyn error::Error>> {
        let map: HashMap<String, String> = HashMap::new();
        let res = http::request(cfg, Method::GET, "/me", &map)?;
    
        #[derive(Debug, Serialize, Deserialize)]
        struct Body {
            data: Option<Me>,
            error: Option<http::Error>,
        }
    
        let body: Body = res.json().unwrap();
    
        match body.error {
            Some(e) => Err(Box::new(e)),
            None => Ok(body.data.unwrap()),
        }        
    }
}

