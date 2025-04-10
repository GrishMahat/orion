use iced::{Application, Element, Text};

fn main() {
    println!("Hello, world!");
}


struct App;

impl Application for App {
    type Message = ();
    fn new() -> Self {
        App
    }
    fn title(&self) -> String {
        "Popup UI".to_string().into()
    }

    fn update(&mut self, _message: Self::Message) {
        // No updates needed
    }

    fn view(&self) -> Element<'_, Self::Message> {
        Text::new("Search hear ..").into()
}
