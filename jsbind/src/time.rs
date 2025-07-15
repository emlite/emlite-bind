use crate::any::Any;
use alloc::vec::Vec;
use crate::function::Function;

pub struct Performance;
impl Performance {
    /// High-resolution time in milliseconds since the page (or worker)
    /// was created (akin to `core::time::Instant`).
    #[inline]
    pub fn now() -> f64 {
        emlite::Val::global("performance")
            .call("now", &[])
            .as_::<f64>()
    }
}

/// `setTimeout(cb, ms, ...args)` → returns timer id (`i32` in browsers).
pub fn set_timeout(cb: &Function, millis: i32, extra_args: &[Any]) -> i32 {
    let mut args: Vec<emlite::Val> = Vec::with_capacity(extra_args.len() + 2);
    args.push(cb.clone().into());
    args.push(millis.into());
    args.extend(extra_args.iter().cloned());
    emlite::Val::global("setTimeout").invoke(&args).as_::<i32>()
}

/// `clearTimeout(id)`
#[inline]
pub fn clear_timeout(id: i32) {
    emlite::Val::global("clearTimeout").call("", &[id.into()]);
}

/// `setInterval(cb, ms, ...args)` → returns id
pub fn set_interval(cb: &Function, millis: i32, extra_args: &[Any]) -> i32 {
    let mut args: Vec<emlite::Val> = Vec::with_capacity(extra_args.len() + 2);
    args.push(cb.clone().into());
    args.push(millis.into());
    args.extend(extra_args.iter().cloned());
    emlite::Val::global("setInterval")
        .invoke(&args)
        .as_::<i32>()
}

/// `clearInterval(id)`
#[inline]
pub fn clear_interval(id: i32) {
    emlite::Val::global("clearInterval").call("", &[id.into()]);
}
