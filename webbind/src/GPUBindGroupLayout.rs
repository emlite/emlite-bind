use super::*;

/// The GPUBindGroupLayout class.
/// [`GPUBindGroupLayout`](https://developer.mozilla.org/en-US/docs/Web/API/GPUBindGroupLayout)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct GPUBindGroupLayout {
    inner: Any,
}
impl FromVal for GPUBindGroupLayout {
    fn from_val(v: &Any) -> Self {
        GPUBindGroupLayout {
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
impl core::ops::Deref for GPUBindGroupLayout {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for GPUBindGroupLayout {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for GPUBindGroupLayout {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for GPUBindGroupLayout {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<GPUBindGroupLayout> for Any {
    fn from(s: GPUBindGroupLayout) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&GPUBindGroupLayout> for Any {
    fn from(s: &GPUBindGroupLayout) -> Any {
        s.inner.clone().into()
    }
}
jsbind::utils::impl_dyn_cast!(GPUBindGroupLayout);

impl GPUBindGroupLayout {
    /// Getter of the `label` attribute.
    /// [`GPUBindGroupLayout.label`](https://developer.mozilla.org/en-US/docs/Web/API/GPUBindGroupLayout/label)
    pub fn label(&self) -> USVString {
        self.inner.get("label").as_::<USVString>()
    }

    /// Setter of the `label` attribute.
    /// [`GPUBindGroupLayout.label`](https://developer.mozilla.org/en-US/docs/Web/API/GPUBindGroupLayout/label)
    pub fn set_label(&mut self, value: &USVString) {
        self.inner.set("label", value);
    }
}
