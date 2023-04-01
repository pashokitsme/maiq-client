use iced::{widget::row, Length};
use iced_aw::{
  menu::{MenuBar, MenuTree},
  Icon,
};

use crate::app::AppMessage;

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
      menu_button(with_icon("Новый", Icon::FileEarmark), AppMessage::Dummy),
      menu_button(with_icon("Импорт", Icon::Download), AppMessage::Dummy),
      menu_button(with_icon("Экспорт", Icon::Upload), AppMessage::Dummy),
    ],
  )
}

fn edit_menu<'a>() -> MenuTree<'a, Message, iced::Renderer> {
  MenuTree::with_children(
    with_icon("Редактировать", Icon::PencilSquare),
    vec![menu_button(with_icon("Сортировка", Icon::ArrowRepeat), AppMessage::Dummy)],
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
