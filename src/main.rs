use app::App;
use iced::{Sandbox, Settings};
use include_dir::{include_dir, Dir};

mod app;
mod env;
mod view;

static ASSETS: Dir<'static> = include_dir!("$CARGO_MANIFEST_DIR/assets");

fn main() {
  env::init();
  // pretty_env_logger::init();
  _ = App::run(Settings { default_font: ASSETS.get_file("Roboto.ttf").map(|f| f.contents()), ..Settings::default() });
}
