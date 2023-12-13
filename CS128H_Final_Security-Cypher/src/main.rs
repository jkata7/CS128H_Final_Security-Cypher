use druid::Event;
use std::sync::{Arc, Mutex};
use druid::EventCtx;
use druid::Insets;
use druid::UnitPoint;
use druid::widget::Align;
use druid::Lens;
use druid::widget::Controller;
use druid::widget::TextBox;
use druid::widget::{Button, Flex, Label};
use druid::{AppLauncher, Env, LocalizedString, PlatformError, Widget, WidgetExt, WindowDesc, Data};

mod enigma;

#[derive(Clone, Default, Lens, Debug)]
struct AppState {
    plugs_input: String,
    rotor_1: i32,
    rotor_1_text: String,
    rotor_2: i32, 
    rotor_2_text: String,
    rotor_3: i32,
    rotor_3_text: String,
    plugs: Vec<(char, char)>,
    valid_input: Option<String>,
    to_encode: String,
    encoded: String,
}

trait AppStateAsRef {
    fn as_ref(&self) -> &AppState;
}

impl AppStateAsRef for AppState {
    fn as_ref(&self) -> &AppState {
        self
    }
}


impl Data for AppState {
    fn same(&self, other: &Self) -> bool {
        
        self.plugs_input == other.plugs_input
            && self.rotor_1 == other.rotor_1
            && self.rotor_2 == other.rotor_2
            && self.rotor_3 == other.rotor_3
            && self.plugs == other.plugs
            && self.valid_input == other.valid_input
            && self.rotor_1_text == other.rotor_1_text
            && self.rotor_2_text == other.rotor_2_text
            && self.rotor_3_text == other.rotor_3_text
            && self.to_encode == other.to_encode
            && self.encoded == other.encoded
    }
}

impl AppState {
    fn update_configs (&mut self) {
        if self.plugs_input.len() == 20 {
            self.plugs.clear();

            for chunk in self.plugs_input.chars().collect::<Vec<_>>().chunks_exact(2) {
                match chunk {
                    [c1, c2] => self.plugs.push((*c1, *c2)),
                    _ => {}
                }
            }
        }

        let rotor_1_result = self.rotor_1_text.parse::<i32>();
        let rotor_2_result = self.rotor_2_text.parse::<i32>();
        let rotor_3_result = self.rotor_3_text.parse::<i32>();

        match rotor_1_result {
            Ok(value) => self.rotor_1 = value,
            Err(_) => {}
        }

        match rotor_2_result {
            Ok(value) => self.rotor_2 = value,
            Err(_) => {}
        }

        match rotor_3_result {
            Ok(value) => self.rotor_3 = value,
            Err(_) => {}
        }

    }

    fn validate_input(&mut self) {
        self.valid_input = None;
        if self.plugs_input.len() > 20 || self.plugs_input.len() < 20 {
            self.valid_input = Some("you did not enter 20 characters".to_string());
            return;
        }

        if self.has_repeat() {
            self.valid_input = Some("repeating character, or non-lowercase alphabetical characters".to_string());
            return;
        }

        let rotor_1_result = self.rotor_1_text.parse::<i32>();
        let rotor_2_result = self.rotor_2_text.parse::<i32>();
        let rotor_3_result = self.rotor_3_text.parse::<i32>();

        match rotor_1_result {
            Ok(_) => {},
            Err(_) => self.valid_input = Some("please type a valid integer between 1 and 26 in rotor 1".to_string())
        }

        match rotor_2_result {
            Ok(_) => {},
            Err(_) => self.valid_input = Some("please type a valid integer between 1 and 26 in rotor 2".to_string())
        }

        match rotor_3_result {
            Ok(_) => {},
            Err(_) => self.valid_input = Some("please type a valid integer between 1 and 26 in rotor 3".to_string())
        }

        let rotor_1_valid: bool = self.rotor_1 >= 1 && self.rotor_1 <= 26;
        let rotor_2_valid: bool = self.rotor_2 >= 1 && self.rotor_2 <= 26;
        let rotor_3_valid: bool = self.rotor_3 >= 1 && self.rotor_3 <= 26;

        if !(rotor_1_valid && rotor_2_valid && rotor_3_valid) {
            self.valid_input = Some("please type valid integers between 1 and 26".to_string());
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
                    let character: char = to_check.unwrap().0;
                    if character.is_ascii_lowercase() {
                        characters.push(character);
                    } else {
                        return true;
                    }
                }
    
                if characters.contains(&to_check.unwrap().1) {
                    return true;
                } else {
                    let character: char = to_check.unwrap().1;
                    if character.is_ascii_lowercase() {
                        characters.push(character);
                    } else {
                        return true;
                    }
                }
            } else {
                return true;
            }
        }
        return false;
    }
}


