pub mod keystore;
use keystore::Keystore;

#[derive(Debug)]
pub struct Client {
    keystore: Keystore,
}

impl Client {
    pub fn new(ks: Keystore) -> Client {
        Client {keystore: ks}
    }
}