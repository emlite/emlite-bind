use super::*;

pub fn gc() -> Promise {
    Any::global("TestUtils").call("gc", &[]).as_::<Promise>()
}
