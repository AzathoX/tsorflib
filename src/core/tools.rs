use base64::{DecodeError, Engine};
use base64::engine::general_purpose::STANDARD;
use x25519_dalek::{PublicKey, StaticSecret};

pub fn load_string_byte_32(
    private_key: &str,
) -> Vec<u8> {
     STANDARD.decode(private_key.trim()).unwrap()
}


pub fn load_string_byte_32_not_unwrap(
    private_key: &str,
) -> Result<Vec<u8>, DecodeError> {
    STANDARD.decode(private_key.trim())
}


pub fn load_private_key(
    str: &str,
) -> Result<StaticSecret, Box<dyn std::error::Error>> {
    let bytes: [u8; 32] = load_string_byte_32(str).try_into().unwrap();

    Ok(StaticSecret::from(bytes))
}

pub fn load_public_key(
    str: &str,
) -> Result<PublicKey, Box<dyn std::error::Error>> {

    let bytes: [u8; 32] = load_string_byte_32(str).try_into().unwrap();

    Ok(PublicKey::from(bytes))
}