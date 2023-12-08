use druid::{Data, widget::{Flex, TextBox, Button, Label}, Env, Widget, WindowDesc, AppLauncher};


#[derive(Clone)]
struct AppState {
    plugs_input: String,
    rotors: (u16, u16, u16),
    plugs: Vec<(char, char)>,
    valid_input: Option<String>,
}
impl AppState {
    fn update_character_pairs (&mut self) {
        if self.plugs_input.len() == 20 {
            self.plugs.clear();

            for chunk in self.plugs_input.chars().collect::<Vec<_>>().chunks_exact(2) {
                match chunk {
                    [c1, c2] => self.plugs.push((*c1, *c2)),
                    _ => {}
                }
            }
        }
    }

    fn validate_input(&mut self) {
        if self.has_repeat() {
            self.valid_input = Some("repeating character".to_string());
        }
    }

    fn has_repeat(&mut self) -> bool {
        let mut characters: Vec<char> = vec![];
        for i in 0..self.plugs.len() {
            let to_check: Option<&(char, char)> = self.plugs.get(i);
            if !to_check.is_none() {
                if characters.contains(&to_check.unwrap().0) {
                    return true;
                } else {
                    characters.push(to_check.unwrap().0);
                }
    
                if characters.contains(&to_check.unwrap().1) {
                    return true;
                } else {
                    characters.push(to_check.unwrap().1);
                }
            } else {
                return true;
            }
            
        }
        return false;
    }
}
fn main() {
    println!("Hello, world!");
    
}
