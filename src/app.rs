use iced::{
  widget::{column, container, scrollable, Rule},
  Length, Sandbox,
};

use crate::components::{menu::toolbar, snapshot_editor::SnapshotEditor, Component, EditorMessage};

#[derive(Debug, Clone)]
pub enum AppMessage {
  Editor(EditorMessage),
  Sort,
  Import,
  Export,
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
    match message {
      AppMessage::Editor(m) => self.editor.update(m),
      AppMessage::Import => todo!(),
      AppMessage::Sort => todo!(),
      AppMessage::Export => todo!(),
      AppMessage::Dummy => println!("Not implemented"),
    };
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
