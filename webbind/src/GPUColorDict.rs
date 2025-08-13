use super::*;




/// The GPUColorDict dictionary.
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct GPUColorDict {
    inner: Any,
}

impl FromVal for GPUColorDict {
    fn from_val(v: &Any) -> Self {
        GPUColorDict { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for GPUColorDict {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for GPUColorDict {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for GPUColorDict {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for GPUColorDict {
    fn as_mut(&mut self) -> &mut Any {
      &mut self.inner
  }
}

impl From<GPUColorDict> for Any {
    fn from(s: GPUColorDict) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&GPUColorDict> for Any {
    fn from(s: &GPUColorDict) -> Any {
        s.inner.clone()
    }
}

impl GPUColorDict {
    /// Getter of the `r` attribute.
    pub fn r(&self) -> f64 {
        self.inner.get("r").as_::<f64>()
    }

    /// Setter of the `r` attribute.
    pub fn set_r(&mut self, value: f64) {
        self.inner.set("r", value);
    }
}
impl GPUColorDict {
    /// Getter of the `g` attribute.
    pub fn g(&self) -> f64 {
        self.inner.get("g").as_::<f64>()
    }

    /// Setter of the `g` attribute.
    pub fn set_g(&mut self, value: f64) {
        self.inner.set("g", value);
    }
}
impl GPUColorDict {
    /// Getter of the `b` attribute.
    pub fn b(&self) -> f64 {
        self.inner.get("b").as_::<f64>()
    }

    /// Setter of the `b` attribute.
    pub fn set_b(&mut self, value: f64) {
        self.inner.set("b", value);
    }
}
impl GPUColorDict {
    /// Getter of the `a` attribute.
    pub fn a(&self) -> f64 {
        self.inner.get("a").as_::<f64>()
    }

    /// Setter of the `a` attribute.
    pub fn set_a(&mut self, value: f64) {
        self.inner.set("a", value);
    }
}
