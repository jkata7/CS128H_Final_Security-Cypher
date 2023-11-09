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
}