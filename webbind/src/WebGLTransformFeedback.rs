use super::*;

/// The WebGLTransformFeedback class.
/// [`WebGLTransformFeedback`](https://developer.mozilla.org/en-US/docs/Web/API/WebGLTransformFeedback)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct WebGLTransformFeedback {
    inner: WebGLObject,
}

impl FromVal for WebGLTransformFeedback {
    fn from_val(v: &Any) -> Self {
        WebGLTransformFeedback {
            inner: WebGLObject::from_val(v),
        }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for WebGLTransformFeedback {
    type Target = WebGLObject;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for WebGLTransformFeedback {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for WebGLTransformFeedback {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for WebGLTransformFeedback {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}

impl From<WebGLTransformFeedback> for Any {
    fn from(s: WebGLTransformFeedback) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&WebGLTransformFeedback> for Any {
    fn from(s: &WebGLTransformFeedback) -> Any {
        s.inner.clone().into()
    }
}

jsbind::utils::impl_dyn_cast!(WebGLTransformFeedback);
