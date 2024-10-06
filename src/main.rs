use std::fs::File;
use std::io::Read;

use iced::Size;
use iced_gif::Frames;

mod view;

const WINDOW_SIZE: Size = Size::new(400.0, 400.0);
const BURNING_PATH: &str = "assets/watching.gif";

fn main() -> iced::Result {
    let x = iced::window::Settings {
        size: WINDOW_SIZE,
        ..iced::window::Settings::default()
    };
    iced::application("Watch the World Burn", update, view::view)
        .window(x)
        .centered()
        .run()
}

// musisz gdzieś zapisać klatki, które bedziesz chciała wyświetlić w widgecie
// dobrym miejscem jest nasz State, tak myślę.
struct WorldState {
    text: String,
    button_size: f32,
    frames: Frames,
    show_gif: bool,
}

impl Default for WorldState {
    fn default() -> Self {
        let mut gif_file = File::open(BURNING_PATH).unwrap();
        let mut byte_buffer = Vec::new();
        gif_file.read_to_end(&mut byte_buffer).unwrap();
        let frames = Frames::from_bytes(byte_buffer).unwrap();

        Self {
            text: String::from("Come on, press it... I know you want to!"),
            button_size: 200.0,
            frames,
            show_gif: false,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
enum IncineratorMessage {
    Incinerate,
}

fn update(state: &mut WorldState, message: IncineratorMessage) {
    println!("update()");

    use IncineratorMessage::*;
    if message == Incinerate {
        state.text = ("Burning...").to_string();
        state.show_gif = true;
        println!("Incinerate");
    }
}
