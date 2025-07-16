use super::*;

/// The AbortController class.
/// [`AbortController`](https://developer.mozilla.org/en-US/docs/Web/API/AbortController)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct AbortController {
    inner: Any,
}
impl FromVal for AbortController {
    fn from_val(v: &Any) -> Self {
        AbortController {
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
impl core::ops::Deref for AbortController {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for AbortController {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for AbortController {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for AbortController {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<AbortController> for Any {
    fn from(s: AbortController) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&AbortController> for Any {
    fn from(s: &AbortController) -> Any {
        s.inner.clone().into()
    }
}
jsbind::utils::impl_dyn_cast!(AbortController);

impl AbortController {
    /// The `new AbortController(..)` constructor, creating a new AbortController instance
    pub fn new() -> AbortController {
        Self {
            inner: Any::global("AbortController").new(&[]).as_::<Any>(),
        }
    }
}
impl AbortController {
    /// Getter of the `signal` attribute.
    /// [`AbortController.signal`](https://developer.mozilla.org/en-US/docs/Web/API/AbortController/signal)
    pub fn signal(&self) -> AbortSignal {
        self.inner.get("signal").as_::<AbortSignal>()
    }
}
impl AbortController {
    /// The abort method.
    /// [`AbortController.abort`](https://developer.mozilla.org/en-US/docs/Web/API/AbortController/abort)
    pub fn abort0(&self) -> Undefined {
        self.inner.call("abort", &[]).as_::<Undefined>()
    }
    /// The abort method.
    /// [`AbortController.abort`](https://developer.mozilla.org/en-US/docs/Web/API/AbortController/abort)
    pub fn abort1(&self, reason: &Any) -> Undefined {
        self.inner
            .call("abort", &[reason.into()])
            .as_::<Undefined>()
    }
}
