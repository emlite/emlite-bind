use jsbind::{Closure, Console};
use webbind::html_button_element::HTMLButtonElement;
use webbind::node::Node;
use webbind::pointer_event::PointerEvent;
use webbind::window;

fn main() {
    let con = Console::get();
    let document = window().document();
    let bodies = document.get_elements_by_tag_name("body".into());
    if bodies.length() == 0 {
        con.log(&["I Ain't got Nobody!".into()]);
        return;
    }
    let body = bodies.item(0);
    let mut button = document
        .create_element0("BUTTON".into())
        .as_::<HTMLButtonElement>();
    button.set_text_content("Click me".into());
    button.add_event_listener0(
        "click".into(),
        Closure::bind1(move |p: PointerEvent| {
            con.log(&[p.client_x().into()]);
        })
        .into(),
    );
    body.append_child(button.as_::<Node>());
    let style = button.style();
    style.set_property0("color".into(), "red".into());
    style.set_property0("background-color".into(), "#aaf".into());
    style.set_property0("border".into(), "solid".into());
}
