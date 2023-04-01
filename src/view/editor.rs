use std::slice::Iter;

use iced::widget::{button, container, row, text};
use iced_aw::{date_picker::Date, DatePicker, Icon, ICON_FONT};
use maiq_shared::{Group, Snapshot, Uid};

use super::{icon_button, Component, GroupMessage};

#[derive(Debug)]
pub struct SnapshotEditor {
  pub is_date_editing: bool,

  date: Date,
  snapshot: Snapshot,
}

#[derive(Debug, Clone)]
pub enum Message {
  ChooseDate,
  DatePickCancel,
  SubmitDate(Date),
  CreateGroup,
  Group((usize, GroupMessage)),
}

impl Default for SnapshotEditor {
  fn default() -> Self {
    Self { date: Date::today(), is_date_editing: false, snapshot: Default::default() }
  }
}

impl SnapshotEditor {
  pub fn groups(&self) -> Iter<Group> {
    self.snapshot.groups.iter()
  }

  pub fn update_group(&mut self, message: GroupMessage, idx: usize) {
    if let Some(g) = self.snapshot.groups.get_mut(idx) {
      g.update(message)
    }
  }

  pub fn create_group(&mut self) {
    self.snapshot.groups.push(Group::default())
  }

  pub fn remove_group(&mut self, idx: usize) {
    self.snapshot.groups.remove(idx);
  }

  fn save_to_file(&self) -> anyhow::Result<()> {
    // let path = format!("{}");
    Ok(())
  }

  fn new_file(&mut self) {}

  fn load_from_file(&mut self) -> anyhow::Result<()> {
    Ok(())
  }
}

impl Component for SnapshotEditor {
  type Message = Message;

  fn update(&mut self, message: Self::Message) {
    self.snapshot.uid = self.snapshot.uid();
    match message {
      Message::Group((idx, GroupMessage::Remove)) => self.remove_group(idx),
      Message::Group((idx, msg)) => self.update_group(msg, idx),
      Message::CreateGroup => self.create_group(),
      Message::ChooseDate => self.is_date_editing = !self.is_date_editing,
      Message::SubmitDate(date) => {
        self.date = date;
        self.is_date_editing = false
      }
      Message::DatePickCancel => self.is_date_editing = false,
    }
  }

  fn view(&self) -> iced::Element<Self::Message> {
    let datepicker = DatePicker::new(
      self.is_date_editing,
      self.date,
      button(text(Icon::PencilSquare).font(ICON_FONT)).on_press(Message::ChooseDate),
      Message::DatePickCancel,
      Message::SubmitDate,
    );

    let date = format!("{:0>2}.{:0>2}.{}", self.date.day, self.date.month, self.date.year);
    let content = row![
      icon_button(Icon::Plus).on_press(Message::CreateGroup),
      text(&format!("UID: {}", self.snapshot.uid())),
      row![text(&date), datepicker]
        .align_items(iced::Alignment::Center)
        .spacing(5),
    ]
    .align_items(iced::Alignment::Center)
    .spacing(25)
    .padding(5);

    container(content).into()
  }
}
