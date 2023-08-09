#![allow(non_snake_case,non_upper_case_globals,non_camel_case_types,unused_imports,unused_mut,unused_variables,dead_code,unused_assignments,unused_macros)]

pub mod helper;
use crate::helper::*;

use std::format as f;
// todo: add enum matching keys to their enum values by string? might not be implemented
// https://docs.rs/keyboard-types/latest/keyboard_types/enum.Key.html
//
use wasm_bindgen::prelude	::*;
use web_sys              	as web;
use web                  	::console;
use web                  	::{Document,Element,HtmlElement,Window};
use web                  	::{KeyboardEvent, KeyboardEvent as KEvt};
use web                  	::{HtmlTextAreaElement,HtmlParagraphElement,HtmlButtonElement};
use gloo                 	::{events::{EventListener,EventListenerOptions}, timers::callback::Timeout};
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
  /*
  fn          	type  	info	doc: developer.mozilla.org/en-US/docs/Web/API/KeyboardEvent/
  key         	String	.   	/key
  location    	u32   	.   	/location
  is_composing	bool  	.   	/isComposing
  char_code   	u32   	.   	/charCode ← deprecated
  key_code    	u32   	.   	/keyCode ← deprecated
   */


/*
todo:
get url text
ad json table to JS script so that users can edit them
read that table in wasm
add my parser to read ⇧a as a key combo
matche to predefined regex in a hashtable
if matches, get the value from the table
value is a list of keyss
match if each key combo matches, and if so, so the action - don't let the website hijack
 */


#[wasm_bindgen(start)] fn run() -> Result<(),JsValue> {
  #[cfg(debug_assertions)]console_error_panic_hook::set_once(); // better error messages in debug mode, disabled in release mode so it doesn't bloat up the file size
  // let _ = evt_listener_keydown_gloo();
  p!("lkasjf");
  Ok(())
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
      NA   	=> write!(f,"{}", format!("NA with ‘{}’"   	,NA   	.val())),
      Win  	=> write!(f,"{}", format!("Win with ‘{}’"  	,Win  	.val())),
      Mac  	=> write!(f,"{}", format!("Mac with ‘{}’"  	,Mac  	.val())),
      Linux	=> write!(f,"{}", format!("Linux with ‘{}’"	,Linux	.val())),
    }
  }
}

