#![allow(non_snake_case,non_upper_case_globals,non_camel_case_types,unused_imports,unused_mut,unused_variables,dead_code,unused_assignments,unused_macros)]

use wasm_bindgen::prelude	::*;

// bind `console.log` manually, without the help of `web_sys`
#[wasm_bindgen] extern "C" {  // manual `#[wasm_bindgen]` annotations
  #[wasm_bindgen(js_namespace=console)]	pub fn log(s:&str);	// `js_namespace` binds `console.log(..)` instead of just `log(..)`
}

#[macro_export] macro_rules! p { // macro that's like `println!` (std eats all output, so it doesn't work directly), only it works for `console.log`. To get `println!`-like behavior in your app you'll likely want a macro like this
  ($($t:tt)*) => (log(&format_args!($($t)*).to_string()))
}
pub(crate) use p; //macro system runs before the module system, this exposes macros elsewhere
// stackoverflow.com/a/31749071/20361194
