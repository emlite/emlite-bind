use super::*;

/// The GPUObjectDescriptorBase dictionary.
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct GPUObjectDescriptorBase {
    inner: Any,
}

impl FromVal for GPUObjectDescriptorBase {
    fn from_val(v: &Any) -> Self {
        GPUObjectDescriptorBase { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for GPUObjectDescriptorBase {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for GPUObjectDescriptorBase {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for GPUObjectDescriptorBase {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for GPUObjectDescriptorBase {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}

impl From<GPUObjectDescriptorBase> for Any {
    fn from(s: GPUObjectDescriptorBase) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&GPUObjectDescriptorBase> for Any {
    fn from(s: &GPUObjectDescriptorBase) -> Any {
        s.inner.clone()
    }
}

impl GPUObjectDescriptorBase {
    /// Getter of the `label` attribute.
    pub fn label(&self) -> JsString {
        self.inner.get("label").as_::<JsString>()
    }

    /// Setter of the `label` attribute.
    pub fn set_label(&mut self, value: &JsString) {
        self.inner.set("label", value);
    }
}
