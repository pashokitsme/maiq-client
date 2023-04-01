use super::{icon_button, Component};
use iced::{
  theme::{Button, Container},
  widget::{column, container, row, text},
  Length,
};
use iced_aw::Icon;

pub struct Notification {
  pub header: String,
  pub body: String,
}

impl Component for Notification {
  type Message = ();

  fn update(&mut self, _: Self::Message) {}

  fn view(&self) -> iced::Element<Self::Message> {
    let row = row![
      column![text(&self.header).size(14), text(&self.body)].padding([0, 7]),
      icon_button(Icon::Trash).on_press(()).style(Button::Destructive)
    ]
    .padding(5)
    .align_items(iced::Alignment::Center);

    container(row).style(Container::Box).into()
  }
}
