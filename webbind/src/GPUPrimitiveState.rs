use super::*;

/// The GPUPrimitiveState dictionary.
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct GPUPrimitiveState {
    inner: Any,
}

impl FromVal for GPUPrimitiveState {
    fn from_val(v: &Any) -> Self {
        GPUPrimitiveState { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for GPUPrimitiveState {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for GPUPrimitiveState {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for GPUPrimitiveState {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for GPUPrimitiveState {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}

impl From<GPUPrimitiveState> for Any {
    fn from(s: GPUPrimitiveState) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&GPUPrimitiveState> for Any {
    fn from(s: &GPUPrimitiveState) -> Any {
        s.inner.clone()
    }
}

impl GPUPrimitiveState {
    /// Getter of the `topology` attribute.
    pub fn topology(&self) -> GPUPrimitiveTopology {
        self.inner.get("topology").as_::<GPUPrimitiveTopology>()
    }

    /// Setter of the `topology` attribute.
    pub fn set_topology(&mut self, value: &GPUPrimitiveTopology) {
        self.inner.set("topology", value);
    }
}
impl GPUPrimitiveState {
    /// Getter of the `stripIndexFormat` attribute.
    pub fn strip_index_format(&self) -> GPUIndexFormat {
        self.inner.get("stripIndexFormat").as_::<GPUIndexFormat>()
    }

    /// Setter of the `stripIndexFormat` attribute.
    pub fn set_strip_index_format(&mut self, value: &GPUIndexFormat) {
        self.inner.set("stripIndexFormat", value);
    }
}
impl GPUPrimitiveState {
    /// Getter of the `frontFace` attribute.
    pub fn front_face(&self) -> GPUFrontFace {
        self.inner.get("frontFace").as_::<GPUFrontFace>()
    }

    /// Setter of the `frontFace` attribute.
    pub fn set_front_face(&mut self, value: &GPUFrontFace) {
        self.inner.set("frontFace", value);
    }
}
impl GPUPrimitiveState {
    /// Getter of the `cullMode` attribute.
    pub fn cull_mode(&self) -> GPUCullMode {
        self.inner.get("cullMode").as_::<GPUCullMode>()
    }

    /// Setter of the `cullMode` attribute.
    pub fn set_cull_mode(&mut self, value: &GPUCullMode) {
        self.inner.set("cullMode", value);
    }
}
impl GPUPrimitiveState {
    /// Getter of the `unclippedDepth` attribute.
    pub fn unclipped_depth(&self) -> bool {
        self.inner.get("unclippedDepth").as_::<bool>()
    }

    /// Setter of the `unclippedDepth` attribute.
    pub fn set_unclipped_depth(&mut self, value: bool) {
        self.inner.set("unclippedDepth", value);
    }
}
