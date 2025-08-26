use crate::{any::Any, array::Array, function::Function};
use core::marker::PhantomData;
use core::ops::{Deref, DerefMut};
use emlite::FromVal;

/// JavaScript [`Promise`](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Promise) wrapper.
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct Promise<T> {
    inner: emlite::Val,
    phantom: PhantomData<T>,
}

impl<T> emlite::FromVal for Promise<T> {
    fn from_val(v: &emlite::Val) -> Self {
        Self {
            inner: v.clone(),
            phantom: PhantomData,
        }
    }
    fn take_ownership(v: emlite::common::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::common::Handle {
        self.inner.as_handle()
    }
}

impl<T> From<Promise<T>> for emlite::Val {
    fn from(x: Promise<T>) -> emlite::Val {
        let handle = x.inner.as_handle();
        core::mem::forget(x);
        emlite::Val::take_ownership(handle)
    }
}

impl<T> From<&Promise<T>> for emlite::Val {
    fn from(x: &Promise<T>) -> emlite::Val {
        x.inner.clone()
    }
}

impl<T> Deref for Promise<T> {
    type Target = emlite::Val;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl<T> DerefMut for Promise<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl<T> AsRef<emlite::Val> for Promise<T> {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}

impl<T> AsMut<emlite::Val> for Promise<T> {
    fn as_mut(&mut self) -> &mut emlite::Val {
        &mut self.inner
    }
}

impl<T> Default for Promise<T> {
    fn default() -> Self {
        emlite::Val::global("Promise").new(&[]).as_::<Self>()
    }
}

impl<T> Promise<T> {
    /// JavaScript `Promise.all(iterable)`
    pub fn all(iterable: &Array) -> Self {
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
    pub fn await_(&self) -> T
    where
        T: emlite::FromVal,
    {
        self.inner.await_().as_::<T>()
    }
}

impl<T> crate::prelude::DynCast for Promise<T> {
    #[inline]
    fn instanceof(val: &Any) -> bool {
        let ctor = Any::global("Promise");
        val.instanceof(ctor)
    }
    #[inline]
    fn unchecked_from_val(v: Any) -> Self {
        v.as_::<Self>() // zero-cost new-type cast
    }
    #[inline]
    fn unchecked_from_val_ref(v: &Any) -> &Self {
        unsafe { &*(v as *const Any as *const Self) }
    }
    #[inline]
    fn unchecked_from_val_mut(v: &mut Any) -> &mut Self {
        unsafe { &mut *(v as *mut Any as *mut Self) }
    }
}
