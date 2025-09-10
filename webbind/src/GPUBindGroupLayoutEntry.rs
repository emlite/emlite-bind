use super::*;

/// The GPUBindGroupLayoutEntry dictionary.
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct GPUBindGroupLayoutEntry {
    inner: Any,
}

impl FromVal for GPUBindGroupLayoutEntry {
    fn from_val(v: &Any) -> Self {
        GPUBindGroupLayoutEntry { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for GPUBindGroupLayoutEntry {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for GPUBindGroupLayoutEntry {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for GPUBindGroupLayoutEntry {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for GPUBindGroupLayoutEntry {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}

impl From<GPUBindGroupLayoutEntry> for Any {
    fn from(s: GPUBindGroupLayoutEntry) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&GPUBindGroupLayoutEntry> for Any {
    fn from(s: &GPUBindGroupLayoutEntry) -> Any {
        s.inner.clone()
    }
}

impl GPUBindGroupLayoutEntry {
    /// Getter of the `binding` attribute.
    pub fn binding(&self) -> Any {
        self.inner.get("binding").as_::<Any>()
    }

    /// Setter of the `binding` attribute.
    pub fn set_binding(&mut self, value: &Any) {
        self.inner.set("binding", value);
    }
}
impl GPUBindGroupLayoutEntry {
    /// Getter of the `visibility` attribute.
    pub fn visibility(&self) -> Any {
        self.inner.get("visibility").as_::<Any>()
    }

    /// Setter of the `visibility` attribute.
    pub fn set_visibility(&mut self, value: &Any) {
        self.inner.set("visibility", value);
    }
}
impl GPUBindGroupLayoutEntry {
    /// Getter of the `buffer` attribute.
    pub fn buffer(&self) -> GPUBufferBindingLayout {
        self.inner.get("buffer").as_::<GPUBufferBindingLayout>()
    }

    /// Setter of the `buffer` attribute.
    pub fn set_buffer(&mut self, value: &GPUBufferBindingLayout) {
        self.inner.set("buffer", value);
    }
}
impl GPUBindGroupLayoutEntry {
    /// Getter of the `sampler` attribute.
    pub fn sampler(&self) -> GPUSamplerBindingLayout {
        self.inner.get("sampler").as_::<GPUSamplerBindingLayout>()
    }

    /// Setter of the `sampler` attribute.
    pub fn set_sampler(&mut self, value: &GPUSamplerBindingLayout) {
        self.inner.set("sampler", value);
    }
}
impl GPUBindGroupLayoutEntry {
    /// Getter of the `texture` attribute.
    pub fn texture(&self) -> GPUTextureBindingLayout {
        self.inner.get("texture").as_::<GPUTextureBindingLayout>()
    }

    /// Setter of the `texture` attribute.
    pub fn set_texture(&mut self, value: &GPUTextureBindingLayout) {
        self.inner.set("texture", value);
    }
}
impl GPUBindGroupLayoutEntry {
    /// Getter of the `storageTexture` attribute.
    pub fn storage_texture(&self) -> GPUStorageTextureBindingLayout {
        self.inner
            .get("storageTexture")
            .as_::<GPUStorageTextureBindingLayout>()
    }

    /// Setter of the `storageTexture` attribute.
    pub fn set_storage_texture(&mut self, value: &GPUStorageTextureBindingLayout) {
        self.inner.set("storageTexture", value);
    }
}
impl GPUBindGroupLayoutEntry {
    /// Getter of the `externalTexture` attribute.
    pub fn external_texture(&self) -> GPUExternalTextureBindingLayout {
        self.inner
            .get("externalTexture")
            .as_::<GPUExternalTextureBindingLayout>()
    }

    /// Setter of the `externalTexture` attribute.
    pub fn set_external_texture(&mut self, value: &GPUExternalTextureBindingLayout) {
        self.inner.set("externalTexture", value);
    }
}