fn build_ui() -> impl Widget<AppState> {
    let plugs_input = TextBox::new()
        .with_placeholder("Enter 20 characters...")
        .lens(AppState::plugs_input)
        .controller(MyController);

    let rotor_1_input = TextBox::new()
        .with_placeholder("Rotor 1: Enter a number between 1 and 26")
        .lens(AppState::rotor_1_text).
        controller(MyController);

    let rotor_2_input = TextBox::new()
        .with_placeholder("Rotor 2: Enter a number between 1 and 26")
        .lens(AppState::rotor_2_text)
        .controller(MyController);

    let rotor_3_input = TextBox::new()
        .with_placeholder("Rotor 3: Enter a number between 1 and 26")
        .lens(AppState::rotor_3_text)
        .controller(MyController);

    let to_encode_input = TextBox::new()
        .with_placeholder("Enter what you would like to encode")
        .lens(AppState::to_encode)
        .controller(MyController);
    
    // let mut to_show_label = Label::new(|_data: &AppState, _env: &_| {
    //     return "".to_string();
    // });

    // let submit_button = Button::new("Submit")
    //     .on_click(move |_, data: &mut AppState, _| {
    //         data.validate_input();
    //         if data.valid_input.is_none() {
    //             to_show_label.set_text("".to_string());
    //             do all the stuff
    //         } else {
    //             to_show_label.set_text(data.valid_input.as_ref().unwrap().to_string());
    //         }
    //     });

    let to_show = Arc::new(Mutex::new(Label::<String>::new("".to_string())));
    
    let submit_button = Button::new("Submit")
    .on_click(move |_, data: &mut AppState, _| {
        data.validate_input();
        let mut label_clone = to_show.lock().unwrap();
        if data.valid_input.is_none() {
            println!("{}", data.rotor_1);
            println!("{}", data.rotor_2);
            println!("{}", data.rotor_3);
            println!("{}", data.to_encode);
            data.encoded = enigma::enigma::enigma(data.plugs.as_ref(), &data.rotor_1, &data.rotor_2, &data.rotor_3, &data.to_encode);
            println!("{}", data.encoded);
            label_clone.set_text(data.encoded.to_string());
            
        } else {
            label_clone.set_text(data.valid_input.as_ref().unwrap().to_string());
        }
    });

    // Arrange the widgets in a column
    Flex::column()
    .with_child(Align::new(UnitPoint::CENTER, Label::new("Enigma machine")).padding(Insets::uniform(10.0)))
    .with_child(Align::new(UnitPoint::CENTER, Label::new("Instructions: Enter a sequence of 20 distinct characters for the plugboard, 
    \n and three numbers between 1 and 26 for the rotor configurations. 
    \n Then, type the text you want to encrypt. 
    \n To decrypt, use the same settings and then type the encrypted text in the last box.")).padding(Insets::uniform(10.0)))
    .with_child(plugs_input)
    .with_child(rotor_1_input)
    .with_child(rotor_2_input)
    .with_child(rotor_3_input)
    .with_child(to_encode_input)
    .with_child(submit_button)
    .with_child(Label::dynamic(move |data: &AppState, _| {
        data.valid_input.as_ref().unwrap_or(&String::new()).clone()
    }))
    .with_child(Label::dynamic(move |data: &AppState, _| {
        data.encoded.clone()
    }))
    .padding(10.0)


}

struct MyController;

impl<W: Widget<AppState>> Controller<AppState, W> for MyController {
    fn event(&mut self, 
        child: &mut W, 
        ctx: &mut EventCtx<'_, '_>, 
        event: &Event,
        data: &mut AppState, 
        env: &Env
    ) {
        data.update_configs();
        child.event(ctx, event, data, env);
    }
}


fn main() -> Result<(), PlatformError> {
    let main_window = WindowDesc::new(build_ui)
        .window_size((400.0, 250.0))
        .title(LocalizedString::new("Extended Input Example"));

    let initial_state = AppState {
        plugs_input: "".to_string(),
        rotor_1_text: "".to_string(),
        rotor_2_text: "".to_string(),
        rotor_3_text: "".to_string(),
        rotor_1: 1,
        rotor_2: 1,
        rotor_3: 1,
        plugs: Vec::<(char, char)>::new(),
        valid_input: None,
        to_encode: "".to_string(),
        encoded: "".to_string()
    };

    AppLauncher::with_window(main_window)
        .launch(initial_state)
}