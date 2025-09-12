use super::*;

/// The ResizeObserver class.
/// [`ResizeObserver`](https://developer.mozilla.org/en-US/docs/Web/API/ResizeObserver)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct ResizeObserver {
    inner: Any,
}

impl FromVal for ResizeObserver {
    fn from_val(v: &Any) -> Self {
        ResizeObserver {
            inner: Any::from_val(v),
        }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for ResizeObserver {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for ResizeObserver {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for ResizeObserver {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for ResizeObserver {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}

impl From<ResizeObserver> for Any {
    fn from(s: ResizeObserver) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&ResizeObserver> for Any {
    fn from(s: &ResizeObserver) -> Any {
        s.inner.clone().into()
    }
}

jsbind::utils::impl_dyn_cast!(ResizeObserver);

impl ResizeObserver {
    /// The `new ResizeObserver(..)` constructor, creating a new ResizeObserver instance
    pub fn new(callback: &Function) -> ResizeObserver {
        Self {
            inner: Any::global("ResizeObserver")
                .new(&[callback.into()])
                .as_::<Any>(),
        }
    }
}

impl ResizeObserver {
    /// The observe method.
    /// [`ResizeObserver.observe`](https://developer.mozilla.org/en-US/docs/Web/API/ResizeObserver/observe)
    pub fn observe(&self, target: &Element) -> Undefined {
        self.inner
            .call("observe", &[target.into()])
            .as_::<Undefined>()
    }
}
impl ResizeObserver {
    /// The observe method.
    /// [`ResizeObserver.observe`](https://developer.mozilla.org/en-US/docs/Web/API/ResizeObserver/observe)
    pub fn observe_with_options(
        &self,
        target: &Element,
        options: &ResizeObserverOptions,
    ) -> Undefined {
        self.inner
            .call("observe", &[target.into(), options.into()])
            .as_::<Undefined>()
    }
}
impl ResizeObserver {
    /// The unobserve method.
    /// [`ResizeObserver.unobserve`](https://developer.mozilla.org/en-US/docs/Web/API/ResizeObserver/unobserve)
    pub fn unobserve(&self, target: &Element) -> Undefined {
        self.inner
            .call("unobserve", &[target.into()])
            .as_::<Undefined>()
    }
}
impl ResizeObserver {
    /// The disconnect method.
    /// [`ResizeObserver.disconnect`](https://developer.mozilla.org/en-US/docs/Web/API/ResizeObserver/disconnect)
    pub fn disconnect(&self) -> Undefined {
        self.inner.call("disconnect", &[]).as_::<Undefined>()
    }
}
