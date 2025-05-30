use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
  #[wasm_bindgen(js_namespace = console)]
  pub fn log(s: &str);

  #[wasm_bindgen(js_namespace = console)]
  pub fn error(s: &str);
}

#[macro_export]
macro_rules! clog {
  ($($t:tt)*) => (log(&format_args!($($t)*).to_string()));
}

#[macro_export]
macro_rules! cerr {
  ($($t:tt)*) => (error(&format_args!($($t)*).to_string()));
}