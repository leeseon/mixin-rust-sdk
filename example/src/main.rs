#![warn(unused_imports)]
use mixin_sdk::{keystore::KeyStore,Client};
// use serde::{Serialize, Deserialize};
use serde_json;

fn main() {
    let the_file = r#"{
        "pin": 123456,
        "client_id": "8aa33da5-aaaa-4f51-88af-ab6110919b94",
        "session_id": "5111e413-aaaa-4baf-9dfa-c39f72f8e9e7",
        "pin_token": "9_z-8RVu4JrKxQNXgclp2Xg4FZ3ms9N9rAdVjgZbflg",
        "private_key": "3gmunU15oc8zlXKAtx879pOXuG8RKWQQ16H5wbTDZ_QbGP0669QCCENkxK9MfL3PjQKbyYUxJ4WvemHhP8LTiw"
       }"#;

    let ks: KeyStore =
        serde_json::from_str(the_file).expect("JSON was not well-formatted");

    println!("{:?}", ks);
}
