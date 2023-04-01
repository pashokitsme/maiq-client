use super::{icon_button, Component};
use iced::{
  theme::{Button, Container, Text},
  widget::{column, container, row, text},
  Color,
};
use iced_aw::Icon;

pub struct Notification {
  pub header: String,
  pub color: Color,
  pub body: String,
}

impl Notification {
  pub fn error(header: impl Into<String>, body: impl Into<String>) -> Self {
    Self { header: header.into(), body: body.into(), color: Color::from_rgb8(240, 0, 0) }
  }

  pub fn ok(header: impl Into<String>, body: impl Into<String>) -> Self {
    Self { header: header.into(), body: body.into(), color: Color::from_rgb8(0, 0, 0) }
  }
}

impl Component for Notification {
  type Message = ();

  fn update(&mut self, _: Self::Message) {}

  fn view(&self) -> iced::Element<Self::Message> {
    let row = row![
      column![text(&self.header).size(14).style(Text::Color(self.color)), text(&self.body)].padding([0, 7]),
      icon_button(Icon::Trash).on_press(()).style(Button::Destructive)
    ]
    .padding(5)
    .align_items(iced::Alignment::Center);

    container(row).style(Container::Box).into()
  }
}
