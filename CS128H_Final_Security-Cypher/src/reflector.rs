pub mod reflector {
    lazy_static! {
        static ref REFLECTOR_MAPPING: HashMap<char, char> = {
            let key_chars = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";
            let value_chars = "EJMZALYXVBWFCRQUONTSPIKHGD";

            let mut char_map = HashMap::new();

            for (key, value) in key_chars.chars().zip(value_chars.chars()) {
                char_map.insert(key, value);
            }

            char_map
        };
    }

    // encryption and decryption for the reflector
    // is symmetric, so separate functions for encryption and decryption are not needed

    pub fn reflector(input: char) -> char {
        let output = REFLECTOR_MAPPING.get(&input);
        return output;
    }
}