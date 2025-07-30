use super::*;

/// The WebGLObject class.
/// [`WebGLObject`](https://developer.mozilla.org/en-US/docs/Web/API/WebGLObject)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct WebGLObject {
    inner: Any,
}
impl FromVal for WebGLObject {
    fn from_val(v: &Any) -> Self {
        WebGLObject {
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
impl core::ops::Deref for WebGLObject {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for WebGLObject {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for WebGLObject {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for WebGLObject {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<WebGLObject> for Any {
    fn from(s: WebGLObject) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&WebGLObject> for Any {
    fn from(s: &WebGLObject) -> Any {
        s.inner.clone().into()
    }
}
jsbind::utils::impl_dyn_cast!(WebGLObject);

impl WebGLObject {
    /// Getter of the `label` attribute.
    /// [`WebGLObject.label`](https://developer.mozilla.org/en-US/docs/Web/API/WebGLObject/label)
    pub fn label(&self) -> JsString {
        self.inner.get("label").as_::<JsString>()
    }

    /// Setter of the `label` attribute.
    /// [`WebGLObject.label`](https://developer.mozilla.org/en-US/docs/Web/API/WebGLObject/label)
    pub fn set_label(&mut self, value: &JsString) {
        self.inner.set("label", value);
    }
}
