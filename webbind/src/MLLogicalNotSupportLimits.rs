use super::*;




/// The MLLogicalNotSupportLimits dictionary.
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct MLLogicalNotSupportLimits {
    inner: Any,
}

impl FromVal for MLLogicalNotSupportLimits {
    fn from_val(v: &Any) -> Self {
        MLLogicalNotSupportLimits { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for MLLogicalNotSupportLimits {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for MLLogicalNotSupportLimits {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for MLLogicalNotSupportLimits {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for MLLogicalNotSupportLimits {
    fn as_mut(&mut self) -> &mut Any {
      &mut self.inner
  }
}

impl From<MLLogicalNotSupportLimits> for Any {
    fn from(s: MLLogicalNotSupportLimits) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&MLLogicalNotSupportLimits> for Any {
    fn from(s: &MLLogicalNotSupportLimits) -> Any {
        s.inner.clone()
    }
}

impl MLLogicalNotSupportLimits {
    /// Getter of the `a` attribute.
    pub fn a(&self) -> MLTensorLimits {
        self.inner.get("a").as_::<MLTensorLimits>()
    }

    /// Setter of the `a` attribute.
    pub fn set_a(&mut self, value: &MLTensorLimits) {
        self.inner.set("a", value);
    }
}
impl MLLogicalNotSupportLimits {
    /// Getter of the `output` attribute.
    pub fn output(&self) -> MLDataTypeLimits {
        self.inner.get("output").as_::<MLDataTypeLimits>()
    }

    /// Setter of the `output` attribute.
    pub fn set_output(&mut self, value: &MLDataTypeLimits) {
        self.inner.set("output", value);
    }
}
