use super::*;

/// The GPUVertexAttribute dictionary.
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct GPUVertexAttribute {
    inner: Any,
}

impl FromVal for GPUVertexAttribute {
    fn from_val(v: &Any) -> Self {
        GPUVertexAttribute { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for GPUVertexAttribute {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for GPUVertexAttribute {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for GPUVertexAttribute {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for GPUVertexAttribute {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}

impl From<GPUVertexAttribute> for Any {
    fn from(s: GPUVertexAttribute) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&GPUVertexAttribute> for Any {
    fn from(s: &GPUVertexAttribute) -> Any {
        s.inner.clone()
    }
}

impl GPUVertexAttribute {
    /// Getter of the `format` attribute.
    pub fn format(&self) -> GPUVertexFormat {
        self.inner.get("format").as_::<GPUVertexFormat>()
    }

    /// Setter of the `format` attribute.
    pub fn set_format(&mut self, value: &GPUVertexFormat) {
        self.inner.set("format", value);
    }
}
impl GPUVertexAttribute {
    /// Getter of the `offset` attribute.
    pub fn offset(&self) -> Any {
        self.inner.get("offset").as_::<Any>()
    }

    /// Setter of the `offset` attribute.
    pub fn set_offset(&mut self, value: &Any) {
        self.inner.set("offset", value);
    }
}
impl GPUVertexAttribute {
    /// Getter of the `shaderLocation` attribute.
    pub fn shader_location(&self) -> Any {
        self.inner.get("shaderLocation").as_::<Any>()
    }

    /// Setter of the `shaderLocation` attribute.
    pub fn set_shader_location(&mut self, value: &Any) {
        self.inner.set("shaderLocation", value);
    }
}
