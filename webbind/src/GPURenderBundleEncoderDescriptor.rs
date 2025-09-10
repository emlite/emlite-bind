use super::*;

/// The GPURenderBundleEncoderDescriptor dictionary.
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct GPURenderBundleEncoderDescriptor {
    inner: Any,
}

impl FromVal for GPURenderBundleEncoderDescriptor {
    fn from_val(v: &Any) -> Self {
        GPURenderBundleEncoderDescriptor { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for GPURenderBundleEncoderDescriptor {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for GPURenderBundleEncoderDescriptor {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for GPURenderBundleEncoderDescriptor {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for GPURenderBundleEncoderDescriptor {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}

impl From<GPURenderBundleEncoderDescriptor> for Any {
    fn from(s: GPURenderBundleEncoderDescriptor) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&GPURenderBundleEncoderDescriptor> for Any {
    fn from(s: &GPURenderBundleEncoderDescriptor) -> Any {
        s.inner.clone()
    }
}

impl GPURenderBundleEncoderDescriptor {
    /// Getter of the `depthReadOnly` attribute.
    pub fn depth_read_only(&self) -> bool {
        self.inner.get("depthReadOnly").as_::<bool>()
    }

    /// Setter of the `depthReadOnly` attribute.
    pub fn set_depth_read_only(&mut self, value: bool) {
        self.inner.set("depthReadOnly", value);
    }
}
impl GPURenderBundleEncoderDescriptor {
    /// Getter of the `stencilReadOnly` attribute.
    pub fn stencil_read_only(&self) -> bool {
        self.inner.get("stencilReadOnly").as_::<bool>()
    }

    /// Setter of the `stencilReadOnly` attribute.
    pub fn set_stencil_read_only(&mut self, value: bool) {
        self.inner.set("stencilReadOnly", value);
    }
}
