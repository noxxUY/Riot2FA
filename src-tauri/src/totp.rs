use hmac::{Hmac, Mac};
use sha2::Sha256;

type HmacSha256 = Hmac<Sha256>;

pub const PERIOD: u64 = 30;
pub const DIGITS: u32 = 6;

pub fn totp_from_key(key: &[u8], unix_time: u64, period: u64, digits: u32) -> String {
    let counter = unix_time / period;
    let mut mac = HmacSha256::new_from_slice(key).expect("HMAC acepta claves de cualquier tamaño");
    mac.update(&counter.to_be_bytes());
    let h = mac.finalize().into_bytes();

    let offset = (h[h.len() - 1] & 0x0f) as usize;
    let bin = (((h[offset] & 0x7f) as u32) << 24)
        | ((h[offset + 1] as u32) << 16)
        | ((h[offset + 2] as u32) << 8)
        | (h[offset + 3] as u32);

    let code = bin % 10u32.pow(digits);
    format!("{:0width$}", code, width = digits as usize)
}

pub fn decode_seed(seed_b32: &str) -> Option<Vec<u8>> {
    let cleaned: String = seed_b32
        .chars()
        .filter(|c| !c.is_whitespace() && *c != '=')
        .collect();
    base32::decode(
        base32::Alphabet::Rfc4648 { padding: false },
        &cleaned.to_uppercase(),
    )
}

pub fn current_code(seed_b32: &str) -> Option<String> {
    let key = decode_seed(seed_b32)?;
    if key.is_empty() {
        return None;
    }
    Some(totp_from_key(&key, now_secs(), PERIOD, DIGITS))
}

pub fn seconds_remaining() -> u64 {
    PERIOD - (now_secs() % PERIOD)
}

fn now_secs() -> u64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.as_secs())
        .unwrap_or(0)
}

#[cfg(test)]
mod tests {
    use super::*;

    const SHA256_KEY: &[u8] = b"12345678901234567890123456789012";

    #[test]
    fn rfc6238_sha256_vectors() {
        assert_eq!(totp_from_key(SHA256_KEY, 59, 30, 8), "46119246");
        assert_eq!(totp_from_key(SHA256_KEY, 1_111_111_109, 30, 8), "68084774");
        assert_eq!(totp_from_key(SHA256_KEY, 1_111_111_111, 30, 8), "67062674");
        assert_eq!(totp_from_key(SHA256_KEY, 1_234_567_890, 30, 8), "91819424");
        assert_eq!(totp_from_key(SHA256_KEY, 2_000_000_000, 30, 8), "90698825");
        assert_eq!(totp_from_key(SHA256_KEY, 20_000_000_000, 30, 8), "77737706");
    }

    #[test]
    fn six_digits_is_mod_1e6() {

        assert_eq!(totp_from_key(SHA256_KEY, 59, 30, 6), "119246");
    }

    #[test]
    fn decode_seed_then_generate() {
        let seed = base32::encode(
            base32::Alphabet::Rfc4648 { padding: false },
            SHA256_KEY,
        );
        let key = decode_seed(&seed).expect("decodifica");
        assert_eq!(key, SHA256_KEY);
        assert_eq!(totp_from_key(&key, 59, 30, 8), "46119246");
    }

    #[test]
    fn decode_tolerates_spaces_and_lowercase() {

        assert_eq!(
            decode_seed("JBSW Y3DP"),
            decode_seed("jbswy3dp"),
        );
        assert!(decode_seed("JBSWY3DP").is_some());
    }
}
