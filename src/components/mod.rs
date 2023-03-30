use iced::Element;

pub mod group;
pub mod lesson;

pub trait Component {
  type Message;
  fn update(&mut self, message: Self::Message);
  fn view(&self) -> Element<Self::Message>;
}
