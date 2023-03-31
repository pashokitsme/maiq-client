use std::{
  fs::{self, File},
  io::BufWriter,
  path::Path,
};

use iced::{
  theme::Button,
  widget::{button, column, container, row, scrollable, text, text_input, Rule},
  Length, Sandbox,
};
use maiq_shared::{
  utils::time::{now, now_date_offset},
  Group, Snapshot, Uid,
};

use crate::components::{group::GroupMessage, Component};

#[derive(Default)]
struct SnapshotState {
  date_buf: String,
  is_date_editing: bool,
}

enum SnapshotMessage {}

#[derive(Debug, Clone)]
pub enum AppMessage {
  Group((usize, GroupMessage)),
  SetSnapshotDate(String),
  NewGroup,
  Sort,
  LoadDefault,
  Export,
  ApplySnapshotDate,
  ToggleDateEdit,
}

#[derive(Default)]
pub struct App {
  snapshot: Snapshot,
  state: SnapshotState,
}

impl App {
  fn update_group(&mut self, message: GroupMessage, idx: usize) {
    if let Some(g) = self.snapshot.groups.get_mut(idx) {
      g.update(message)
    }
  }

  fn remove_group(&mut self, idx: usize) {
    self.snapshot.groups.remove(idx);
  }

  fn export(&mut self) {
    if Path::new("export").metadata().is_err() {
      fs::create_dir("export").unwrap();
    }
    let filename = format!("export/{}.json", self.snapshot.uid);
    self.snapshot.parsed_date = now();
    let file = File::create(&filename).unwrap();
    let writer = BufWriter::new(file);
    serde_json::to_writer_pretty(writer, &self.snapshot).unwrap();
    println!("Exported to {}", filename);
  }

  fn set_snapshot_date(&self) {
    println!("123");
  }
}

impl Sandbox for App {
  type Message = AppMessage;

  fn new() -> Self {
    let mut app = App::default();
    app.snapshot.date = now_date_offset(1);
    app.snapshot.uid = app.snapshot.uid();
    app.state.date_buf = app.snapshot.date.format("%d.%m.%Y").to_string();
    app
  }

  fn title(&self) -> String {
    "Iced!".into()
  }

  fn update(&mut self, message: Self::Message) {
    self.snapshot.uid = self.snapshot.uid();
    match message {
      AppMessage::Group((idx, GroupMessage::Remove)) => self.remove_group(idx),
      AppMessage::Group((idx, msg)) => self.update_group(msg, idx),
      AppMessage::SetSnapshotDate(date) => self.state.date_buf = date,
      AppMessage::ApplySnapshotDate => self.set_snapshot_date(),
      AppMessage::NewGroup => self.snapshot.groups.push(Group::default()),
      AppMessage::LoadDefault => todo!(),
      AppMessage::Sort => todo!(),
      AppMessage::Export => self.export(),
      AppMessage::ToggleDateEdit => self.state.is_date_editing = !self.state.is_date_editing,
    };
  }

  fn view(&self) -> iced::Element<'_, Self::Message> {
    let toolbar = row![
      button("Добавить группу")
        .on_press(AppMessage::NewGroup)
        .style(Button::Secondary),
      button("Отсортировать")
        .on_press(AppMessage::Sort)
        .style(Button::Secondary),
      button("Экспорт")
        .on_press(AppMessage::Export)
        .style(Button::Secondary),
      button("Загрузить стандартное расписание")
        .on_press(AppMessage::LoadDefault)
        .style(Button::Secondary),
    ]
    .padding([0, 0, 5, 0])
    .spacing(15);
    let info = row![text(format!("UUID {}", self.snapshot.uid)),]
      .align_items(iced::Alignment::Center)
      .spacing(25);

    let info = if self.state.is_date_editing {
      info.push(
        text_input("Дата", &self.state.date_buf, AppMessage::SetSnapshotDate)
          .width(Length::Fixed(100.))
          .on_submit(AppMessage::ApplySnapshotDate),
      )
    } else {
      info.push(
        button(text(self.snapshot.date.format("%d.%m.%Y").to_string()))
          .on_press(AppMessage::ToggleDateEdit)
          .style(Button::Text),
      )
    };

    let groups = scrollable(
      column(
        self
          .snapshot
          .groups
          .iter()
          .enumerate()
          .map(|(idx, group)| group.view().map(move |msg| AppMessage::Group((idx, msg))))
          .collect(),
      )
      .padding(15),
    );

    let content = column![toolbar, info, Rule::horizontal(1), container(groups).width(Length::Fill).padding([15, 0, 0, 0])];

    container(content).padding(15).into()
  }
}
