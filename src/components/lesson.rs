use iced::{
  widget::{button, row, text, text_input},
  Element, Length,
};

use super::Component;

#[derive(Default)]
pub struct Lesson {
  pub num: Option<u8>,
  pub subgroup: Option<u8>,
  pub name: String,
  pub teacher: String,
  pub classroom: String,
}

#[derive(Debug, Clone)]
pub enum LessonMessage {
  EditNum(String),
  EditSubgroup(String),
  EditName(String),
  EditTeacher(String),
  EditClassroom(String),
  Remove,
  Default,
}

impl Lesson {
  pub fn new(prev: Option<&Lesson>) -> Self {
    let num = prev.map(|l| l.num.map(|n| n + 1)).unwrap_or_default();
    Self { num, ..Lesson::default() }
  }

  fn set_num(&mut self, num: String) {
    let num = num
      .chars()
      .last()
      .filter(|c| c.is_ascii_digit())
      .map(|c| c as u8 - b'0');

    match num {
      Some(x) if x > 0 => self.num = Some(x),
      _ => self.num = None,
    }
  }

  fn set_subgroup(&mut self, num: String) {
    let num = num
      .chars()
      .last()
      .filter(|c| c.is_ascii_digit())
      .map(|c| c as u8 - b'0');

    match num {
      Some(x) if x > 0 && x < 3 => self.subgroup = Some(x),
      _ => self.subgroup = None,
    }
  }
}

impl Component for Lesson {
  type Message = LessonMessage;

  fn update(&mut self, message: Self::Message) {
    match message {
      LessonMessage::EditNum(n) => self.set_num(n),
      LessonMessage::EditSubgroup(sb) => self.set_subgroup(sb),
      LessonMessage::EditName(x) => self.name = x,
      LessonMessage::EditTeacher(x) => self.teacher = x,
      LessonMessage::EditClassroom(x) => self.classroom = x,
      LessonMessage::Default => *self = Self::default(),
      _ => (),
    }
  }

  fn view(&self) -> Element<Self::Message> {
    row![
      text("#").size(20),
      text_input("#", &self.num.map(|n| n.to_string()).unwrap_or_default(), LessonMessage::EditNum).width(20),
      text_input("&", &self.subgroup.map(|sb| sb.to_string()).unwrap_or_default(), LessonMessage::EditSubgroup).width(20),
      text_input("Предмет", &self.name, LessonMessage::EditName).width(Length::FillPortion(7)),
      text_input("Преподаватель", &self.teacher, LessonMessage::EditTeacher).width(Length::FillPortion(3)),
      text_input("Ауд.", &self.classroom, LessonMessage::EditClassroom).width(Length::FillPortion(1)),
      button("D").on_press(LessonMessage::Default),
      button("R").on_press(LessonMessage::Remove)
    ]
    .align_items(iced::Alignment::Center)
    .padding([0, 0, 0, 15])
    .spacing(10)
    .into()
  }
}
