use std::str::FromStr;

pub fn parse_var<T: FromStr>(var: &'static str) -> Option<T> {
  self::var(var).and_then(|x| x.parse().ok())
}

pub fn var(var: &'static str) -> Option<String> {
  dotenvy::var(var).ok()
}

pub fn check<T: FromStr>(var: &'static str) -> bool {
  parse_var::<T>(var)
    .is_none()
    .then(|| info!("Var {}: {} is not present", var, std::any::type_name::<T>().split("::").last().unwrap()))
    .is_none()
}

macro_rules! vars {
  [$($var_name: ident: $getter: ident -> $ty: tt),*] => {
    $(const $var_name: &'static str = stringify!($var_name);

    pub fn $getter() -> $ty {
      parse_var($var_name).unwrap()
    })*

    pub fn init() {
      info!("Reading .env.. ");
      match dotenvy::dotenv() {
        Ok(_) => info!("ok"),
        Err(err) => info!("{}", err)
      };
      let mut failed = false;
      $(failed |= !check::<$ty>($var_name);)*
      failed.then(|| {
        error!("Not all .env args are set");
        panic!("Not all .env args are set");
      });
    }
  };
}

vars![EXPORT_DIRECTORY: export_dir -> String];
