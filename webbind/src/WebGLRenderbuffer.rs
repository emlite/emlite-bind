use super::*;

/// The WebGLRenderbuffer class.
/// [`WebGLRenderbuffer`](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderbuffer)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct WebGLRenderbuffer {
    inner: WebGLObject,
}
impl FromVal for WebGLRenderbuffer {
    fn from_val(v: &Any) -> Self {
        WebGLRenderbuffer {
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
impl core::ops::Deref for WebGLRenderbuffer {
    type Target = WebGLObject;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for WebGLRenderbuffer {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for WebGLRenderbuffer {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for WebGLRenderbuffer {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<WebGLRenderbuffer> for Any {
    fn from(s: WebGLRenderbuffer) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&WebGLRenderbuffer> for Any {
    fn from(s: &WebGLRenderbuffer) -> Any {
        s.inner.clone().into()
    }
}
jsbind::utils::impl_dyn_cast!(WebGLRenderbuffer);
