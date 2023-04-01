use std::{fs, str::FromStr};

use lazy_static::lazy_static;
use maiq_shared::default::DefaultDay;

pub fn parse_var<T: FromStr>(var: &'static str) -> Option<T> {
  self::var(var).and_then(|x| x.parse().ok())
}

pub fn var(var: &'static str) -> Option<String> {
  dotenvy::var(var).ok()
}

pub fn check<T: FromStr>(var: &'static str) -> bool {
  parse_var::<T>(var)
    .is_none()
    .then(|| println!("Var {}: {} is not present", var, std::any::type_name::<T>().split("::").last().unwrap()))
    .is_none()
}

lazy_static! {
  pub static ref DEFAULTS: Vec<DefaultDay> = {
    fn read(path: &String) -> Option<DefaultDay> {
      fs::read_to_string(path).ok().map(|content| {
        serde_json::from_str(content.as_str()).unwrap_or_else(|_| panic!("Can't parse default timetable from `{}`", &path))
      })
    }

    ["mon", "tue", "wed", "thu", "fri", "sat"]
      .iter()
      .map(|f| {
        let path = format!("default/{}.json", f);
        (read(&path), path)
      })
      .filter(|(f, path)| match f {
        Some(_) => true,
        None => {
          eprintln!("warn -> no default found in {}", path);
          false
        }
      })
      .map(|f| f.0.unwrap())
      .collect::<Vec<DefaultDay>>()
  };
}

macro_rules! vars {
  [$($var_name: ident: $getter: ident -> $ty: tt),*] => {
    $(const $var_name: &'static str = stringify!($var_name);

    pub fn $getter() -> $ty {
      parse_var($var_name).unwrap()
    })*

    pub fn init() {
      print!("Reading .env.. ");
      match dotenvy::dotenv() {
        Ok(_) => println!("ok"),
        Err(err) => println!("{}", err)
      };
      let mut failed = false;
      $(failed |= !check::<$ty>($var_name);)*
      failed.then(|| panic!("Not all .env args are set") );
    }
  };
}

vars![EXPORT_DIRECTORY: export_dir -> String];
