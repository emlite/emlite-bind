use crate::utils::bind;
use crate::{Any, Function}; // same bind! macro you used before

/// 1-to-1 wrapper around a JavaScript `Promise`.
#[derive(Clone)]
pub struct Promise {
    inner: emlite::Val,
}

bind!(Promise); // ⇢ FromVal / Into<Val> / take_ownership …

impl Default for Promise {
    /// Constructs `undefined`, identical to the C++ default ctor.
    fn default() -> Self {
        emlite::Val::undefined().as_::<Self>()
    }
}

impl Promise {
    /* ─────────── combinators ─────────── */

    /// JS `Promise.prototype.then`.
    pub fn then(&self, on_fulfilled: &Function, on_rejected: &Function) -> Self {
        self.inner
            .call(
                "then",
                &[on_fulfilled.clone().into(), on_rejected.clone().into()],
            )
            .as_::<Self>()
    }

    /// JS `Promise.prototype.catch`.  (`catch` is a reserved keyword; keep C++
    /// name `_catch` for symmetry.)
    pub fn catch(&self, on_rejected: &Function) -> Self {
        self.inner
            .call("catch", &[on_rejected.clone().into()])
            .as_::<Self>()
    }

    /// JS `Promise.prototype.finally`.
    pub fn finally(&self, on_finally: &Function) -> Self {
        self.inner
            .call("finally", &[on_finally.clone().into()])
            .as_::<Self>()
    }

    /* ─────────── static helpers ─────────── */

    /// JS `Promise.resolve(value?)`
    pub fn resolve(value: &Any) -> Self {
        emlite::Val::global("Promise")
            .call("resolve", &[value.clone()])
            .as_::<Self>()
    }

    /// JS `Promise.reject(reason?)`
    pub fn reject(reason: &Any) -> Self {
        emlite::Val::global("Promise")
            .call("reject", &[reason.clone()])
            .as_::<Self>()
    }

    pub fn await_<T>(&self) -> T
    where
        T: emlite::FromVal, // same bound `Val::as_::<T>()` uses
    {
        self.inner.await_().as_::<T>()
    }
}
