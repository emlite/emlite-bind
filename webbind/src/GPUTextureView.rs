use super::*;

/// The GPUTextureView class.
/// [`GPUTextureView`](https://developer.mozilla.org/en-US/docs/Web/API/GPUTextureView)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct GPUTextureView {
    inner: Any,
}

impl FromVal for GPUTextureView {
    fn from_val(v: &Any) -> Self {
        GPUTextureView {
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

impl core::ops::Deref for GPUTextureView {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for GPUTextureView {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for GPUTextureView {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for GPUTextureView {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}

impl From<GPUTextureView> for Any {
    fn from(s: GPUTextureView) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&GPUTextureView> for Any {
    fn from(s: &GPUTextureView) -> Any {
        s.inner.clone().into()
    }
}

jsbind::utils::impl_dyn_cast!(GPUTextureView);

impl GPUTextureView {
    /// Getter of the `label` attribute.
    /// [`GPUTextureView.label`](https://developer.mozilla.org/en-US/docs/Web/API/GPUTextureView/label)
    pub fn label(&self) -> JsString {
        self.inner.get("label").as_::<JsString>()
    }

    /// Setter of the `label` attribute.
    /// [`GPUTextureView.label`](https://developer.mozilla.org/en-US/docs/Web/API/GPUTextureView/label)
    pub fn set_label(&mut self, value: &JsString) {
        self.inner.set("label", value);
    }
}
