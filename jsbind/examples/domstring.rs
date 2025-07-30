use jsbind::prelude::*;

fn main() {
    let s: JsString = "Hello".into();
    Console::get().log(&[s.clone().into()]);
    println!("{:?}", s.as_str());
}
