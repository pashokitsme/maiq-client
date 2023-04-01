use anyhow::anyhow;
use iced::{
  widget::{column, container, row, scrollable, Rule},
  Length, Sandbox,
};

use crate::{
  env::DEFAULTS,
  view::{editor::SnapshotEditor, notification::Notification, toolbar::toolbar, Component, EditorMessage},
};

#[derive(Debug, Clone)]
pub enum AppMessage {
  Editor(EditorMessage),
  Sort,
  Import(usize),
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
      AppMessage::Import(idx) => self.editor.set_groups(self.editor.load_from_default(&DEFAULTS[idx])),
      AppMessage::Sort => self.editor.sort(),
      AppMessage::Export => self.editor.save_to_file(),
      AppMessage::DeleteNotification(idx) => {
        self.notifications.remove(idx);
        Ok(None)
      }
      AppMessage::New => self.editor.set_groups(vec![]),
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
    let notifications_container = scrollable(
      container(
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
        .padding([0, 0]),
      )
      .padding(10),
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
