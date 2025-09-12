use super::*;

/// The TransformStreamDefaultController class.
/// [`TransformStreamDefaultController`](https://developer.mozilla.org/en-US/docs/Web/API/TransformStreamDefaultController)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct TransformStreamDefaultController {
    inner: Any,
}

impl FromVal for TransformStreamDefaultController {
    fn from_val(v: &Any) -> Self {
        TransformStreamDefaultController {
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

impl core::ops::Deref for TransformStreamDefaultController {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for TransformStreamDefaultController {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for TransformStreamDefaultController {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for TransformStreamDefaultController {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}

impl From<TransformStreamDefaultController> for Any {
    fn from(s: TransformStreamDefaultController) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&TransformStreamDefaultController> for Any {
    fn from(s: &TransformStreamDefaultController) -> Any {
        s.inner.clone().into()
    }
}

jsbind::utils::impl_dyn_cast!(TransformStreamDefaultController);

impl TransformStreamDefaultController {
    /// Getter of the `desiredSize` attribute.
    /// [`TransformStreamDefaultController.desiredSize`](https://developer.mozilla.org/en-US/docs/Web/API/TransformStreamDefaultController/desiredSize)
    pub fn desired_size(&self) -> f64 {
        self.inner.get("desiredSize").as_::<f64>()
    }
}
impl TransformStreamDefaultController {
    /// The enqueue method.
    /// [`TransformStreamDefaultController.enqueue`](https://developer.mozilla.org/en-US/docs/Web/API/TransformStreamDefaultController/enqueue)
    pub fn enqueue(&self) -> Undefined {
        self.inner.call("enqueue", &[]).as_::<Undefined>()
    }
}
impl TransformStreamDefaultController {
    /// The enqueue method.
    /// [`TransformStreamDefaultController.enqueue`](https://developer.mozilla.org/en-US/docs/Web/API/TransformStreamDefaultController/enqueue)
    pub fn enqueue_with_chunk(&self, chunk: &Any) -> Undefined {
        self.inner
            .call("enqueue", &[chunk.into()])
            .as_::<Undefined>()
    }
}
impl TransformStreamDefaultController {
    /// The error method.
    /// [`TransformStreamDefaultController.error`](https://developer.mozilla.org/en-US/docs/Web/API/TransformStreamDefaultController/error)
    pub fn error(&self) -> Undefined {
        self.inner.call("error", &[]).as_::<Undefined>()
    }
}
impl TransformStreamDefaultController {
    /// The error method.
    /// [`TransformStreamDefaultController.error`](https://developer.mozilla.org/en-US/docs/Web/API/TransformStreamDefaultController/error)
    pub fn error_with_reason(&self, reason: &Any) -> Undefined {
        self.inner
            .call("error", &[reason.into()])
            .as_::<Undefined>()
    }
}
impl TransformStreamDefaultController {
    /// The terminate method.
    /// [`TransformStreamDefaultController.terminate`](https://developer.mozilla.org/en-US/docs/Web/API/TransformStreamDefaultController/terminate)
    pub fn terminate(&self) -> Undefined {
        self.inner.call("terminate", &[]).as_::<Undefined>()
    }
}
