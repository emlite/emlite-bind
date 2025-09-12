use jsbind::prelude::*;
use webbind::*;

fn main() {
    let context = AudioContext::new0();
    println!("Got an AudioContext");

    // Create oscillator
    let mut oscillator = context.create_oscillator();
    println!("Configuring oscillator");
    oscillator.set_type_(&OscillatorType::TRIANGLE);
    oscillator.frequency().set_value(261.63); // Middle C

    let document = window().document();
    let body = document.get_elements_by_tag_name(&"body".into()).item(0);
    let mut button = document
        .create_element0(&"BUTTON".into())
        .dyn_into::<HTMLButtonElement>()
        .unwrap();

    button.set_text_content(&"Click me".into());
    button.add_event_listener0(
        &JsString::from("click"),
        &Closure::bind1(move |_p: PointerEvent| {
            println!("Playing");
            oscillator.connect3(context.destination().unchecked_ref::<AudioParam>());
            oscillator.start1(0.0);
            println!("All done!");
        })
        .into(),
    );
    body.append_child(button.dyn_ref::<Node>().unwrap());
}
