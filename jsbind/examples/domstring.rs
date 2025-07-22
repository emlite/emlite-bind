use jsbind::prelude::*;

fn main() {
    let s: DOMString = "Hello".into();
    Console::get().log(&[s.clone().into()]);
    println!("{}", s.as_str());
}
