use app::App;
use iced::{Application, Settings};
use include_dir::{include_dir, Dir};

mod app;
mod components;

static ASSETS: Dir<'static> = include_dir!("$CARGO_MANIFEST_DIR/assets");

fn main() {
  _ = App::run(Settings { default_font: ASSETS.get_file("Roboto.ttf").map(|f| f.contents()), ..Settings::default() });
}
