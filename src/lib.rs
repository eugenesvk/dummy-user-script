#![allow(non_snake_case,non_upper_case_globals,non_camel_case_types,unused_imports,unused_mut,unused_variables,dead_code,unused_assignments,unused_macros)]
use std::format as f;

use wasm_bindgen::prelude	::*;
use web_sys              	as web;
use web                  	::console;
use web                  	::{Document,Element,HtmlElement,Window};
use web                  	::{KeyboardEvent, KeyboardEvent as KEvt};
use gloo                 	::{events::EventListener, timers::callback::Timeout};
  // gloo::events::EventListener › RAII type which is used to manage DOM event listeners. When the EventListener is dropped, it will automatically deregister the event listener and clean up the closure’s memory

pub trait EventListenerAlias { // add wrapper trait to allow using better names
  fn keep_alive(self);
  }
impl      EventListenerAlias    for EventListener {
  fn keep_alive(mut self) {self.forget()}
  //fn   forget(mut self) {self.callback.take().unwrap_throw().forget()}
  }

pub trait KeyboardEventAlias { // add wrapper trait to allow using better names
  fn phys_key	(&self) -> String;
  fn phys    	(&self) -> String;
  fn isShift 	(&self) -> bool;
  fn isCtrl  	(&self) -> bool;
  fn isMeta  	(&self) -> bool;
  fn isAlt   	(&self) -> bool;
  fn isHeld  	(&self) -> bool;
  }
impl      KeyboardEventAlias    for KeyboardEvent {
  fn phys_key	(&self) -> String {self.code     	()}
  fn phys    	(&self) -> String {self.code     	()}
  fn isShift 	(&self) -> bool   {self.shift_key	()}
  fn isCtrl  	(&self) -> bool   {self.ctrl_key 	()}
  fn isMeta  	(&self) -> bool   {self.meta_key 	()}
  fn isAlt   	(&self) -> bool   {self.alt_key  	()}
  fn isHeld  	(&self) -> bool   {self.repeat   	()}
  }

#[wasm_bindgen(start)] fn run() -> Result<(),JsValue> {
  #[cfg(debug_assertions)]console_error_panic_hook::set_once(); // better error messages in debug mode, disabled in release mode so it doesn't bloat up the file size
  let _ = evt_listener_keydown_gloo();

  Ok(())
}

// bind `console.log` manually, without the help of `web_sys`
#[wasm_bindgen] extern "C" {  // manual `#[wasm_bindgen]` annotations
  #[wasm_bindgen(js_namespace=console            )]	fn log     (s:&str);	// `js_namespace` binds `console.log(..)` instead of just `log(..)`
}
macro_rules! p { // macro that's like `println!` (std eats all output, so it doesn't work directly), only it works for `console.log`. To get `println!`-like behavior in your app you'll likely want a macro like this
  ($($t:tt)*) => (log(&format_args!($($t)*).to_string()))
}

use enum_iterator::{all, cardinality, first, last, next, previous, reverse_all, Sequence};
#[derive(Debug,Clone,PartialEq,Sequence)] pub enum UserAgentOS {Win,Mac,Linux,NA}
impl UserAgentOS {
  const fn val(&self) -> &'static str {
    use UserAgentOS::*;
    match self {
      NA   	=> "unknown",
      Win  	=> "windows ",
      Mac  	=> "mac os x",
      Linux	=> "linux",
    }
  }
}
use std::fmt;
impl fmt::Display for UserAgentOS {
  fn fmt(&self, f:&mut fmt::Formatter) -> fmt::Result {
    use UserAgentOS::*;
    match self {
      NA   	=> write!(f,"{}", "NA:"   	.to_owned()	+ (NA   	.val())),
      Win  	=> write!(f,"{}", "Win:"  	.to_owned()	+ (Win  	.val())),
      Mac  	=> write!(f,"{}", "Mac:"  	.to_owned()	+ (Mac  	.val())),
      Linux	=> write!(f,"{}", "Linux:"	.to_owned()	+ (Linux	.val())),
    }
  }
}

fn evt_listener_keydown_gloo(){ // create a keydown event using the EventListener Struct and bind it with a HTML textarea element. Whenever a key is entered with the keyboard on the textarea, the keyboard event type and key is set as the text content of a paragraph element
  let win 	= web::window() .expect("should have a window in this context");
  let doc 	= win.document().expect("window should have a document");
  let body	= doc.body()    .expect("document should have a body");
}
