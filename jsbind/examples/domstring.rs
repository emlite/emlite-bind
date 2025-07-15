use jsbind::prelude::*;

fn main() {
    let s: DOMString = "Hello".into();
    Console::get().log(&[s.into()]);
}
