use super::lesson::{Lesson, LessonMessage};
use super::Component;
use iced::widget::{button, row, text_input};
use iced::widget::{column, container, rule::Rule, text};
use iced::{ContentFit, Element, Length};

pub struct Group {
  pub name: String,
  pub lessons: Vec<Lesson>,
}

#[derive(Debug, Clone)]
pub enum GroupMessage {
  EditName(String),
  Lesson((usize, LessonMessage)),
  CreateLesson,
  Remove,
}

impl Group {
  fn update_lesson(&mut self, idx: usize, message: LessonMessage) {
    if let Some(l) = self.lessons.get_mut(idx) {
      l.update(message)
    }
  }

  fn remove_lesson(&mut self, idx: usize) {
    self.lessons.remove(idx);
  }
}

impl Default for Group {
  fn default() -> Self {
    Self { name: "Группа?".into(), lessons: Default::default() }
  }
}

impl Component for Group {
  type Message = GroupMessage;

  fn update(&mut self, message: GroupMessage) {
    match message {
      GroupMessage::EditName(name) => self.name = name,
      GroupMessage::CreateLesson => self.lessons.push(Lesson::new(self.lessons.last())),
      GroupMessage::Lesson((idx, LessonMessage::Remove)) => self.remove_lesson(idx),
      GroupMessage::Lesson((idx, message)) => self.update_lesson(idx, message),
      _ => (),
    }
  }

  fn view(&self) -> Element<Self::Message> {
    let name_field = text_input("Группа", &self.name.to_string(), GroupMessage::EditName).width(Length::Fixed(80.));
    let header = row![name_field, button("+").on_press(GroupMessage::CreateLesson), button("R").on_press(GroupMessage::Remove)]
      .spacing(20)
      .padding([30, 10, 10, 15]);

    let lessons = column(
      self
        .lessons
        .iter()
        .enumerate()
        .map(|(idx, l)| l.view().map(move |msg| GroupMessage::Lesson((idx, msg))))
        .collect(),
    )
    .spacing(10)
    .padding([10, 0, 15, 0]);

    let content = column![header, Rule::horizontal(1), lessons];
    container(content).into()
  }
}
