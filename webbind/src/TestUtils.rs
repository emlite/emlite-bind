use super::*;

pub fn gc() -> Promise<Undefined> {
    Any::global("TestUtils")
        .call("gc", &[])
        .as_::<Promise<Undefined>>()
}
