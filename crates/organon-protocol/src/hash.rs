use sha2::{Digest, Sha256};

/// Lowercase hex SHA-256 of `data`.
#[must_use]
pub fn sha256_hex(data: &[u8]) -> String {
    use std::fmt::Write as _;
    Sha256::digest(data)
        .iter()
        .fold(String::with_capacity(64), |mut s, b| {
            let _ = write!(s, "{b:02x}");
            s
        })
}

/// `prefix` followed by the first 16 hex digits of the SHA-256 of `data`.
#[must_use]
pub fn short_hash(prefix: &str, data: &[u8]) -> String {
    let hex = sha256_hex(data);
    format!("{prefix}{}", &hex[..16])
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn known_digest() {
        assert_eq!(
            sha256_hex(b"abc"),
            "ba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015ad"
        );
        assert_eq!(short_hash("s_", b"abc"), "s_ba7816bf8f01cfea");
    }
}
