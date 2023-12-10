pub mod plugboard {
    pub fn plugboard(configuration: &Vec<(char, char)>, letter: &char) -> char{
        for (letter_1, letter_2) in configuration {
            if letter == letter_1 {
                return *letter_2;
            } else if letter == letter_2 {
                return *letter_1
            }
        }

        return *letter;
    }
}