use iced::{
  widget::{button, column, container, row, scrollable, Rule},
  Length, Sandbox,
};
use maiq_shared::Group;

use crate::components::{group::GroupMessage, Component};

#[derive(Debug, Clone)]
pub enum AppMessage {
  Group((usize, GroupMessage)),
  NewGroup,
  Sort,
  Export,
}

#[derive(Default)]
pub struct App {
  groups: Vec<Group>,
}

impl App {
  fn update_group(&mut self, message: GroupMessage, idx: usize) {
    if let Some(g) = self.groups.get_mut(idx) {
      g.update(message)
    }
  }

  fn remove_group(&mut self, idx: usize) {
    self.groups.remove(idx);
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
    match message {
      AppMessage::Group((idx, GroupMessage::Remove)) => self.remove_group(idx),
      AppMessage::Group((idx, msg)) => self.update_group(msg, idx),
      AppMessage::NewGroup => self.groups.push(Group::default()),
      AppMessage::Sort => todo!(),
      AppMessage::Export => todo!(),
    };
  }

  fn view(&self) -> iced::Element<'_, Self::Message> {
    let toolbar = row![
      button("Добавить группу").on_press(AppMessage::NewGroup),
      button("Отсортировать").on_press(AppMessage::Sort),
      button("Экспорт").on_press(AppMessage::Export)
    ]
    .padding([0, 0, 5, 0])
    .spacing(15);
    let groups = scrollable(
      column(
        self
          .groups
          .iter()
          .enumerate()
          .map(|(idx, group)| group.view().map(move |msg| AppMessage::Group((idx, msg))))
          .collect(),
      )
      .padding(15),
    );

    let content = column![toolbar, Rule::horizontal(1), container(groups).width(Length::Fill).padding([15, 0, 0, 0])];

    container(content).padding(15).into()
  }
}
