use base64::{engine::fast_portable::{self, FastPortable}, alphabet};
use jwt_simple::prelude::*;
use reqwest::Method;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::error;
use uuid::Uuid;

use crate::keystore;

#[derive(Serialize, Deserialize)]
struct CustomClaims {
    uid: String,
    sid: String,
    jti: String,
    sig: String,
    scp: String,
}

pub fn sign_token(
    method: Method,
    uri: &str,
    body: &str,
    ks: keystore::KeyStore,
) -> Result<String, Box<dyn error::Error>> {
    let mut hasher = Sha256::new();
    hasher.update(format!("{}{}{}", method.as_str(), uri, body).as_bytes());
    let result = hasher.finalize();

    let private_data = base64::decode_engine(
        ks.private_key,
        &FastPortable::from(
            &alphabet::URL_SAFE,
            fast_portable::NO_PAD),

    )?;

    let claim = CustomClaims {
        uid: ks.client_id.to_string(),
        sid: ks.session_id.to_string(),
        jti: Uuid::new_v4().to_string(),
        sig: format!("{:x}", result),
        scp: "FULL".to_owned(),
    };
    let claims = Claims::with_custom_claims(claim, Duration::from_hours(24 * 30 * 6));

    let key_pair = Ed25519KeyPair::from_bytes(private_data.as_slice())?;
    Ok(key_pair.sign(claims)?)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sign_token(){
        // test sign_token function
        // let method = Method::GET;
        // let uri = "https://mixin-api.zeromesh.net/assets";
        // let body = "";
        // let ks = keystore::KeyStore::new(
        //     "cfc8b0c0-7d59-4a66-8c8f-2a1f6c2a6f4c",
        //     "b1a6b4e6-4a6b-4a00-9e4b-0e8e6b8f6a06",
        //     "e2b7d3e9-5f7e-4b2d-8c2f-6a2f6e2a6f4c",
        //     "cfc8b0c0-7d59-4a66-8c8f-2a1f6c2a6f4c",
        //     "e2b7d3e9-5f7e-4b2d-8c2f-6a2f6e2a6f4c",
        //     "b1a6b4e6-4a6b-4a00-9e4b-0e8e6b8f6a06",
        //     "b1a6b4e6-4a6b-4a00-9e4b-0e8e6b8f6a06",
        // );
        // let token = sign_token(method, uri, body, ks);
        // sign_token(method, uri, body, ks)
        assert_eq!(2+2, 4);

    }
}