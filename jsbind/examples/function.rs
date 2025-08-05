use jsbind::prelude::*;

fn main() {
    emlite::init();
    // Create a JS function: function add(a, b) { return a + b; }
    let f = Function::new(&["a", "b"], "return a + b;");
    println!("Created Function: {:?}", f);

    // Call the function with arguments (this_arg is undefined)
    let result = f.call(&Any::undefined(), &[Any::from(1), Any::from(2)]);
    println!("Function call result: {:?}", result);
    Console::get().log(&[result]);
}
