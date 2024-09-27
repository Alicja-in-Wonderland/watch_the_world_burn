use iced::{
    widget::{button, column, container, row, text},
    Sandbox, Settings,
};

fn main() -> iced::Result {
    BurnBabyBurn::run(Settings::default())
}

struct BurnBabyBurn {
    text: String,
}

#[derive(Debug, Clone, PartialEq)]
enum Incinerator {
    Incinerate,
}

impl Sandbox for BurnBabyBurn {
    type Message = Incinerator;

    fn new() -> Self {
        Self {
            text: String::from("Come on, press it... I know you want to!"),
        }
    }

    fn title(&self) -> String {
        String::from("Watch the World Burn")
    }

    fn update(&mut self, message: Self::Message) {
        println!("update()");

        use Incinerator::*;
        if message == Incinerate {
            self.text.push_str("Burn 🔥");
            println!("Incinerate");
        }
    }

    fn view(&self) -> iced::Element<'_, Self::Message> {
        let prompt = row!(text(&self.text)).padding(10);
        let trigger = row!(button("Burn Baby, Burn").on_press(Incinerator::Incinerate).style()).padding(10);

        let contents = column!(prompt, trigger).padding(10);

        container(contents)
            .height(700)
            .width(1000)
            .align_x(iced::alignment::Horizontal::Center)
            .padding(10)
            .into()
    }

    fn theme(&self) -> iced::Theme {
        iced::Theme::Dark
    }
}
