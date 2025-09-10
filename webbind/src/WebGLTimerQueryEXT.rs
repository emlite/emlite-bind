use super::*;

/// The WebGLTimerQueryEXT class.
/// [`WebGLTimerQueryEXT`](https://developer.mozilla.org/en-US/docs/Web/API/WebGLTimerQueryEXT)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct WebGLTimerQueryEXT {
    inner: WebGLObject,
}

impl FromVal for WebGLTimerQueryEXT {
    fn from_val(v: &Any) -> Self {
        WebGLTimerQueryEXT {
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

impl core::ops::Deref for WebGLTimerQueryEXT {
    type Target = WebGLObject;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for WebGLTimerQueryEXT {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for WebGLTimerQueryEXT {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for WebGLTimerQueryEXT {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}

impl From<WebGLTimerQueryEXT> for Any {
    fn from(s: WebGLTimerQueryEXT) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&WebGLTimerQueryEXT> for Any {
    fn from(s: &WebGLTimerQueryEXT) -> Any {
        s.inner.clone().into()
    }
}

jsbind::utils::impl_dyn_cast!(WebGLTimerQueryEXT);
