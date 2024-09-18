mod text_document;

use iced::{Element, Sandbox, Settings, Theme, Length, alignment};
use iced::widget::{container, text, Column, column, scrollable, text_input};
use crate::text_document::TextDocument;

#[derive(Debug, Clone)]
pub enum Message {
    KeyPress(char),
    File,
    Scroll
}

impl Sandbox for TextDocument {
    type Message = Message;

    fn new() -> TextDocument {
        TextDocument {line_buffer: Vec::new(), length: 0, character: 'a'}
    }
    fn title(&self) -> String {
        String::from("TextEditor")
    }
    fn update(&mut self, message:Message) {

    }
    fn view(&self) -> Element<'_, Self::Message> {
        container(
            column!((&self.line_buffer),
            text_input(&self.character))
        )
            .height(Length::Fill)
            .width(Length::Fill)
            .into()

    }
    fn theme(&self) -> iced::Theme {
        Theme::Dark
    }
}

fn paint_text(lines: &Vec<String>) -> Element<'static, Message> {
    let mut column = Column::new()
        .spacing(20)
        .width(Length::Fill);
    for line in lines {
        column = column.push(text(line))
    }
    scrollable(
        container(
            column
        )
    )
        .height(250)
        .width(300)
        .into()

}


fn main() {
    TextDocument::run(Settings::default());
}
