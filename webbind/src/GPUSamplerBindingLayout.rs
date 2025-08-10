use super::*;

/// The GPUSamplerBindingLayout dictionary.
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct GPUSamplerBindingLayout {
    inner: Any,
}

impl FromVal for GPUSamplerBindingLayout {
    fn from_val(v: &Any) -> Self {
        GPUSamplerBindingLayout { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for GPUSamplerBindingLayout {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for GPUSamplerBindingLayout {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for GPUSamplerBindingLayout {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for GPUSamplerBindingLayout {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}

impl From<GPUSamplerBindingLayout> for Any {
    fn from(s: GPUSamplerBindingLayout) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&GPUSamplerBindingLayout> for Any {
    fn from(s: &GPUSamplerBindingLayout) -> Any {
        s.inner.clone()
    }
}

impl GPUSamplerBindingLayout {
    /// Getter of the `type` attribute.
    pub fn type_(&self) -> GPUSamplerBindingType {
        self.inner.get("type").as_::<GPUSamplerBindingType>()
    }

    /// Setter of the `type` attribute.
    pub fn set_type_(&mut self, value: &GPUSamplerBindingType) {
        self.inner.set("type", value);
    }
}
