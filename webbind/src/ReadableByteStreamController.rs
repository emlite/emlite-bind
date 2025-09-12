use super::*;

/// The ReadableByteStreamController class.
/// [`ReadableByteStreamController`](https://developer.mozilla.org/en-US/docs/Web/API/ReadableByteStreamController)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct ReadableByteStreamController {
    inner: Any,
}

impl FromVal for ReadableByteStreamController {
    fn from_val(v: &Any) -> Self {
        ReadableByteStreamController {
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

impl core::ops::Deref for ReadableByteStreamController {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for ReadableByteStreamController {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for ReadableByteStreamController {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for ReadableByteStreamController {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}

impl From<ReadableByteStreamController> for Any {
    fn from(s: ReadableByteStreamController) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&ReadableByteStreamController> for Any {
    fn from(s: &ReadableByteStreamController) -> Any {
        s.inner.clone().into()
    }
}

jsbind::utils::impl_dyn_cast!(ReadableByteStreamController);

impl ReadableByteStreamController {
    /// Getter of the `byobRequest` attribute.
    /// [`ReadableByteStreamController.byobRequest`](https://developer.mozilla.org/en-US/docs/Web/API/ReadableByteStreamController/byobRequest)
    pub fn byob_request(&self) -> ReadableStreamBYOBRequest {
        self.inner
            .get("byobRequest")
            .as_::<ReadableStreamBYOBRequest>()
    }
}
impl ReadableByteStreamController {
    /// Getter of the `desiredSize` attribute.
    /// [`ReadableByteStreamController.desiredSize`](https://developer.mozilla.org/en-US/docs/Web/API/ReadableByteStreamController/desiredSize)
    pub fn desired_size(&self) -> f64 {
        self.inner.get("desiredSize").as_::<f64>()
    }
}
impl ReadableByteStreamController {
    /// The close method.
    /// [`ReadableByteStreamController.close`](https://developer.mozilla.org/en-US/docs/Web/API/ReadableByteStreamController/close)
    pub fn close(&self) -> Undefined {
        self.inner.call("close", &[]).as_::<Undefined>()
    }
}
impl ReadableByteStreamController {
    /// The enqueue method.
    /// [`ReadableByteStreamController.enqueue`](https://developer.mozilla.org/en-US/docs/Web/API/ReadableByteStreamController/enqueue)
    pub fn enqueue(&self, chunk: &Any) -> Undefined {
        self.inner
            .call("enqueue", &[chunk.into()])
            .as_::<Undefined>()
    }
}
impl ReadableByteStreamController {
    /// The error method.
    /// [`ReadableByteStreamController.error`](https://developer.mozilla.org/en-US/docs/Web/API/ReadableByteStreamController/error)
    pub fn error(&self) -> Undefined {
        self.inner.call("error", &[]).as_::<Undefined>()
    }
}
impl ReadableByteStreamController {
    /// The error method.
    /// [`ReadableByteStreamController.error`](https://developer.mozilla.org/en-US/docs/Web/API/ReadableByteStreamController/error)
    pub fn error_with_e(&self, e: &Any) -> Undefined {
        self.inner.call("error", &[e.into()]).as_::<Undefined>()
    }
}
