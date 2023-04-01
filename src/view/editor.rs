use std::{
  fs::{self, File},
  io::BufWriter,
  path::Path,
  slice::Iter,
};

use chrono::{Datelike, NaiveDateTime, NaiveTime, Utc};
use iced::widget::{button, container, row, text};
use iced_aw::{date_picker::Date, DatePicker, Icon, ICON_FONT};
use maiq_shared::{default::DefaultDay, utils::time::now, Group, Lesson, Snapshot, Uid};

use crate::env;

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
    let mut editor = Self { date: Date::today(), is_date_editing: false, snapshot: Default::default() };
    editor.update_date();
    editor
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

  fn update_date(&mut self) {
    self.snapshot.parsed_date = now();
    let (d, m, y) = (self.date.day, self.date.month, self.date.year);
    let date = chrono::NaiveDate::from_ymd_opt(y, m, d).unwrap();
    let date_utc = chrono::DateTime::<Utc>::from_local(NaiveDateTime::new(date, NaiveTime::default()), Utc);
    self.snapshot.date = date_utc;
  }

  pub fn save_to_file(&mut self) -> anyhow::Result<()> {
    let dir = &*env::export_dir();
    if Path::new(dir).metadata().is_err() {
      fs::create_dir(dir).unwrap();
    }

    let filename = format!("{}/{}.json", dir, self.snapshot.uid);
    let file = File::create(&filename).unwrap();
    let writer = BufWriter::new(file);
    serde_json::to_writer_pretty(writer, &self.snapshot).unwrap();
    println!("Exported to {}", filename);
    Ok(())
  }

  pub fn set_groups(&mut self, groups: Vec<Group>) {
    self.snapshot.groups = groups;
  }

  pub fn load_from_default(&self, day: &DefaultDay) -> Vec<Group> {
    let is_even = self.snapshot.date.iso_week().week0() % 2 == 0;
    let mut res = day
      .groups
      .iter()
      .map(|g| Group {
        name: g.name.clone(),
        lessons: g
          .lessons
          .iter()
          .filter(|l| !matches!(l.is_even, Some(x) if x != is_even))
          .map(|l| Lesson {
            num: Some(l.num),
            name: l.name.clone(),
            subgroup: l.subgroup,
            teacher: l.teacher.clone(),
            classroom: l.classroom.clone(),
          })
          .collect(),
        uid: "".into(),
      })
      .collect::<Vec<Group>>();
    res.iter_mut().for_each(|g| g.uid = g.uid());
    res
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
        self.is_date_editing = false;
        self.update_date();
      }
      Message::DatePickCancel => self.is_date_editing = false,
    }
    self.snapshot.uid = self.snapshot.uid();
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
