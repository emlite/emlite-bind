use super::*;

/// The GPUPipelineError class.
/// [`GPUPipelineError`](https://developer.mozilla.org/en-US/docs/Web/API/GPUPipelineError)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct GPUPipelineError {
    inner: DOMException,
}
impl FromVal for GPUPipelineError {
    fn from_val(v: &Any) -> Self {
        GPUPipelineError {
            inner: DOMException::from_val(v),
        }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for GPUPipelineError {
    type Target = DOMException;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for GPUPipelineError {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for GPUPipelineError {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for GPUPipelineError {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<GPUPipelineError> for Any {
    fn from(s: GPUPipelineError) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&GPUPipelineError> for Any {
    fn from(s: &GPUPipelineError) -> Any {
        s.inner.clone().into()
    }
}
jsbind::utils::impl_dyn_cast!(GPUPipelineError);

impl GPUPipelineError {
    /// The `new GPUPipelineError(..)` constructor, creating a new GPUPipelineError instance
    pub fn new0() -> GPUPipelineError {
        Self {
            inner: Any::global("GPUPipelineError")
                .new(&[])
                .as_::<DOMException>(),
        }
    }

    /// The `new GPUPipelineError(..)` constructor, creating a new GPUPipelineError instance
    pub fn new1(message: &str) -> GPUPipelineError {
        Self {
            inner: Any::global("GPUPipelineError")
                .new(&[message.into()])
                .as_::<DOMException>(),
        }
    }

    /// The `new GPUPipelineError(..)` constructor, creating a new GPUPipelineError instance
    pub fn new2(message: &str, options: &Any) -> GPUPipelineError {
        Self {
            inner: Any::global("GPUPipelineError")
                .new(&[message.into(), options.into()])
                .as_::<DOMException>(),
        }
    }
}
impl GPUPipelineError {
    /// Getter of the `reason` attribute.
    /// [`GPUPipelineError.reason`](https://developer.mozilla.org/en-US/docs/Web/API/GPUPipelineError/reason)
    pub fn reason(&self) -> GPUPipelineErrorReason {
        self.inner.get("reason").as_::<GPUPipelineErrorReason>()
    }
}
