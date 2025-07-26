use jsbind::prelude::*;

fn main() {
    // Create Any from different Rust types
    let num = Any::from(123);
    let text = Any::from("hello");
    let boolean = Any::from(true);
    println!("Any from int: {:?}", num);
    println!("Any from str: {:?}", text);
    println!("Any from bool: {:?}", boolean);
    Console::get().log(&[num.clone(), text.clone(), boolean.clone()]);

    // Convert Any back to Rust types using as_::<T>()
    let n: i32 = num.as_();
    let s: Option<String> = text.as_();
    let b: bool = boolean.as_();
    println!("Extracted i32: {}", n);
    println!("Extracted String: {:?}", s);
    println!("Extracted bool: {}", b);
}
