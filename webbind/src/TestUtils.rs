use super::*;


/// The gc function from the TestUtils namespace.
pub fn gc() -> Promise<Undefined> {
    Any::global("TestUtils").call("gc", &[]).as_::<Promise<Undefined>>()
}

