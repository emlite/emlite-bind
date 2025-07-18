use crate::utils::*;
use crate::{any::Any, function::Function, sequence::Sequence};

/// JavaScript [`Promise`](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Promise) wrapper.
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct Promise {
    inner: emlite::Val,
}

bind!(Promise);
impl_dyn_cast!(Promise);

impl Default for Promise {
    fn default() -> Self {
        emlite::Val::global("Promise").new(&[]).as_::<Self>()
    }
}

impl Promise {
    /// JavaScript `Promise.all(iterable)`
    pub fn all(iterable: &Sequence<Any>) -> Self {
        emlite::Val::global("Promise")
            .call("all", &[iterable.clone().into()])
            .as_::<Self>()
    }
    /// Equivalent to `promise.then(onFulfilled, onRejected)`.
    pub fn then(&self, on_fulfilled: &Function, on_rejected: &Function) -> Self {
        self.inner
            .call(
                "then",
                &[on_fulfilled.clone().into(), on_rejected.clone().into()],
            )
            .as_::<Self>()
    }

    /// Equivalent to `promise.catch(onRejected)`.
    pub fn catch(&self, on_rejected: &Function) -> Self {
        self.inner
            .call("catch", &[on_rejected.clone().into()])
            .as_::<Self>()
    }

    /// Equivalent to `promise.finally(onFinally)`
    pub fn finally(&self, on_finally: &Function) -> Self {
        self.inner
            .call("finally", &[on_finally.clone().into()])
            .as_::<Self>()
    }

    /// `Promise.resolve(value)`
    pub fn resolve(value: &Any) -> Self {
        emlite::Val::global("Promise")
            .call("resolve", &[value.clone()])
            .as_::<Self>()
    }

    /// `Promise.reject(reason)`
    pub fn reject(reason: &Any) -> Self {
        emlite::Val::global("Promise")
            .call("reject", &[reason.clone()])
            .as_::<Self>()
    }

    /// Synchronously wait until the promise settles and convert the resolved
    /// value to `T`.
    ///
    /// If the promise is rejected the behaviour depends on the underlying
    /// `emlite::Val::await_()` implementation.
    pub fn await_<T>(&self) -> T
    where
        T: emlite::FromVal,
    {
        self.inner.await_().as_::<T>()
    }
}
