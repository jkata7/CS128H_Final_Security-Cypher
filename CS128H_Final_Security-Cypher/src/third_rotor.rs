pub mod third_rotor {
    extern crate lazy_static;
    use lazy_static::lazy_static;
    use std::collections::HashMap;

    lazy_static! {
        static ref FORWARD_MAPPING_THIRD_ROTOR: HashMap<char, char> = {
            let key_chars = "ABCDEFGHIJKLMNOPQRSTUVWXYZ".to_ascii_lowercase();
            let value_chars = "BDFHJLCPRTXVZNYEIWGAKMUSQO".to_ascii_lowercase();

            let mut char_map = HashMap::new();

            for (key, value) in key_chars.chars().zip(value_chars.chars()) {
                char_map.insert(key, value);
            }

            return char_map;
        };
    }

    lazy_static! {
        static ref BACKWARD_MAPPING_THIRD_ROTOR: HashMap<char, char> = {
            let key_chars = "BDFHJLCPRTXVZNYEIWGAKMUSQO".to_ascii_lowercase();
            let value_chars = "ABCDEFGHIJKLMNOPQRSTUVWXYZ".to_ascii_lowercase();

            let mut char_map = HashMap::new();

            for (key, value) in key_chars.chars().zip(value_chars.chars()) {
                char_map.insert(key, value);
            }

            return char_map;
        };
    }

    // the letter is encoded both forward and backwards through the rotor because it is "reflected" in the reflector
    // effectively sending it the other way through the rotors

    pub fn forward_mapping_third_rotor(input: &char) -> char {
        let output = FORWARD_MAPPING_THIRD_ROTOR.get(input).unwrap_or_else(|| {
            return input;
        });
        return *output;
    }

    pub fn backward_mapping_third_rotor(input: &char) -> char {
        let output = BACKWARD_MAPPING_THIRD_ROTOR.get(input).unwrap_or_else(|| {
            return input;
        });
        return *output;
    }
}