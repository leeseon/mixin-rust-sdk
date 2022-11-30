use mixin_sdk::{keystore::Keystore,Client};

fn main() {
    // let key 
    let ks = Keystore::new(Keystore {
        client_id: String::from(""),
        client_secret: String::from(""),
        session_id: String::from(""),
        private_key: String::from(""),
        pin_token: String::from(""),
        pin: String::from(""),
        scope: String::from("FULL"),
    });
    let client = Client::new(ks);
    println!("{:?}", client);
}
