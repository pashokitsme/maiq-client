use iced::{
  theme::Button,
  widget::{button, row, text, text_input},
  Element, Length,
};
use iced_aw::Icon;
use maiq_shared::Lesson;

use super::{icon_button, Component};

pub trait LessonComponent {
  fn new(prev: Option<&Lesson>) -> Self;
  fn set_num(&mut self, num: String);
  fn set_subgroup(&mut self, num: String);
}

#[derive(Debug, Clone)]
pub enum Message {
  EditNum(String),
  EditSubgroup(String),
  EditName(String),
  EditTeacher(String),
  EditClassroom(String),
  Remove,
  Default,
}

impl LessonComponent for Lesson {
  fn new(prev: Option<&Lesson>) -> Self {
    let num = prev
      .map(|l| l.num.filter(|n| *n < 9).map(|n| n + 1))
      .unwrap_or_default();
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
  type Message = Message;

  fn update(&mut self, message: Self::Message) {
    match message {
      Message::EditNum(n) => self.set_num(n),
      Message::EditSubgroup(sb) => self.set_subgroup(sb),
      Message::EditName(x) => self.name = x,
      Message::EditTeacher(x) => self.teacher = Some(x),
      Message::EditClassroom(x) => self.classroom = Some(x),
      Message::Default => *self = Self::default(),
      _ => (),
    }
  }

  fn view(&self) -> Element<Self::Message> {
    row![
      text("#").size(20),
      text_input("#", &self.num.map(|n| n.to_string()).unwrap_or_default(), Message::EditNum).width(20),
      text_input("&", &self.subgroup.map(|sb| sb.to_string()).unwrap_or_default(), Message::EditSubgroup).width(20),
      text_input("Предмет", &self.name, Message::EditName).width(Length::FillPortion(7)),
      text_input("Преподаватель", if let Some(teacher) = &self.teacher { teacher } else { "" }, Message::EditTeacher)
        .width(Length::FillPortion(3)),
      text_input("Ауд.", if let Some(classroom) = &self.classroom { classroom } else { "" }, Message::EditClassroom)
        .width(Length::FillPortion(1)),
      button("D").on_press(Message::Default).style(Button::Secondary),
      icon_button(Icon::Trash)
        .on_press(Message::Remove)
        .style(Button::Destructive)
    ]
    .align_items(iced::Alignment::Center)
    .padding([0, 0, 0, 15])
    .spacing(10)
    .into()
  }
}
