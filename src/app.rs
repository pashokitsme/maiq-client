use anyhow::anyhow;
use chrono::{Datelike, Weekday};
use iced::{
  widget::{column, container, row, scrollable, Rule},
  Length, Sandbox,
};
use maiq_shared::utils::time::{now_date, now_date_offset};

use crate::{
  env::DEFAULTS,
  view::{editor::SnapshotEditor, notification::Notification, toolbar::toolbar, Component, EditorMessage},
};

#[derive(Debug, Clone)]
pub enum AppMessage {
  Editor(EditorMessage),
  Sort,
  Import(usize),
  ImportToday,
  ImportNext,
  Export,
  New,
  DeleteNotification(usize),
  Dummy,
  Nothing,
}

#[derive(Default)]
pub struct App {
  editor: SnapshotEditor,
  notifications: Vec<Notification>,
}

fn resolve_weekday(today: bool) -> Weekday {
  let date = match today {
    true => now_date(),
    false => now_date_offset(1),
  };
  let weekday = date.weekday();
  if weekday == Weekday::Sun {
    weekday.succ()
  } else {
    weekday
  }
}

impl Sandbox for App {
  type Message = AppMessage;

  fn new() -> Self {
    App::default()
  }

  fn title(&self) -> String {
    "Iced!".into()
  }

  fn update(&mut self, message: Self::Message) {
    println!("Message: {:?}", message);
    let res = match message {
      AppMessage::Editor(m) => {
        self.editor.update(m);
        Ok(None)
      }
      AppMessage::Import(idx) => self.editor.set_groups(&DEFAULTS[idx]),
      AppMessage::ImportToday => self
        .editor
        .set_groups(&DEFAULTS[resolve_weekday(true).number_from_monday() as usize - 1]),
      AppMessage::ImportNext => self
        .editor
        .set_groups(&DEFAULTS[resolve_weekday(false).number_from_monday() as usize - 1]),
      AppMessage::Sort => self.editor.sort(),
      AppMessage::Export => self.editor.save_to_file(),
      AppMessage::DeleteNotification(idx) => {
        self.notifications.remove(idx);
        Ok(None)
      },
      AppMessage::Nothing => Ok(None),
      _ => Err(anyhow!("Not yet implemented!")),
    };

    if let Err(err) = &res {
      self
        .notifications
        .push(Notification::error("Ошибка!", "Не реализовано"));
      eprintln!("{}", err);
    }

    if let Ok(Some(ok)) = &res {
      self.notifications.push(Notification::ok("Инфо", ok));
    }
  }

  fn view(&self) -> iced::Element<'_, Self::Message> {
    let groups = scrollable(
      column(
        self
          .editor
          .groups()
          .enumerate()
          .map(|(idx, group)| {
            group
              .view()
              .map(move |msg| AppMessage::Editor(EditorMessage::Group((idx, msg))))
          })
          .collect(),
      )
      .padding([0, 15]),
    );

    let noty_count = self.notifications.len();
    let pad = if noty_count > 0 { 20 } else { 10 };
    let notifications_container = scrollable(
      row(
        self
          .notifications
          .iter()
          .rev()
          .enumerate()
          .map(|(idx, n)| {
            n.view()
              .map(move |_| AppMessage::DeleteNotification(noty_count - (idx + 1)))
          })
          .collect(),
      )
      .spacing(10)
      .padding([10, 0, pad, 0]),
    )
    .horizontal_scroll(iced::widget::scrollable::Properties::default().margin(5));

    let content = column![
      toolbar(),
      Rule::horizontal(1),
      container(self.editor.view().map(AppMessage::Editor)).padding([10, 0, 0, 0]),
      Rule::horizontal(1),
      notifications_container,
      container(groups).width(Length::Fill).padding([0, 0, 0, 0])
    ];

    container(content).padding(5).into()
  }
}
