use super::*;




/// The MLGemmSupportLimits dictionary.
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct MLGemmSupportLimits {
    inner: Any,
}

impl FromVal for MLGemmSupportLimits {
    fn from_val(v: &Any) -> Self {
        MLGemmSupportLimits { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for MLGemmSupportLimits {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for MLGemmSupportLimits {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for MLGemmSupportLimits {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for MLGemmSupportLimits {
    fn as_mut(&mut self) -> &mut Any {
      &mut self.inner
  }
}

impl From<MLGemmSupportLimits> for Any {
    fn from(s: MLGemmSupportLimits) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&MLGemmSupportLimits> for Any {
    fn from(s: &MLGemmSupportLimits) -> Any {
        s.inner.clone()
    }
}

impl MLGemmSupportLimits {
    /// Getter of the `a` attribute.
    pub fn a(&self) -> MLTensorLimits {
        self.inner.get("a").as_::<MLTensorLimits>()
    }

    /// Setter of the `a` attribute.
    pub fn set_a(&mut self, value: &MLTensorLimits) {
        self.inner.set("a", value);
    }
}
impl MLGemmSupportLimits {
    /// Getter of the `b` attribute.
    pub fn b(&self) -> MLTensorLimits {
        self.inner.get("b").as_::<MLTensorLimits>()
    }

    /// Setter of the `b` attribute.
    pub fn set_b(&mut self, value: &MLTensorLimits) {
        self.inner.set("b", value);
    }
}
impl MLGemmSupportLimits {
    /// Getter of the `c` attribute.
    pub fn c(&self) -> MLTensorLimits {
        self.inner.get("c").as_::<MLTensorLimits>()
    }

    /// Setter of the `c` attribute.
    pub fn set_c(&mut self, value: &MLTensorLimits) {
        self.inner.set("c", value);
    }
}
impl MLGemmSupportLimits {
    /// Getter of the `output` attribute.
    pub fn output(&self) -> MLDataTypeLimits {
        self.inner.get("output").as_::<MLDataTypeLimits>()
    }

    /// Setter of the `output` attribute.
    pub fn set_output(&mut self, value: &MLDataTypeLimits) {
        self.inner.set("output", value);
    }
}
