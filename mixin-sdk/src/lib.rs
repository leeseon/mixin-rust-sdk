pub mod keystore;
use keystore::KeyStore;

#[derive(Debug)]
pub struct Client {
    keystore: KeyStore,
}

impl Client {
    pub fn new(ks: KeyStore) -> Client {
        Client {keystore: ks}
    }
}

#[cfg(test)]
mod tests {
    use crate::KeyStore;
    #[test]   
    fn keystore_deser() {
        let the_file = r#"{
            "pin": "445761",
            "client_id": "8aa33da5-d15a-4f51-88af-ab6110919b94",
            "session_id": "5111e413-6469-4baf-9dfa-c39f72f8e9e7",
            "pin_token": "9_z-8RVu4JrKxQNXgclp1Xg4FZ3ms9N9rAdVjgZbflg",
            "private_key": "3gmunU15oc6zlXKAtx879pOXuG8RKWQQ16H5wbTDZ_QbGP0669QCCENkxK9MfL3PjQKbyYUxJ4WvemHhP8LTiw"
           }"#;
    
        let ks: KeyStore =
            serde_json::from_str(the_file).expect("JSON was not well-formatted");
        } 
}