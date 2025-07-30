use super::*;

/// The WebGLActiveInfo class.
/// [`WebGLActiveInfo`](https://developer.mozilla.org/en-US/docs/Web/API/WebGLActiveInfo)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct WebGLActiveInfo {
    inner: Any,
}
impl FromVal for WebGLActiveInfo {
    fn from_val(v: &Any) -> Self {
        WebGLActiveInfo {
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
impl core::ops::Deref for WebGLActiveInfo {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for WebGLActiveInfo {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for WebGLActiveInfo {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for WebGLActiveInfo {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<WebGLActiveInfo> for Any {
    fn from(s: WebGLActiveInfo) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&WebGLActiveInfo> for Any {
    fn from(s: &WebGLActiveInfo) -> Any {
        s.inner.clone().into()
    }
}
jsbind::utils::impl_dyn_cast!(WebGLActiveInfo);

impl WebGLActiveInfo {
    /// Getter of the `size` attribute.
    /// [`WebGLActiveInfo.size`](https://developer.mozilla.org/en-US/docs/Web/API/WebGLActiveInfo/size)
    pub fn size(&self) -> Any {
        self.inner.get("size").as_::<Any>()
    }
}
impl WebGLActiveInfo {
    /// Getter of the `type` attribute.
    /// [`WebGLActiveInfo.type`](https://developer.mozilla.org/en-US/docs/Web/API/WebGLActiveInfo/type)
    pub fn type_(&self) -> Any {
        self.inner.get("type").as_::<Any>()
    }
}
impl WebGLActiveInfo {
    /// Getter of the `name` attribute.
    /// [`WebGLActiveInfo.name`](https://developer.mozilla.org/en-US/docs/Web/API/WebGLActiveInfo/name)
    pub fn name(&self) -> JsString {
        self.inner.get("name").as_::<JsString>()
    }
}
