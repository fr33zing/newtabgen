use sha1::{Digest, Sha1};

pub fn sha1_base32(bytes: &[u8]) -> String {
    let mut hasher = Sha1::new();
    hasher.update(bytes);
    let hash = hasher.finalize();
    let hash_base32 = data_encoding::BASE32HEX_NOPAD.encode(&hash);
    hash_base32.to_lowercase()[..8].into()
}
