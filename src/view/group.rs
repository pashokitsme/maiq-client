use super::lesson::LessonComponent;
use super::{icon_button, Component, LessonMessage};
use iced::theme::Button;
use iced::widget::{column, container, rule::Rule};
use iced::widget::{row, text_input};
use iced::{Element, Length};
use iced_aw::Icon;
use maiq_shared::{Group, Lesson, Uid};

pub trait GroupComponent {
  fn new() -> Self;
  fn update_lesson(&mut self, idx: usize, message: LessonMessage);
  fn remove_lesson(&mut self, idx: usize);
}

#[derive(Debug, Clone)]
pub enum Message {
  EditName(String),
  Lesson((usize, LessonMessage)),
  CreateLesson,
  Remove,
}

impl GroupComponent for Group {
  fn new() -> Self {
    Group { uid: "?".into(), ..Group::default() }
  }

  fn update_lesson(&mut self, idx: usize, message: LessonMessage) {
    if let Some(l) = self.lessons.get_mut(idx) {
      l.update(message);
    }
  }

  fn remove_lesson(&mut self, idx: usize) {
    self.lessons.remove(idx);
  }
}

impl Component for Group {
  type Message = Message;

  fn update(&mut self, message: Message) {
    self.uid = self.uid();
    match message {
      Message::EditName(name) => self.name = name,
      Message::CreateLesson => self.lessons.push(Lesson::new(self.lessons.last())),
      Message::Lesson((idx, LessonMessage::Remove)) => self.remove_lesson(idx),
      Message::Lesson((idx, message)) => self.update_lesson(idx, message),
      _ => (),
    }
  }

  fn view(&self) -> Element<Self::Message> {
    let name_field = text_input("Группа", &self.name.to_string(), Message::EditName).width(Length::Fixed(80.));
    let header = row![
      name_field,
      icon_button(Icon::Plus).on_press(Message::CreateLesson),
      icon_button(Icon::Trash)
        .on_press(Message::Remove)
        .style(Button::Destructive),
    ]
    .align_items(iced::Alignment::Center)
    .spacing(20)
    .padding([0, 10, 10, 15]);

    let lessons = column(
      self
        .lessons
        .iter()
        .enumerate()
        .map(|(idx, l)| l.view().map(move |msg| Message::Lesson((idx, msg))))
        .collect(),
    )
    .spacing(10)
    .padding([10, 0, 15, 0]);

    let content = column![header, Rule::horizontal(1), lessons];
    container(content).into()
  }
}
