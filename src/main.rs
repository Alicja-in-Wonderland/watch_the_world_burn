use iced::{
    widget::{button, column, container, row, text},
    Element,
};

fn main() -> iced::Result {
    iced::run("Watch the World Burn", update, view)
}

struct BurnBabyBurn {
    text: String,
}
impl Default for BurnBabyBurn {
    fn default() -> Self {
        Self {
            text: String::from("Come on, press it... I know you want to!"),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
enum Incinerator {
    Incinerate,
}

fn update(state: &mut BurnBabyBurn, message: Incinerator) {
    println!("update()");

    use Incinerator::*;
    if message == Incinerate {
        state.text.push_str("Burn 🔥");
        println!("Incinerate");
    }
}

fn view(state: &BurnBabyBurn) -> Element<Incinerator> {
    let prompt = row!(text(&state.text)).padding(10);
    let trigger = row!(button("Burn Baby, Burn").on_press(Incinerator::Incinerate)).padding(10);

    let contents = column!(prompt, trigger).padding(10);

    container(contents)
        .height(700)
        .width(1000)
        .align_x(iced::alignment::Horizontal::Center)
        .padding(10)
        .into()
}

// fn theme(&self) -> iced::Theme {
//     iced::Theme::Dark
// }
