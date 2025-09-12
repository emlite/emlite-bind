use super::*;

/// The WritableStreamDefaultController class.
/// [`WritableStreamDefaultController`](https://developer.mozilla.org/en-US/docs/Web/API/WritableStreamDefaultController)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct WritableStreamDefaultController {
    inner: Any,
}

impl FromVal for WritableStreamDefaultController {
    fn from_val(v: &Any) -> Self {
        WritableStreamDefaultController {
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

impl core::ops::Deref for WritableStreamDefaultController {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for WritableStreamDefaultController {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for WritableStreamDefaultController {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for WritableStreamDefaultController {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}

impl From<WritableStreamDefaultController> for Any {
    fn from(s: WritableStreamDefaultController) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&WritableStreamDefaultController> for Any {
    fn from(s: &WritableStreamDefaultController) -> Any {
        s.inner.clone().into()
    }
}

jsbind::utils::impl_dyn_cast!(WritableStreamDefaultController);

impl WritableStreamDefaultController {
    /// Getter of the `signal` attribute.
    /// [`WritableStreamDefaultController.signal`](https://developer.mozilla.org/en-US/docs/Web/API/WritableStreamDefaultController/signal)
    pub fn signal(&self) -> AbortSignal {
        self.inner.get("signal").as_::<AbortSignal>()
    }
}
impl WritableStreamDefaultController {
    /// The error method.
    /// [`WritableStreamDefaultController.error`](https://developer.mozilla.org/en-US/docs/Web/API/WritableStreamDefaultController/error)
    pub fn error(&self) -> Undefined {
        self.inner.call("error", &[]).as_::<Undefined>()
    }
}
impl WritableStreamDefaultController {
    /// The error method.
    /// [`WritableStreamDefaultController.error`](https://developer.mozilla.org/en-US/docs/Web/API/WritableStreamDefaultController/error)
    pub fn error_with_e(&self, e: &Any) -> Undefined {
        self.inner.call("error", &[e.into()]).as_::<Undefined>()
    }
}
