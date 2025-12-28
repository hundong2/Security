use sha2::Sha256;
use hmac::{Hmac, Mac, NewMac};

fn send_message(key: &[u8], message: &[u8]){
    let mut mac = hmac::<Sha256>::new(key.into());
    mac.update(message);
    mac.finalize().into_bytes().to_vec();
}
fn main() {
    println!("Hello, world!");
}
