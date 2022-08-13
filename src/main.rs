use iced::{Application, button, Command, Element, Settings, text_input, TextInput};
use iced::Font::Default;

fn main() -> iced::Result {
    let mut settings = Settings::default();
    settings.window.decorations = true;
    settings.window.transparent = false;
    settings.window.size = (800, 200);
    settings.window.max_size = Some(settings.window.size);
    settings.window.min_size = Some(settings.window.size);
    Counter::run(settings)
}

#[derive(Debug, Clone)]
pub enum Message {
    IncrementPressed,
    DecrementPressed,
    RefreshContent(String),
}

struct Counter {
    // The counter value
    value: i32,
    // the app title
    title: String,

    // The local state of the two buttons
    increment_button: button::State,
    decrement_button: button::State,

    contentState: text_input::State,
    content: String,
}


impl Application for Counter {
    type Executor = iced::executor::Default;
    type Message = Message;
    type Flags = ();

    fn new(_flags: Self::Flags) -> (Self, Command<Self::Message>) {
        return (Counter {
            value: 0,
            title: String::from("A Counter"),
            increment_button: button::State::new(),
            decrement_button: button::State::new(),
            contentState: text_input::State::new(),
            content: String::from(""),
        },
                Command::none());
    }

    fn title(&self) -> String {
        self.title.clone()
    }

    fn update(&mut self, message: Message) -> Command<Message> {
        match message {
            Message::IncrementPressed => {
                self.value += 1;
            }
            Message::DecrementPressed => {
                self.value -= 1;
            }
            Message::RefreshContent(c) => {
                self.content = c
            }
        }

        return Command::none();
    }


    fn view(&mut self) -> Element<Message> {
        let content = self.content.clone();
        TextInput::new(&mut self.contentState, "11111111", content.as_str(), |msg| -> Message{
            println!("{}", msg);
            return Message::RefreshContent(msg);
        }).into()
        // // We use a column: a simple vertical layout
        // Column::new()
        //     .push(
        //         // The increment button. We tell it to produce an
        //         // `IncrementPressed` message when pressed
        //         Button::new(&mut self.increment_button, Text::new("+"))
        //             .on_press(Message::IncrementPressed),
        //     )
        //     .push(
        //         // We show the value of the counter here
        //         Text::new(&self.value.to_string()).size(50),
        //     )
        //     .push(
        //         // The decrement button. We tell it to produce a
        //         // `DecrementPressed` message when pressed
        //         Button::new(&mut self.decrement_button, Text::new("-"))
        //             .on_press(Message::DecrementPressed),
        //     ).into()
    }
}

