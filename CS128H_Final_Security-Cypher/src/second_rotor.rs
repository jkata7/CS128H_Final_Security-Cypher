pub mod second_rotor {
    extern crate lazy_static;
    use lazy_static::lazy_static;
    use std::collections::HashMap;

    lazy_static! {
        static ref FORWARD_MAPPING_SECOND_ROTOR: HashMap<char, char> = {
            let key_chars = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";
            let value_chars = "AJDKSIRUXBLHWTMCQGZNPYFVOE";

            let mut char_map = HashMap::new();

            for (key, value) in key_chars.chars().zip(value_chars.chars()) {
                char_map.insert(key, value);
            }

            char_map
        };
    }

    lazy_static! {
        static ref BACKWARD_MAPPING_SECOND_ROTOR: HashMap<char, char> = {
            let key_chars = "AJDKSIRUXBLHWTMCQGZNPYFVOE";
            let value_chars = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";

            let mut char_map = HashMap::new();

            for (key, value) in key_chars.chars().zip(value_chars.chars()) {
                char_map.insert(key, value);
            }

            char_map
        };
    }

    // the letter is encoded both forward and backwards through the rotor because it is "reflected" in the reflector
    // effectively sending it the other way through the rotors

    pub fn forward_mapping_second_rotor(input: char) -> char {
        let output = FORWARD_MAPPING_SECOND_ROTOR.get(&input);
        return output;
    }

    pub fn backward_mapping_second_rotor(input: char) -> char {
        let output = BACKWARD_MAPPING_SECOND_ROTOR.get(&input);
        return output;
    }
}