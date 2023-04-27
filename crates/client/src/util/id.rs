use rand::{self, RngCore};

const RANDOM_ID_ENTROPY_BYTES: usize = 17;
const RANDOM_ID_BASE: u32 = 36;
const MAX_RANDOM_ID_LENGTH: usize = 25;

// Generate a random ID, based on <https://github.com/moby/buildkit/blob/52c2fe5ab15da7cdbdff799d96ed85a88761df33/identity/randomid.go>
pub fn random_id() -> String {
    let mut p: [u8; RANDOM_ID_ENTROPY_BYTES] = Default::default();
    rand::thread_rng().fill_bytes(&mut p);
    p[0] |= 0x80; // set high bit to avoid the need for padding
    num::BigInt::from_bytes_be(num::bigint::Sign::Plus, &p[..]).to_str_radix(RANDOM_ID_BASE)
        [1..MAX_RANDOM_ID_LENGTH + 1]
        .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_random_id() {
        let id = random_id();
        assert_eq!(id.len(), MAX_RANDOM_ID_LENGTH);
    }

    #[test]
    #[ignore]
    fn test_random_id_print() {
        for _ in 0..100 {
            println!("{}", random_id());
        }
    }
}
