// maths
// author: Mia Stein

use hex;
use ring::hmac;


pub fn sign(secret: &str, msg: &str) -> String {
    
    let key = hmac::Key::new(hmac::HMAC_SHA256, secret.as_bytes());
    let tag = hmac::sign(&key, msg.as_bytes());
    hex::encode(tag.as_ref())

}

