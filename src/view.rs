use iced::{
    border::Radius,
    widget::{button, column, container, row, text},
    Alignment::{self, Center},
    Border, Color, Element, Length, Theme,
};
use iced_gif::gif;

use crate::{IncineratorMessage, WorldState};

pub fn view(state: &WorldState) -> Element<IncineratorMessage> {
    let square_with_text = container(column!["Burn,", "Baby Burn!"].align_x(Center))
        .align_x(Center)
        .align_y(Center);

    let trigger = button(square_with_text)
        .height(state.button_size)
        .width(state.button_size)
        .on_press(IncineratorMessage::Incinerate)
        .style(burning_style);

    // .style(move |_, _| button::Style {
    //     border: iced::border::rounded(1000),
    //     background: Some(iced::Background::Color(Color::new(1.0, 0.0, 0.0, 1.0))),
    //     ..button::Style::default()
    // });

    let gif = gif(&state.frames);

    let prompt = row!(text(&state.text)).padding(10);

    if state.show_gif {
        let contents = column!(gif).padding(10);
        container(contents).into()
    } else {
        let contents = column!(prompt, trigger)
            .align_x(Alignment::Center)
            .padding(10);
        container(contents).center(Length::Fill).into()
    }
}

pub fn burning_style(_theme: &Theme, status: button::Status) -> button::Style {
    use button::{Status, Style};
    use iced::{Background, Shadow, Vector};

    let base_style = button::Style {
        text_color: Color::WHITE,
        border: Border {
            color: Color {
                r: 0.0,
                g: 0.0,
                b: 0.0,
                a: 0.85,
            },
            width: 2.5,
            radius: Radius::new(150.0),
        },

        shadow: Shadow {
            color: Color::BLACK,
            offset: Vector::new(5.0, 5.0),
            blur_radius: 15.0,
        },
        background: Some(Background::Color(Color::new(1.0, 0.0, 0.0, 1.0))),
    };

    // DRY! - we only set stuff that's different from base style, and..
    let hovered_style = button::Style {
        text_color: Color::BLACK,
        shadow: Shadow {
            color: Color::BLACK,
            offset: Vector::new(2.5, 2.5),
            blur_radius: 10.0,
        },
        background: Some(Background::Color(Color::new(1.0, 0.0, 0.0, 0.0))),
        ..base_style // ..the rest is as in base style
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
