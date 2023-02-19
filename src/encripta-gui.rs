use iced::alignment::Horizontal;
use iced::event::{self, Event};
use iced::keyboard;
use iced::subscription;
use iced::theme::Theme;
use iced::widget::{self, button, column, container, row, text, text_input};
use iced::{alignment, window, Font, Settings};
use iced::{Application, Element};
use iced::{Color, Command, Length, Subscription};

use once_cell::sync::Lazy;

static INPUT_ID: Lazy<text_input::Id> = Lazy::new(text_input::Id::unique);
static CYPHER_ID: Lazy<text_input::Id> = Lazy::new(text_input::Id::unique);

#[derive(Default, Debug, Clone, Copy)]
pub(crate) enum Direction {
    #[default] Forward,
    Backward,
}

impl ToString for Direction {
    fn to_string(&self) -> String {
        match self {
            Direction::Forward => String::from("Forward"),
            Direction::Backward => String::from("Backward"),
        }
    }
}

impl Direction {
    fn switched(&self) -> Self {
        match self {
            Self::Backward => Self::Forward,
            Self::Forward => Self::Backward,
        }
    }
}

#[derive(Debug)]
pub(crate) struct State {
    input_value:  String,
    output_value: String,
    cypher:       String,
    direction:    Direction,
}

impl Default for State {
    fn default() -> Self {
        Self {
            input_value:  String::new(),
            output_value: String::new(),
            cypher:       String::from_utf8(encripta::MAGIC.to_vec()).unwrap(),
            direction:    Direction::default(),
        }
    }
}

#[derive(Debug, Clone)]
pub(crate) enum Message {
    InputChanged(String),
    CypherChanged(String),
    TabPressed { shift: bool },
    SwitchDirection,
    ProcessText,
}

impl State {
    fn shift_update(&mut self) {
        self.output_value = match self.direction {
            Direction::Forward => {
                match encripta::forward(self.input_value.as_str(), self.cypher.as_bytes()) {
                    Ok(t) => t,
                    Err(e) => match e {
                        encripta::ShiftError::NonRepresentableInput(e) => {
                            format!("Input Error: {e}")
                        }
                        encripta::ShiftError::NonRepresentableOutput((b, e)) => {
                            format!("Output Error: {e}\n\tRaw output: {b:?}")
                        }
                        encripta::ShiftError::InvalidCypher(e) => format!("Cypher Error: {e}"),
                    },
                }
            }
            Direction::Backward => {
                match encripta::backward(self.input_value.as_str(), self.cypher.as_bytes()) {
                    Ok(t) => t,
                    Err(e) => match e {
                        encripta::ShiftError::NonRepresentableInput(e) => {
                            format!("Input Error: {e}")
                        }
                        encripta::ShiftError::NonRepresentableOutput((b, e)) => {
                            format!("Output Error: {e}\n\tRaw output: {b:?}")
                        }
                        encripta::ShiftError::InvalidCypher(e) => format!("Cypher Error: {e}"),
                    },
                }
            }
        };
    }
}

impl Application for State {
    type Message = Message;
    type Theme = Theme;
    type Executor = iced::executor::Default;
    type Flags = ();

    fn new(_flags: ()) -> (State, Command<Message>) {
        (State::default(), Command::none())
    }

    fn title(&self) -> String {
        String::from("Encripta-RS")
    }

    fn update(&mut self, message: Message) -> Command<Message> {
        match message {
            Message::InputChanged(value) => {
                self.input_value = value;
                self.shift_update();
                Command::none()
            }
            Message::CypherChanged(value) => {
                self.cypher = value;
                self.shift_update();
                Command::none()
            }
            Message::ProcessText => {
                self.shift_update();
                Command::none()
            }
            Message::TabPressed { shift } => {
                if shift {
                    widget::focus_previous()
                } else {
                    widget::focus_next()
                }
            }
            Message::SwitchDirection => {
                self.direction = self.direction.switched();
                self.shift_update();
                Command::none()
            }
        }
    }

    fn view(&self) -> Element<Message> {
        let title = text("Encripta-RS")
            .width(Length::Fill)
            .size(70)
            .style(Color::from([0.7, 0.5, 0.5]))
            .horizontal_alignment(alignment::Horizontal::Center);

        let input = text_input("INPUT: ", self.input_value.as_str(), Message::InputChanged)
            .id(INPUT_ID.clone())
            .padding(10)
            .font(CHARACTERS)
            .size(18)
            .on_submit(Message::ProcessText);
        let cypher = text_input("CYPHER:", self.cypher.as_str(), Message::CypherChanged)
            .id(CYPHER_ID.clone())
            .padding(10)
            .font(CHARACTERS)
            .size(16)
            .on_submit(Message::ProcessText);
        let directionswitch = button(text(self.direction).font(CHARACTERS).size(16))
            .on_press(Message::SwitchDirection)
            .padding(10)
            .width(Length::Fill);
        let output = text(self.output_value.as_str())
            .font(CHARACTERS)
            .size(24)
            .width(Length::Fill)
            .horizontal_alignment(Horizontal::Center);

        let content = column![
            title,
            input,
            row(vec![cypher.into(), directionswitch.into()]).padding(10).width(Length::Fill),
            output
        ]
        .spacing(20)
        .max_width(800);

        container(content)
            .width(Length::Fill)
            .padding(40)
            .center_x()
            .into()
    }

    fn subscription(&self) -> Subscription<Message> {
        subscription::events_with(|event, status| match (event, status) {
            (
                Event::Keyboard(keyboard::Event::KeyPressed {
                    key_code: keyboard::KeyCode::Tab,
                    modifiers,
                    ..
                }),
                event::Status::Ignored,
            ) => Some(Message::TabPressed {
                shift: modifiers.shift(),
            }),
            _ => None,
        })
    }
}

// Fonts
const CHARACTERS: Font = Font::External {
    name: "Unifont",
    bytes: include_bytes!("../assets/unifont.ttf"),
};

pub fn main() -> iced::Result {
    State::run(Settings {
        window: window::Settings {
            size: (500, 500),
            ..window::Settings::default()
        },
        ..Settings::default()
    })
}
