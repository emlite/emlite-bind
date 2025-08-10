use super::*;

/// The GPUExternalTexture class.
/// [`GPUExternalTexture`](https://developer.mozilla.org/en-US/docs/Web/API/GPUExternalTexture)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct GPUExternalTexture {
    inner: Any,
}

impl FromVal for GPUExternalTexture {
    fn from_val(v: &Any) -> Self {
        GPUExternalTexture {
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

impl core::ops::Deref for GPUExternalTexture {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for GPUExternalTexture {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for GPUExternalTexture {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for GPUExternalTexture {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}

impl From<GPUExternalTexture> for Any {
    fn from(s: GPUExternalTexture) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&GPUExternalTexture> for Any {
    fn from(s: &GPUExternalTexture) -> Any {
        s.inner.clone().into()
    }
}

jsbind::utils::impl_dyn_cast!(GPUExternalTexture);

impl GPUExternalTexture {
    /// Getter of the `label` attribute.
    /// [`GPUExternalTexture.label`](https://developer.mozilla.org/en-US/docs/Web/API/GPUExternalTexture/label)
    pub fn label(&self) -> JsString {
        self.inner.get("label").as_::<JsString>()
    }

    /// Setter of the `label` attribute.
    /// [`GPUExternalTexture.label`](https://developer.mozilla.org/en-US/docs/Web/API/GPUExternalTexture/label)
    pub fn set_label(&mut self, value: &JsString) {
        self.inner.set("label", value);
    }
}
