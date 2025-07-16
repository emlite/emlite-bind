use super::*;

/// The GPUBindGroup class.
/// [`GPUBindGroup`](https://developer.mozilla.org/en-US/docs/Web/API/GPUBindGroup)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct GPUBindGroup {
    inner: Any,
}
impl FromVal for GPUBindGroup {
    fn from_val(v: &Any) -> Self {
        GPUBindGroup {
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
impl core::ops::Deref for GPUBindGroup {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for GPUBindGroup {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for GPUBindGroup {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for GPUBindGroup {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<GPUBindGroup> for Any {
    fn from(s: GPUBindGroup) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&GPUBindGroup> for Any {
    fn from(s: &GPUBindGroup) -> Any {
        s.inner.clone().into()
    }
}
jsbind::utils::impl_dyn_cast!(GPUBindGroup);

impl GPUBindGroup {
    /// Getter of the `label` attribute.
    /// [`GPUBindGroup.label`](https://developer.mozilla.org/en-US/docs/Web/API/GPUBindGroup/label)
    pub fn label(&self) -> String {
        self.inner.get("label").as_::<String>()
    }

    /// Setter of the `label` attribute.
    /// [`GPUBindGroup.label`](https://developer.mozilla.org/en-US/docs/Web/API/GPUBindGroup/label)
    pub fn set_label(&mut self, value: &str) {
        self.inner.set("label", value);
    }
}
