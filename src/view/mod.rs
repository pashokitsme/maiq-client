use iced::{
  theme::Button,
  widget::{button, row, text},
  Element,
};
use iced_aw::{Icon, ICON_FONT};

pub mod editor;
pub mod group;
pub mod lesson;
pub mod toolbar;

pub type GroupMessage = group::Message;
pub type LessonMessage = lesson::Message;
pub type EditorMessage = editor::Message;

pub trait Component {
  type Message;
  fn update(&mut self, message: Self::Message);
  fn view(&self) -> Element<Self::Message>;
}

pub fn basic_button<'a, Message>(
  content: impl Into<Element<'a, Message, iced::Renderer>>,
  message: Message,
) -> button::Button<'a, Message, iced::Renderer> {
  button(content).on_press(message).padding(4).style(Button::Secondary)
}

pub fn icon_button<'a, Message>(icon: Icon) -> button::Button<'a, Message, iced::Renderer> {
  button(text(icon).font(ICON_FONT))
}

pub fn with_icon<'a, Message: 'a>(content: impl Into<Element<'a, Message>>, icon: Icon) -> Element<'a, Message> {
  row![text(icon).font(ICON_FONT), content.into()]
    .spacing(5)
    .align_items(iced::Alignment::Center)
    .into()
}
