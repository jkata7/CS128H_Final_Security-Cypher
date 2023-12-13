#[path = "./plugboard.rs"]
mod plugboard;
#[path = "./first_rotor.rs"]
mod first_rotor;
#[path = "./second_rotor.rs"]
mod second_rotor;
#[path = "./third_rotor.rs"]
mod third_rotor;
#[path = "./reflector.rs"]
mod reflector;

pub mod enigma {
    use super::{plugboard, first_rotor, second_rotor, third_rotor, reflector};

    pub fn enigma(plugboard_config: &Vec<(char, char)>, rotor_1_config: &i32, rotor_2_config: &i32, rotor_3_config: &i32, to_encrypt: &String) -> String {
        let mut to_return: String = "".to_string();
        let input_string_copy = to_encrypt.to_ascii_lowercase();
        let mut rotor_1_config_copy = rotor_1_config.clone();
        let mut rotor_2_config_copy = rotor_2_config.clone();
        let mut rotor_3_config_copy = rotor_3_config.clone();
        //   a b c
        // a b c
        // a b c
//     a b c
//     a b c
    //   a b c
    //   a b c
        // a b c

        for mut character in input_string_copy.chars() {
            println!("Encryption sequence for {}", character);
            if character.is_ascii_lowercase() {
                character = plugboard::plugboard::plugboard(plugboard_config, &character);
                println!("{}", character);
                character = shift_character(&character, &rotor_1_config_copy);
                println!("{}", character);
                character = first_rotor::first_rotor::forward_mapping_first_rotor(&character);
                println!("{}", character);
                character = shift_character_backwards(&character, &rotor_1_config_copy);
                println!("{}", character);
                character = shift_character(&character, &rotor_2_config_copy);
                println!("{}, {}", character, rotor_2_config_copy);
                character = second_rotor::second_rotor::forward_mapping_second_rotor(&character);
                println!("{}", character);
                character = shift_character_backwards(&character, &rotor_2_config_copy);
                println!("{}", character);
                character = shift_character(&character, &rotor_3_config_copy);
                println!("{}", character);
                character = third_rotor::third_rotor::forward_mapping_third_rotor(&character);
                println!("{}", character);
                character = shift_character_backwards(&character, &rotor_3_config_copy);
                println!("{}", character);
                character = reflector::reflector::reflector(&character);
                println!("{}", character);
                character = shift_character(&character, &rotor_3_config_copy);
                println!("{}", character);
                character = third_rotor::third_rotor::backward_mapping_third_rotor(&character);
                println!("{}", character);
                character = shift_character_backwards(&character, &rotor_3_config_copy);
                println!("{}", character);
                character = shift_character(&character, &rotor_2_config_copy);
                println!("{}", character);
                character = second_rotor::second_rotor::backward_mapping_second_rotor(&character);
                println!("{}", character);
                character = shift_character_backwards(&character, &rotor_2_config_copy);
                println!("{}", character);
                character = shift_character(&character, &rotor_1_config_copy);
                println!("{}", character);
                character = first_rotor::first_rotor::backward_mapping_first_rotor(&character);
                println!("{}", character);
                character = shift_character_backwards(&character, &rotor_1_config_copy);
                println!("{}", character);
                character = plugboard::plugboard::plugboard(plugboard_config, &character);
                println!("{}", character);

                rotor_1_config_copy += 1;

                if rotor_1_config_copy == 27 {
                    rotor_1_config_copy = 1;
                    rotor_2_config_copy += 1;
                    if rotor_2_config_copy == 27 {
                        rotor_2_config_copy = 1;
                        rotor_3_config_copy += 1;
                        if rotor_3_config_copy == 27 {
                            rotor_3_config_copy = 1;
                        }
                    }
                }
            }
            to_return.push(character);
        }

        return to_return;
    }

    pub fn shift_character(character: &char, shift: &i32) -> char {
        let new_position = (((char_to_position(character) - 1) + (shift - 1)) % 26) + 1;
        return position_to_char(&new_position);
    }

    pub fn shift_character_backwards(character: &char, shift: &i32) -> char {
        let new_position = (((char_to_position(character) - 1) - (shift - 1) + 26) % 26) + 1;
        return position_to_char(&new_position);
    }
    fn char_to_position(character: &char) -> i32 {
        return (*character as u8 - b'a' + 1) as i32;
    }
    
    
    fn position_to_char(position: &i32) -> char {
        return (*position as u8 + b'a' - 1) as char;
    }
    
}