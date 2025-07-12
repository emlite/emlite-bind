use super::*;

pub fn gc() -> jsbind::Promise {
    emlite::Val::global("TestUtils")
        .call("gc", &[])
        .as_::<jsbind::Promise>()
}
