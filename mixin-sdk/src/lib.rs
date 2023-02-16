pub mod keystore;
pub mod http;
pub mod authorization;

use reqwest::Method;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::{error, fmt};

#[derive(Debug)]
pub struct Client {
    keystore: keystore::KeyStore,
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

#[derive(Debug, Serialize, Deserialize)]
pub struct MixinHttpError {
    pub status: u32,
    pub code: u32,

    #[serde(default)]
    pub description: String,
    #[serde(default)]
    pub extra: String,
}

impl fmt::Display for MixinHttpError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "status: {}, code: {}, description: {}, extra: {}",
            self.status, self.code, self.description, self.extra
        )
    }
}

impl error::Error for MixinHttpError {}

impl Client {
    pub fn new(ks: keystore::KeyStore) -> Client {
        Client {keystore: ks}
    }

    pub fn me(self) -> Result<Me, Box<dyn error::Error>> {
        let map: HashMap<String, String> = HashMap::new();
        let res = http::request(self.keystore, Method::GET, "/me", &map)?;
    
        #[derive(Debug, Serialize, Deserialize)]
        struct Body {
            data: Option<Me>,
            error: Option<MixinHttpError>,
        }
    
        let body: Body = res.json().unwrap();
    
        match body.error {
            Some(e) => Err(Box::new(e)),
            None => Ok(body.data.unwrap()),
        }        
    }

    pub fn req(self, method: Method, url: &str) -> Result<String, Box<dyn error::Error>> {
        let map: HashMap<String, String> = HashMap::new();
        let res = http::request(self.keystore, method, url, &map)?;
    
        #[derive(Debug, Serialize, Deserialize)]
        struct Body {
            data: Option<String>,
            error: Option<MixinHttpError>,
        }
    
        let body: Body = res.json().unwrap();
    
        match body.error {
            Some(e) => Err(Box::new(e)),
            None => Ok(body.data.unwrap()),
        }        
    }
}

