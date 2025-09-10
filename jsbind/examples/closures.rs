use jsbind::prelude::*;

// Smoke examples for Closure::bind{0..4} and invocation via Function
fn main() {
    emlite::init();

    // 0-arg closure returning a number
    let c0 = Closure::bind0(|| 42);
    let f0: Function = (&c0).into();
    let r0 = f0.call(&Any::undefined(), &[]).unwrap().as_::<i32>();
    Console::get().log(&[Any::from(r0)]);

    // 1-arg closure: add 1
    let c1 = Closure::bind1(|x: i32| x + 1);
    let f1: Function = (&c1).into();
    let r1 = f1
        .call(&Any::undefined(), &[Any::from(9)])
        .unwrap()
        .as_::<i32>();
    Console::get().log(&[Any::from(r1)]);

    // 2-arg closure: sum
    let c2 = Closure::bind2(|a: i32, b: i32| a + b);
    let f2: Function = (&c2).into();
    let r2 = f2
        .call(&Any::undefined(), &[Any::from(2), Any::from(3)])
        .unwrap()
        .as_::<i32>();
    Console::get().log(&[Any::from(r2)]);

    // 3-arg closure: format string
    let c3 = Closure::bind3(|a: i32, b: i32, c: i32| format!("{}-{}-{}", a, b, c));
    let f3: Function = (&c3).into();
    let r3 = f3
        .call(
            &Any::undefined(),
            &[Any::from(1), Any::from(2), Any::from(3)],
        )
        .unwrap()
        .as_::<Option<String>>()
        .unwrap();
    Console::get().log(&[Any::from(r3)]);

    // 4-arg closure: sum of four
    let c4 = Closure::bind4(|a: i32, b: i32, c: i32, d: i32| a + b + c + d);
    let f4: Function = (&c4).into();
    let r4 = f4
        .call(
            &Any::undefined(),
            &[Any::from(1), Any::from(2), Any::from(3), Any::from(4)],
        )
        .unwrap()
        .as_::<i32>();
    Console::get().log(&[Any::from(r4)]);
}
