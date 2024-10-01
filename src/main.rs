use iced::{
    widget::{button, column, container, row, text},
    Alignment::Center,
    Color, Element, Theme,
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
        state.text = ("Burning....").to_string();
        println!("Incinerate");
    }
}

fn view(state: &BurnBabyBurn) -> Element<Incinerator> {
    let square_with_text = container(column!["Burn,", "Baby Burn!"].align_x(Center))
        .center_x(200)
        .center_y(200);
   
    let trigger = button(square_with_text)
        .height(150)
        .width(150)
        .on_press(Incinerator::Incinerate)
        .style(burning_style);

    // .style(move |_, _| button::Style {
    //     border: iced::border::rounded(1000),
    //     background: Some(iced::Background::Color(Color::new(1.0, 0.0, 0.0, 1.0))),
    //     ..button::Style::default()
    // });

    let prompt = row!(text(&state.text)).padding(10);
    let contents = column!(prompt, trigger).padding(10);

    container(contents).into()
}

pub fn burning_style(_theme: &Theme, status: button::Status) -> button::Style {
    use button::{Status, Style};
    use iced::{border, Background, Shadow, Vector};

    let base_style = button::Style {
        text_color: Color::WHITE,
        border: border::rounded(150),
        shadow: Shadow {
            color: Color::BLACK,
            offset: Vector::new(5.0, 5.0),
            blur_radius: 15.0,
        },
        background: Some(Background::Color(Color::new(1.0, 0.0, 0.0, 1.0))),
        ..Style::default()
    };

    let hovered_style = button::Style {
        text_color: Color::BLACK,
        border: border::rounded(150),
        shadow: Shadow {
            color: Color::BLACK,
            offset: Vector::new(5.0, 5.0),
            blur_radius: 15.0,
        },
        background: Some(Background::Color(Color::new(1.0, 0.0, 0.0, 0.90))),
    };

    match status {
        // Status::Active | Status::Pressed | Status::Hovered => our_style,
        Status::Active => base_style,
        Status::Pressed => base_style,
        Status::Hovered => hovered_style,
        Status::Disabled => Style {
            background: base_style
                .background
                .map(|background| background.scale_alpha(0.5)),
            text_color: base_style.text_color.scale_alpha(0.5),
            ..base_style
        },
    }
}

// fn theme(&self) -> iced::Theme {
//     iced::Theme::Dark
// }
