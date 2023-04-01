use anyhow::anyhow;
use iced::{
  widget::{column, container, scrollable, Rule},
  Length, Sandbox,
};

use crate::{
  env::DEFAULTS,
  view::{editor::SnapshotEditor, toolbar::toolbar, Component, EditorMessage},
};

#[derive(Debug, Clone)]
pub enum AppMessage {
  Editor(EditorMessage),
  Sort,
  Import(usize),
  Export,
  New,
  Dummy,
}

#[derive(Default)]
pub struct App {
  editor: SnapshotEditor,
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
    info!("Message: {:?}", message);
    let res = match message {
      AppMessage::Editor(m) => {
        self.editor.update(m);
        Ok(())
      }
      AppMessage::Import(idx) => {
        self.editor.set_groups(self.editor.load_from_default(&DEFAULTS[idx]));
        Ok(())
      }
      AppMessage::Sort => todo!(),
      AppMessage::Export => self.editor.save_to_file(),
      AppMessage::New => {
        self.editor.new_file();
        Ok(())
      }
      _ => Err(anyhow!("Not yet implemented!")),
    };

    if let Err(err) = res {
      eprintln!("{}", err);
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
      .padding(15),
    );

    let content = column![
      toolbar(),
      Rule::horizontal(1),
      container(self.editor.view().map(AppMessage::Editor)).padding([10, 0, 0, 0]),
      Rule::horizontal(1),
      container(groups).width(Length::Fill).padding([15, 0, 0, 0])
    ];

    container(content).padding(5).into()
  }
}