fn evt_listener_keydown_gloo(){ // create a keydown event using the EventListener Struct and bind it with a HTML textarea element. Whenever a key is entered with the keyboard on the textarea, the keyboard event type and key is set as the text content of a paragraph element
  let win 	= web::window() .expect("should have a window in this context");
  let doc 	= win.document().expect("window should have a document");
  let body	= doc.body()    .expect("document should have a body");
  // let body: &web_sys::Node = body.as_ref(); // needed?
  let navigator 	= win.navigator();
  let user_agent	= navigator.user_agent();

  use UserAgentOS as OS;
  let mut user_os = UserAgentOS::NA;
  match &user_agent {
    Ok(agent_s) => { for os in all::<UserAgentOS>().collect::<Vec<_>>() {
      let os_s = os.val();
      let is_match = agent_s.to_lowercase()
        .matches(os_s.to_lowercase().as_str())
        .next().is_some();
      if is_match {user_os = os; break}
      }}
    Err(_) => panic!("Failed to get user agent needed to get browser OS!"),
  };
  let user_agent_s	= user_agent.clone().unwrap();
  let user_os_s = user_os.val();
  p!("identified os as enum: {user_os}");
  p!("identified os as value: {user_os_s}");
  p!("from user_agent_s: {user_agent_s}");
  let mut keybinds = [("ctrl","a")];
  match user_os {
    OS::Win  	=> keybinds = [("ctrl","a")],
    OS::Mac  	=> keybinds = [("cmd","a")],
    OS::Linux	=> keybinds = [("ctrl","a")],
    _        	=> (),
  };

  let txt:HtmlTextAreaElement 	= doc.create_element("textarea"	).unwrap().dyn_into().unwrap(); //get_element_by_id
  let msg:HtmlParagraphElement	= doc.create_element("p"       	).unwrap().dyn_into().unwrap();

  txt.set_text_content(Some("Enter text here to detect Keyboard events."));
  // style
  txt.set_rows(5); txt.set_cols(70);
  msg.style().set_property("white-space","pre-line").map_err(|_|()).unwrap(); //Text will wrap when necessary, and on line breaks
  // attach
  body.append_child(&msg).unwrap();
  body.append_child(&txt).unwrap();

  // Handling the Left and Right Alt Keys
  // Keyboard Layout	key       	code      	Notes
  // US             	"Alt"     	"AltLeft" 	DOM_KEY_LOCATION_LEFT
  // French         	"Alt"     	"AltLeft" 	DOM_KEY_LOCATION_LEFT
  // US             	"Alt"     	"AltRight"	DOM_KEY_LOCATION_RIGHT
  // French         	"AltGraph"	"AltRight"	DOM_KEY_LOCATION_RIGHT

  // https://rustwasm.github.io/wasm-bindgen/api/web_sys/struct.KeyboardEvent.html
  // todo: change from txt to document to track any keybinds
  let listen_opt = EventListenerOptions::enable_prevent_default();
  let on_keydown = EventListener::new_with_options(&txt, "keydown", listen_opt, move |evt| {
    let kevt = evt.clone().dyn_into::<web_sys::KeyboardEvent>().unwrap();
    let mut evt_s = String::from("");

    evt_s.push_str(&"global val\t¦");
    evt_s.push_str(&user_os_s);
    evt_s.push_str(&"¦\nglobal val\t¦");
    evt_s.push_str(&user_agent_s);
    evt_s.push_str(&"¦\nevent type\t¦");
    evt_s.push_str(&evt.type_()); // type field: keydown/keyup/...
    evt_s.push_str(&"¦\nkey\t¦");
    let k = kevt.key();
    evt_s.push_str(match &*k	{ // get &str out of the String to match
      "Shift" => "⇧",
      "Dead"	=> "☠",
      _     	=> &k
    });
    // evt_s.push_str(&KEvt::key(&kevt)); // &evt_kbd.key()
      /* str key pressed by the user, taking into consideration the state of modifier keys such as Shift as well as the keyboard locale and layout
        zZ яЯ               	printable representation (⇧z=Z)
        Dead                	dead key
        control/special char	Alt, ... Unidentified developer.mozilla.org/en-US/docs/Web/API/UI_Events/Keyboard_event_key_values
       */
    let phys_key = &kevt.phys_key();
    let isMeta = &kevt.isMeta();
    let isAlt = &kevt.isAlt();
    // todo: convert modifiers from source to bitfield
    // todo: convert modifiers from event to bitfield
    // compare
    // preventDefault() method of the Event interface tells the user agent that if the event does not get explicitly handled, its default action should not be taken as it normally would be (can prevent typing)
    if  phys_key == "Digit1"{
      p!("you pressed 1, stop propagation, block default, return");
      evt.stop_immediate_propagation();evt.prevent_default(); return;
    };
    if  phys_key == "Digit2"{
      p!("you pressed 2, stop propagation, block default, return");
      evt.stop_immediate_propagation();evt.prevent_default(); return;
    };
    if  phys_key == "Digit1" && *isMeta{
      p!("you pressed ◆1, stop propagation, block default, return");
      evt.stop_immediate_propagation();evt.prevent_default(); return;
    };
    if  phys_key == "Digit2" && *isMeta{
      p!("you pressed ◆2, stop propagation, block default, return");
      evt.stop_immediate_propagation();evt.prevent_default(); return;
    };
    if  phys_key == "Digit5" && *isMeta{
      evt_s.push_str(&"¦\nyou pressed ◆5, stopping propagation, but not returning\t¦");
      p!("you pressed ◆5, stopping propagation, but not returning");
      evt.stop_immediate_propagation();
    };
    if  phys_key == "Digit6" && *isMeta{
      evt_s.push_str(&"¦\nyou pressed ◆6, stopping propagation, and returning\t¦");
      p!("you pressed ◆6, stopping propagation, and returning");
      evt.stop_immediate_propagation();return
    };
    if  phys_key == "Digit1" && *isAlt{
      evt_s.push_str(&"¦\nyou pressed ⎇1\t¦");
    };
    evt_s.push_str(&"¦\ncode=phys_key\t¦");
    evt_s.push_str(&phys_key); // KeyA physical key (not generated char) not altered ⇧mods
    evt_s.push_str(&"¦\nlocation\t¦");
    let location_string = match &kevt.location() {
      &KEvt::DOM_KEY_LOCATION_STANDARD	=> "std".to_string(),
      &KEvt::DOM_KEY_LOCATION_LEFT    	=> "‹".to_string(),
      &KEvt::DOM_KEY_LOCATION_RIGHT   	=> "›".to_string(),
      &KEvt::DOM_KEY_LOCATION_NUMPAD  	=> "🔢".to_string(),
      _                               	=> "🛑".to_string(),
    };
    evt_s.push_str(&location_string);
    evt_s.push_str(&"¦\nmodifiers           	\t¦");
    evt_s.push_str(match kevt.isShift()     	{true=>&"⇧"	,false=>" "});
    evt_s.push_str(match kevt.isCtrl()      	{true=>&"⎈"	,false=>" "});
    evt_s.push_str(match kevt.isMeta()      	{true=>&"◆"	,false=>" "});
    evt_s.push_str(match kevt.isAlt()       	{true=>&"⎇"	,false=>" "});
    evt_s.push_str(&"¦\nrep/compose         	\t¦");
    evt_s.push_str(match kevt.isHeld()      	{true=>&"🠿"	,false=>" "}); //held down (doesn't need to be repeated, can be apple's tooltip with symbols 🔁)
    evt_s.push_str(match kevt.is_composing()	{true=>&"⎀"	,false=>" "}); //event is fired within a composition session (after compositionstart and before compositionend.)

    msg.set_text_content(Some(&evt_s)); // not html, so no <br/>
  });
  on_keydown.keep_alive();
}
