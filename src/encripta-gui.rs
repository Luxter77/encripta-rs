use cli_clipboard::{ClipboardContext, ClipboardProvider};
use iced::alignment::{Horizontal, Vertical};
use iced::event::{self, Event};
use iced::keyboard;
use iced::subscription;
use iced::theme::Theme;
use iced::widget::{self, button, column, container, row, scrollable, text, text_input};
use iced::{alignment, window, Font, Settings};
use iced::{Application, Element};
use iced::{Color, Command, Length, Subscription};

use once_cell::sync::Lazy;

static INPUT_ID: Lazy<text_input::Id> = Lazy::new(text_input::Id::unique);
static CYPHER_ID: Lazy<text_input::Id> = Lazy::new(text_input::Id::unique);

static APP_TITLE: Lazy<widget::Text> = Lazy::new(|| {
    iced::widget::Text::new("Encripta-RS")
        .width(Length::Fill)
        .size(70)
        .style(Color::from([0.7, 0.5, 0.5]))
        .horizontal_alignment(alignment::Horizontal::Center)
});

#[derive(Default, Debug, Clone, Copy)]
pub(crate) enum Direction {
    #[default]
    Forward,
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

pub(crate) struct State {
    input_value: String,
    output_tvalue: Option<String>,
    output_message: Option<String>,
    output_bvalue: Option<Vec<u8>>,
    cypher: String,
    direction: Direction,
    clip: ClipboardContext,
}

impl Default for State {
    fn default() -> Self {
        Self {
            input_value: String::new(),
            output_tvalue: None,
            output_message: None,
            output_bvalue: Some(Vec::new()),
            cypher: String::from_utf8(encripta::MAGIC.to_vec()).unwrap(),
            direction: Direction::default(),
            clip: ClipboardContext::new().unwrap(),
        }
    }
}

#[derive(Debug, Clone)]
pub(crate) enum Message {
    TabPressed { shift: bool },
    CypherChanged(String),
    InputChanged(String),
    BinaryToClipboard,
    TextToClipboard,
    SwitchDirection,
    ProcessText,
}

impl State {
    fn shift_update(&mut self) {
        let o: Result<String, encripta::ShiftError> = match self.direction {
            Direction::Forward => {
                encripta::forward(self.input_value.as_str(), self.cypher.as_bytes())
            }
            Direction::Backward => {
                encripta::backward(self.input_value.as_str(), self.cypher.as_bytes())
            }
        };
        match o {
            Ok(t) => {
                self.output_message = None;
                self.output_bvalue = Some(t.as_bytes().to_vec());
                self.output_tvalue = Some(t);
            }
            Err(e) => match e {
                encripta::ShiftError::NonRepresentableInput(e) => {
                    self.output_message = Some(format!("Input Error: {e}"));
                    self.output_bvalue = None;
                    self.output_tvalue = None;
                }
                encripta::ShiftError::InvalidCypher(e) => {
                    self.output_message = Some(format!("Cypher Error: {e}"));
                    self.output_bvalue = None;
                    self.output_tvalue = None;
                }
                encripta::ShiftError::NonRepresentableOutput((b, e)) => {
                    self.output_message = Some(format!("Output Error: {e}"));
                    self.output_bvalue = Some(b);
                    self.output_tvalue = None;
                }
            },
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
            },
            Message::TextToClipboard => {
                if let Some(o) = self.output_tvalue.clone() {
                    self.clip.set_contents(o).unwrap()
                };
                Command::none()
            },
            Message::BinaryToClipboard => {
                if let Some(o) = self.output_bvalue.clone() {
                    self.clip.set_contents(format!("{:?}", o)).unwrap()
                };
                Command::none()
            },
        }
    }

    fn view(&self) -> Element<Message> {
        let input: widget::TextInput<Message> =
            text_input("INPUT: ", self.input_value.as_str(), Message::InputChanged)
                .id(INPUT_ID.clone())
                .on_submit(Message::ProcessText)
                .padding(10)
                .font(FONT)
                .size(18);

        let cypher: widget::TextInput<Message> =
            text_input("CYPHER:", self.cypher.as_str(), Message::CypherChanged)
                .id(CYPHER_ID.clone())
                .on_submit(Message::ProcessText)
                .padding(10)
                .font(FONT)
                .size(16);

        let directionswitch: widget::Button<Message> =
            button(text(self.direction).font(FONT).size(16))
                .on_press(Message::SwitchDirection)
                .padding(10)
                .width(Length::Fill);

        let mut copyt: widget::Button<Message> = button(
            text("Copy Text")
                .font(FONT)
                .size(16)
                .horizontal_alignment(Horizontal::Center)
                .vertical_alignment(Vertical::Center),
        )
        .width(Length::Shrink)
        .padding(5);

        let mut copyb: widget::Button<Message> = button(
            text("Copy Binary")
                .font(FONT)
                .size(16)
                .horizontal_alignment(Horizontal::Center)
                .vertical_alignment(Vertical::Center),
        )
        .width(Length::Shrink)
        .padding(5);

        let output_box: widget::Text = if let Some(t) = self.output_tvalue.clone() {
            copyt = copyt.on_press(Message::TextToClipboard);
            copyb = copyb.on_press(Message::BinaryToClipboard);
            text(t)
        } else {
            text(String::new())
        }
        .font(FONT)
        .size(24)
        .width(Length::Fill)
        .horizontal_alignment(Horizontal::Center);

        let output_message: widget::Text = if let Some(t) = self.output_message.clone() {
            copyb = copyb.on_press(Message::BinaryToClipboard);
            text(t)
        } else {
            text(String::new())
        }
        .font(FONT)
        .size(22)
        .width(Length::Fill)
        .horizontal_alignment(Horizontal::Center)
        .style(Color::from([0.3, 0.3, 0.3]));

        let content = column![
            APP_TITLE.clone(),
            input,
            row(vec![cypher.into(), directionswitch.into()])
                .padding(10)
                .width(Length::Fill)
                .spacing(10),
            row(vec![copyt.into(), copyb.into()])
                .padding(10)
                .width(Length::Fill)
                .spacing(10),
            scrollable(output_message),
            scrollable(output_box),
        ]
        .spacing(20)
        .max_width(500);

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
const FONT: Font = Font::External {
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
