use std::{
  fs::{self, File},
  io::BufWriter,
  path::Path,
  slice::Iter,
};

use iced::widget::{container, row, text};
use iced_aw::Icon;
use maiq_shared::default::{DefaultDay, DefaultGroup};

use crate::env;

use super::{icon_button, Component, GroupMessage};

#[derive(Debug)]
pub struct SnapshotEditor {
  snapshot: DefaultDay,
}

#[derive(Debug, Clone)]
pub enum Message {
  CreateGroup,
  Group((usize, GroupMessage)),
}

impl Default for SnapshotEditor {
  fn default() -> Self {
    Self { snapshot: DefaultDay { day: chrono::Weekday::Mon, groups: vec![] } }
  }
}

impl SnapshotEditor {
  pub fn groups(&self) -> Iter<DefaultGroup> {
    self.snapshot.groups.iter()
  }

  pub fn update_group(&mut self, message: GroupMessage, idx: usize) {
    if let Some(g) = self.snapshot.groups.get_mut(idx) {
      g.update(message)
    }
  }

  pub fn create_group(&mut self) {
    self.snapshot.groups.push(DefaultGroup::default())
  }

  pub fn remove_group(&mut self, idx: usize) {
    self.snapshot.groups.remove(idx);
  }

  pub fn sort(&mut self) -> anyhow::Result<Option<String>> {
    self.snapshot.groups.sort_by(|a, b| a.name.cmp(&b.name));
    self
      .snapshot
      .groups
      .iter_mut()
      .for_each(|g| g.lessons.sort_by_key(|k| k.num));
    Ok(Some("Отсортировано".into()))
  }

  // fn update_date(&mut self) {
  //   let (d, m, y) = (self.date.day, self.date.month, self.date.year);
  //   let date = chrono::NaiveDate::from_ymd_opt(y, m, d).unwrap();
  //   let date_utc = chrono::DateTime::<Utc>::from_local(NaiveDateTime::new(date, NaiveTime::default()), Utc);
  //   self.snapshot.date = date_utc;
  // }

  pub fn save_to_file(&mut self) -> anyhow::Result<Option<String>> {
    let dir = &*env::export_dir();
    if Path::new(dir).metadata().is_err() {
      fs::create_dir(dir).unwrap();
    }
    // self.snapshot.uid = self.snapshot.uid();
    let filename = format!("{}/{}.json", dir, self.snapshot.day.to_string().to_lowercase());
    let file = File::create(&filename).unwrap();
    let writer = BufWriter::new(file);
    serde_json::to_writer_pretty(writer, &self.snapshot).unwrap();
    println!("Exported to {}", filename);
    Ok(Some(format!("Экспортировано в {}", filename)))
  }

  pub fn set_groups(&mut self, day: &DefaultDay) -> anyhow::Result<Option<String>> {
    self.snapshot = day.clone();
    Ok(Some(format!("Загружен: {:?}", self.snapshot.day)))
  }
}

impl Component for SnapshotEditor {
  type Message = Message;

  fn update(&mut self, message: Self::Message) {
    match message {
      Message::Group((idx, GroupMessage::Remove)) => self.remove_group(idx),
      Message::Group((idx, msg)) => self.update_group(msg, idx),
      Message::CreateGroup => self.create_group(),
    }
  }

  fn view(&self) -> iced::Element<Self::Message> {
    let content = row![icon_button(Icon::Plus).on_press(Message::CreateGroup), text(&format!("День: {}", self.snapshot.day))]
      .align_items(iced::Alignment::Center)
      .spacing(25)
      .padding(5);

    container(content).into()
  }
}
