use jsbind::prelude::*;
use webbind::html_button_element::HTMLButtonElement;
use webbind::node::Node;
use webbind::pointer_event::PointerEvent;
use webbind::window;

fn main() {
    let con = Console::get();
    let document = window().document();
    let bodies = document.get_elements_by_tag_name("body");
    if bodies.length() == 0 {
        con.log(&["I Ain't got Nobody!".into()]);
        return;
    }
    let body = bodies.item(0);
    let mut button = document
        .create_element0("BUTTON")
        .dyn_into::<HTMLButtonElement>()
        .unwrap();

    let style = button.style();
    style.set_property0("color", "red");
    style.set_property0("background-color", "#aaf");
    style.set_property0("border", "solid");

    button.set_text_content("Click me");
    button.add_event_listener0(
        "click",
        &Closure::bind1(move |p: PointerEvent| {
            con.log(&[p.client_x().into()]);
        })
        .into(),
    );
    body.append_child(button.dyn_ref::<Node>().unwrap());
}
