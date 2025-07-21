use jsbind::prelude::*;

fn main() {
    // Create a new JS object
    let obj = Object::new();
    println!("Created Object: {:?}", obj);

    // Set properties
    obj.set("foo", &Any::from(123));
    obj.set("bar", &Any::from("baz"));
    println!("Object after setting properties: {:?}", obj);

    // Get properties (Object::get expects &Any)
    let foo_key = Any::from("foo");
    let bar_key = Any::from("bar");
    let foo = obj.get(&foo_key);
    let bar = obj.get(&bar_key);
    println!("foo: {:?}, bar: {:?}", foo, bar);
    Console::get().log(&[foo, bar]);
}
