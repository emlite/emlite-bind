use super::*;

/// The ReadableStreamDefaultController class.
/// [`ReadableStreamDefaultController`](https://developer.mozilla.org/en-US/docs/Web/API/ReadableStreamDefaultController)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct ReadableStreamDefaultController {
    inner: Any,
}
impl FromVal for ReadableStreamDefaultController {
    fn from_val(v: &Any) -> Self {
        ReadableStreamDefaultController {
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
impl core::ops::Deref for ReadableStreamDefaultController {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for ReadableStreamDefaultController {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for ReadableStreamDefaultController {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for ReadableStreamDefaultController {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<ReadableStreamDefaultController> for Any {
    fn from(s: ReadableStreamDefaultController) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&ReadableStreamDefaultController> for Any {
    fn from(s: &ReadableStreamDefaultController) -> Any {
        s.inner.clone().into()
    }
}
jsbind::utils::impl_dyn_cast!(ReadableStreamDefaultController);

impl ReadableStreamDefaultController {
    /// Getter of the `desiredSize` attribute.
    /// [`ReadableStreamDefaultController.desiredSize`](https://developer.mozilla.org/en-US/docs/Web/API/ReadableStreamDefaultController/desiredSize)
    pub fn desired_size(&self) -> f64 {
        self.inner.get("desiredSize").as_::<f64>()
    }
}
impl ReadableStreamDefaultController {
    /// The close method.
    /// [`ReadableStreamDefaultController.close`](https://developer.mozilla.org/en-US/docs/Web/API/ReadableStreamDefaultController/close)
    pub fn close(&self) -> Undefined {
        self.inner.call("close", &[]).as_::<Undefined>()
    }
}
impl ReadableStreamDefaultController {
    /// The enqueue method.
    /// [`ReadableStreamDefaultController.enqueue`](https://developer.mozilla.org/en-US/docs/Web/API/ReadableStreamDefaultController/enqueue)
    pub fn enqueue0(&self) -> Undefined {
        self.inner.call("enqueue", &[]).as_::<Undefined>()
    }
    /// The enqueue method.
    /// [`ReadableStreamDefaultController.enqueue`](https://developer.mozilla.org/en-US/docs/Web/API/ReadableStreamDefaultController/enqueue)
    pub fn enqueue1(&self, chunk: &Any) -> Undefined {
        self.inner
            .call("enqueue", &[chunk.into()])
            .as_::<Undefined>()
    }
}
impl ReadableStreamDefaultController {
    /// The error method.
    /// [`ReadableStreamDefaultController.error`](https://developer.mozilla.org/en-US/docs/Web/API/ReadableStreamDefaultController/error)
    pub fn error0(&self) -> Undefined {
        self.inner.call("error", &[]).as_::<Undefined>()
    }
    /// The error method.
    /// [`ReadableStreamDefaultController.error`](https://developer.mozilla.org/en-US/docs/Web/API/ReadableStreamDefaultController/error)
    pub fn error1(&self, e: &Any) -> Undefined {
        self.inner.call("error", &[e.into()]).as_::<Undefined>()
    }
}
