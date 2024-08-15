mod widgets;

use iced::widget::{column, container, row, text};
use iced::{Alignment, Element, Length, Renderer, Sandbox, Settings};
use widgets::letter_box::letter_box;

pub fn main() -> iced::Result {
    env_logger::init();

    Wordle::run(Settings::default())
}

pub struct Wordle {

}

impl Sandbox for Wordle {
    type Message = Message;

    fn new() -> Self {
        Self {}
    }

    fn title(&self) -> String {
        "Title thing".to_owned()
    }

    fn update(&mut self, event: Message) {
        match event {

        }
    }

    fn view(&self) -> Element<Message> {
        let content = column![
            text("text"),
            row![
                letter_box(),
                letter_box(),
                letter_box(),
                letter_box(),
                letter_box()
            ],
            row![
                letter_box(),
                letter_box(),
                letter_box(),
                letter_box(),
                letter_box()
            ],
            row![
                letter_box(),
                letter_box(),
                letter_box(),
                letter_box(),
                letter_box()
            ],
            row![
                letter_box(),
                letter_box(),
                letter_box(),
                letter_box(),
                letter_box()
            ],
            row![
                letter_box(),
                letter_box(),
                letter_box(),
                letter_box(),
                letter_box()
            ]
        ]
        .padding(20)
        .spacing(20)
        .max_width(500)
        .align_items(Alignment::Center);

        container(content).height(Length::Fill).center_y().into()
    }
}

#[derive(Debug, Clone)]
pub enum Message {

}