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

