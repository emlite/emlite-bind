use jsbind::prelude::*;

fn main() {
    // Create a resolved Promise with a value
    let value = Any::from(42);
    let promise = Promise::<Any>::resolve(&value);
    println!("Created resolved Promise: {:?}", promise);

    // Create a rejected Promise
    let reason = Any::from("error");
    let rejected = Promise::<Any>::reject(&reason);
    println!("Created rejected Promise: {:?}", rejected);

    // Use Promise.all
    let arr = TypedArray::from(vec![
        Promise::<Any>::resolve(&Any::from(1)).into(),
        Promise::<Any>::resolve(&Any::from(2)).into(),
    ]);
    let all = Promise::<Any>::all(&arr);
    println!("Promise.all result: {:?}", all);

    // Chaining with then/catch/finally using JS function bodies
    let on_fulfilled = Function::new(
        &["value"],
        "console.log('Promise fulfilled with:', value); return 'ok';",
    );
    let on_rejected = Function::new(
        &["reason"],
        "console.log('Promise rejected with:', reason); return 'fail';",
    );
    let chained = promise.then(&on_fulfilled, &on_rejected);
    let caught = chained.catch(&on_rejected);
    let finalized = caught.finally(&on_fulfilled);
    println!("Chained Promise: {:?}", finalized);
    Console::get().log(&[finalized.into()]);

    // Await a promise (requires T: FromVal)
    // let result: Any = promise.await_();
    // println!("Promise awaited result: {:?}", result);
}
