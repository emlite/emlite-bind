use super::*;


pub fn gc() -> Promise {
    emlite::Val::global("TestUtils").call("gc", &[]).as_::<Promise>()
}

