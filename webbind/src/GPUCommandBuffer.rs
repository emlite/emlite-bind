use super::*;

/// The GPUCommandBuffer class.
/// [`GPUCommandBuffer`](https://developer.mozilla.org/en-US/docs/Web/API/GPUCommandBuffer)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct GPUCommandBuffer {
    inner: Any,
}
impl FromVal for GPUCommandBuffer {
    fn from_val(v: &Any) -> Self {
        GPUCommandBuffer {
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
impl core::ops::Deref for GPUCommandBuffer {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for GPUCommandBuffer {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for GPUCommandBuffer {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for GPUCommandBuffer {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<GPUCommandBuffer> for Any {
    fn from(s: GPUCommandBuffer) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&GPUCommandBuffer> for Any {
    fn from(s: &GPUCommandBuffer) -> Any {
        s.inner.clone().into()
    }
}
jsbind::utils::impl_dyn_cast!(GPUCommandBuffer);

impl GPUCommandBuffer {
    /// Getter of the `label` attribute.
    /// [`GPUCommandBuffer.label`](https://developer.mozilla.org/en-US/docs/Web/API/GPUCommandBuffer/label)
    pub fn label(&self) -> JsString {
        self.inner.get("label").as_::<JsString>()
    }

    /// Setter of the `label` attribute.
    /// [`GPUCommandBuffer.label`](https://developer.mozilla.org/en-US/docs/Web/API/GPUCommandBuffer/label)
    pub fn set_label(&mut self, value: &JsString) {
        self.inner.set("label", value);
    }
}
