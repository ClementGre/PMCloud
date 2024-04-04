use rand::RngCore;
use rand::rngs::OsRng;

pub fn random_token(bytes: usize) -> Vec<u8> {
    let mut auth_token = vec![0u8; bytes];
    OsRng.fill_bytes(&mut auth_token);
    return auth_token;
}

pub fn random_code(digits: u32) -> u32 {
    return OsRng.next_u32() % (10 * digits);
}
