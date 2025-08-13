use super::*;




/// The GPUOrigin3DDict dictionary.
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct GPUOrigin3DDict {
    inner: Any,
}

impl FromVal for GPUOrigin3DDict {
    fn from_val(v: &Any) -> Self {
        GPUOrigin3DDict { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for GPUOrigin3DDict {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for GPUOrigin3DDict {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for GPUOrigin3DDict {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for GPUOrigin3DDict {
    fn as_mut(&mut self) -> &mut Any {
      &mut self.inner
  }
}

impl From<GPUOrigin3DDict> for Any {
    fn from(s: GPUOrigin3DDict) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&GPUOrigin3DDict> for Any {
    fn from(s: &GPUOrigin3DDict) -> Any {
        s.inner.clone()
    }
}

impl GPUOrigin3DDict {
    /// Getter of the `x` attribute.
    pub fn x(&self) -> Any {
        self.inner.get("x").as_::<Any>()
    }

    /// Setter of the `x` attribute.
    pub fn set_x(&mut self, value: &Any) {
        self.inner.set("x", value);
    }
}
impl GPUOrigin3DDict {
    /// Getter of the `y` attribute.
    pub fn y(&self) -> Any {
        self.inner.get("y").as_::<Any>()
    }

    /// Setter of the `y` attribute.
    pub fn set_y(&mut self, value: &Any) {
        self.inner.set("y", value);
    }
}
impl GPUOrigin3DDict {
    /// Getter of the `z` attribute.
    pub fn z(&self) -> Any {
        self.inner.get("z").as_::<Any>()
    }

    /// Setter of the `z` attribute.
    pub fn set_z(&mut self, value: &Any) {
        self.inner.set("z", value);
    }
}
