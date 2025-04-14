use iced::{Application, Element, widget::Text, executor, Theme};

fn main() {
    App::run(Default::default()).unwrap();
}

struct App;

impl Application for App {
    type Message = ();
    type Theme = Theme;
    type Executor = executor::Default;
    type Flags = ();

    fn new(_flags: ()) -> (Self, iced::Command<Self::Message>) {
        (App, iced::Command::none())
    }

    fn title(&self) -> String {
        "Popup UI".to_string()
    }

    fn update(&mut self, _message: Self::Message) -> iced::Command<Self::Message> {
        iced::Command::none()
    }

    fn view(&self) -> Element<'_, Self::Message> {
        Text::new("Search here...").into()
    }
}
