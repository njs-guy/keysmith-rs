pub mod key;
pub mod timestamp;
pub mod uuid;
pub mod gen_char;

// TODO: More tests, probably

#[cfg(test)]
mod tests {
    use super::*;
    
    // Tests that the generated key is the correct length
    #[test]
    fn test_length() {
        let length = 32;
        let key = key::gen_key(length);

        assert_eq!(key.len(), 32);
    }
}
