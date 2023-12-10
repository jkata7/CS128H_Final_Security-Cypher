pub mod reflector {
    extern crate lazy_static;
    use lazy_static::lazy_static;
    use std::collections::HashMap;
    lazy_static! {
        static ref REFLECTOR_MAPPING: HashMap<char, char> = {
            let key_chars = "ABCDEFGHIJKLMNOPQRSTUVWXYZ".to_ascii_lowercase();
            let value_chars = "EJMZALYXVBWFCRQUONTSPIKHGD".to_ascii_lowercase();

            let mut char_map = HashMap::new();

            for (key, value) in key_chars.chars().zip(value_chars.chars()) {
                char_map.insert(key, value);
            }

            char_map
        };
    }

    // encryption and decryption for the reflector
    // is symmetric, so separate functions for encryption and decryption are not needed

    pub fn reflector(input: &char) -> char {
        let output = REFLECTOR_MAPPING.get(input).unwrap_or_else(|| {
            return input;
        });
        return *output;
    }
}