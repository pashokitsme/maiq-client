use chrono::Weekday;
use iced::{
  alignment::{Horizontal, Vertical},
  widget::{container, row, text, Rule},
  Length,
};
use iced_aw::{
  menu::{MenuBar, MenuTree},
  Icon, ICON_FONT,
};
use maiq_shared::default::DefaultDay;

use crate::{app::AppMessage, env::DEFAULTS};

use super::with_icon;

type Message = AppMessage;
type Element<'a> = iced::Element<'a, Message, iced::Renderer>;

fn menu_button<'a>(content: impl Into<Element<'a>>, message: Message) -> MenuTree<'a, Message, iced::Renderer> {
  MenuTree::new(super::basic_button(content, message).width(Length::Fill))
}

fn file_menu<'a>() -> MenuTree<'a, Message, iced::Renderer> {
  MenuTree::with_children(
    with_icon("Файл", Icon::FileEarmark),
    vec![
      menu_button(with_icon("Новый", Icon::FileEarmark), AppMessage::New),
      import_menu(&DEFAULTS),
      menu_button(with_icon("Экспорт", Icon::Upload), AppMessage::Export),
    ],
  )
}

fn import_menu<'a>(default: &[DefaultDay]) -> MenuTree<'a, Message, iced::Renderer> {
  let mut childs = vec![
    menu_button(with_icon("Из файла", Icon::Calendar), AppMessage::ImportFromFile),
    MenuTree::new(
      text("Стандартное")
        .vertical_alignment(Vertical::Center)
        .horizontal_alignment(Horizontal::Center)
        .height(Length::Fill)
        .width(Length::Fill),
    ),
    menu_button(with_icon("Сегодня", Icon::Calendar), AppMessage::ImportToday),
    menu_button(with_icon("Завтра", Icon::Calendar), AppMessage::ImportNext)
  ];
  default
    .iter()
    .enumerate()
    .map(|(idx, d)| menu_button(with_icon(map_weekday_to_str(d.day), Icon::Calendar), AppMessage::Import(idx)))
    .for_each(|menu| childs.push(menu));

  MenuTree::with_children(
    super::basic_button(
      row![with_icon("Импорт", Icon::Download), text(Icon::ChevronBarRight).font(ICON_FONT)],
      AppMessage::Nothing,
    )
    .width(Length::Fill),
    childs,
  )
}

fn edit_menu<'a>() -> MenuTree<'a, Message, iced::Renderer> {
  MenuTree::with_children(
    with_icon("Редактировать", Icon::PencilSquare),
    vec![menu_button(with_icon("Сортировка", Icon::ArrowRepeat), AppMessage::Sort)],
  )
}

fn server_menu<'a>() -> MenuTree<'a, Message, iced::Renderer> {
  MenuTree::with_children(
    with_icon("Сервер", Icon::Cloud),
    vec![
      menu_button(with_icon("API токен", Icon::Shield), AppMessage::Dummy),
      menu_button(with_icon("Опубликовать", Icon::CloudUpload), AppMessage::Dummy),
    ],
  )
}

pub fn toolbar<'a>() -> Element<'a> {
  row![MenuBar::new(vec![file_menu(), edit_menu(), server_menu()]).spacing(10.0)].into()
}

const fn map_weekday_to_str(d: Weekday) -> &'static str {
  match d {
    Weekday::Mon => "Понедельник",
    Weekday::Tue => "Вторник",
    Weekday::Wed => "Среда",
    Weekday::Thu => "Четверг",
    Weekday::Fri => "Пятница",
    Weekday::Sat => "Суббота",
    Weekday::Sun => "Воскресенье",
  }
}
