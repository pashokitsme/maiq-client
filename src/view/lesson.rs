use std::fmt::Display;

use iced::{
  theme::Button,
  widget::{button, container, pick_list, row, text_input},
  Element, Length,
};
use iced_aw::Icon;
use maiq_shared::default::{self, DefaultLesson};

use super::{icon_button, Component};

pub trait LessonComponent {
  fn new(prev: Option<&DefaultLesson>) -> Self;
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
  ForDaySelected(ForDay),
  Remove,
}

impl LessonComponent for DefaultLesson {
  fn new(prev: Option<&DefaultLesson>) -> Self {
    let num = prev.map(|l| if l.num > 9 { l.num } else { l.num + 1 }).unwrap_or(1);
    Self { num, ..DefaultLesson::default() }
  }

  fn set_num(&mut self, num: String) {
    let num = num
      .chars()
      .last()
      .filter(|c| c.is_ascii_digit())
      .map(|c| c as u8 - b'0');

    match num {
      Some(x) if x > 0 => self.num = x,
      _ => (),
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ForDay {
  #[default]
  Every,
  Even,
  Odd,
}

impl From<ForDay> for Option<bool> {
  fn from(value: ForDay) -> Self {
    match value {
      ForDay::Even => Some(true),
      ForDay::Odd => Some(false),
      ForDay::Every => None,
    }
  }
}

impl From<Option<bool>> for ForDay {
  fn from(value: Option<bool>) -> Self {
    match value {
      Some(true) => ForDay::Even,
      Some(false) => ForDay::Odd,
      None => ForDay::Every,
    }
  }
}

impl Display for ForDay {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(
      f,
      "{}",
      match self {
        ForDay::Every => "Всегда",
        ForDay::Even => "Чётная",
        ForDay::Odd => "Нечётная",
      }
    )
  }
}

const DAYS: [ForDay; 3] = [ForDay::Every, ForDay::Even, ForDay::Odd];

impl Component for DefaultLesson {
  type Message = Message;

  fn update(&mut self, message: Self::Message) {
    match message {
      Message::EditNum(n) => self.set_num(n),
      Message::EditSubgroup(sb) => self.set_subgroup(sb),
      Message::EditName(x) => self.name = x,
      Message::EditTeacher(x) => self.teacher = Some(x),
      Message::EditClassroom(x) => self.classroom = Some(x),
      Message::ForDaySelected(x) => self.is_even = x.into(),
      _ => (),
    }
  }

  fn view(&self) -> Element<Self::Message> {
    let dropdown = container(pick_list(&DAYS[..], Some(Into::<ForDay>::into(self.is_even)), Message::ForDaySelected))
      .width(Length::Fixed(110.0));
    row![
      text_input("#", &self.num.to_string(), Message::EditNum).width(20),
      text_input("&", &self.subgroup.map(|sb| sb.to_string()).unwrap_or_default(), Message::EditSubgroup).width(20),
      dropdown,
      text_input("Предмет", &self.name, Message::EditName).width(Length::FillPortion(7)),
      text_input("Преподаватель", if let Some(teacher) = &self.teacher { teacher } else { "" }, Message::EditTeacher)
        .width(Length::FillPortion(3)),
      text_input("Ауд.", if let Some(classroom) = &self.classroom { classroom } else { "" }, Message::EditClassroom)
        .width(Length::FillPortion(1)),
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
