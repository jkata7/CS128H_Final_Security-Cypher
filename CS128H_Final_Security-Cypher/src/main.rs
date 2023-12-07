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

            self.valid_input = None;
        }
    }
}
fn main() {
    println!("Hello, world!");
    
}
