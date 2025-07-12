use jsbind::Console;
use jsbind::DOMString;

fn main() {
    let s: DOMString = "Hello".into();
    Console::get().log(&[s.into()]);
}
