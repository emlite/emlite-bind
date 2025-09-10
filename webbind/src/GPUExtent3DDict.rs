use super::*;

/// The GPUExtent3DDict dictionary.
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct GPUExtent3DDict {
    inner: Any,
}

impl FromVal for GPUExtent3DDict {
    fn from_val(v: &Any) -> Self {
        GPUExtent3DDict { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for GPUExtent3DDict {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for GPUExtent3DDict {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for GPUExtent3DDict {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for GPUExtent3DDict {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}

impl From<GPUExtent3DDict> for Any {
    fn from(s: GPUExtent3DDict) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&GPUExtent3DDict> for Any {
    fn from(s: &GPUExtent3DDict) -> Any {
        s.inner.clone()
    }
}

impl GPUExtent3DDict {
    /// Getter of the `width` attribute.
    pub fn width(&self) -> Any {
        self.inner.get("width").as_::<Any>()
    }

    /// Setter of the `width` attribute.
    pub fn set_width(&mut self, value: &Any) {
        self.inner.set("width", value);
    }
}
impl GPUExtent3DDict {
    /// Getter of the `height` attribute.
    pub fn height(&self) -> Any {
        self.inner.get("height").as_::<Any>()
    }

    /// Setter of the `height` attribute.
    pub fn set_height(&mut self, value: &Any) {
        self.inner.set("height", value);
    }
}
impl GPUExtent3DDict {
    /// Getter of the `depthOrArrayLayers` attribute.
    pub fn depth_or_array_layers(&self) -> Any {
        self.inner.get("depthOrArrayLayers").as_::<Any>()
    }

    /// Setter of the `depthOrArrayLayers` attribute.
    pub fn set_depth_or_array_layers(&mut self, value: &Any) {
        self.inner.set("depthOrArrayLayers", value);
    }
}
