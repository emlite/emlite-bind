use super::*;




/// The MLNormalizationSupportLimits dictionary.
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct MLNormalizationSupportLimits {
    inner: Any,
}

impl FromVal for MLNormalizationSupportLimits {
    fn from_val(v: &Any) -> Self {
        MLNormalizationSupportLimits { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for MLNormalizationSupportLimits {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for MLNormalizationSupportLimits {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for MLNormalizationSupportLimits {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for MLNormalizationSupportLimits {
    fn as_mut(&mut self) -> &mut Any {
      &mut self.inner
  }
}

impl From<MLNormalizationSupportLimits> for Any {
    fn from(s: MLNormalizationSupportLimits) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&MLNormalizationSupportLimits> for Any {
    fn from(s: &MLNormalizationSupportLimits) -> Any {
        s.inner.clone()
    }
}

impl MLNormalizationSupportLimits {
    /// Getter of the `input` attribute.
    pub fn input(&self) -> MLTensorLimits {
        self.inner.get("input").as_::<MLTensorLimits>()
    }

    /// Setter of the `input` attribute.
    pub fn set_input(&mut self, value: &MLTensorLimits) {
        self.inner.set("input", value);
    }
}
impl MLNormalizationSupportLimits {
    /// Getter of the `scale` attribute.
    pub fn scale(&self) -> MLTensorLimits {
        self.inner.get("scale").as_::<MLTensorLimits>()
    }

    /// Setter of the `scale` attribute.
    pub fn set_scale(&mut self, value: &MLTensorLimits) {
        self.inner.set("scale", value);
    }
}
impl MLNormalizationSupportLimits {
    /// Getter of the `bias` attribute.
    pub fn bias(&self) -> MLTensorLimits {
        self.inner.get("bias").as_::<MLTensorLimits>()
    }

    /// Setter of the `bias` attribute.
    pub fn set_bias(&mut self, value: &MLTensorLimits) {
        self.inner.set("bias", value);
    }
}
impl MLNormalizationSupportLimits {
    /// Getter of the `output` attribute.
    pub fn output(&self) -> MLDataTypeLimits {
        self.inner.get("output").as_::<MLDataTypeLimits>()
    }

    /// Setter of the `output` attribute.
    pub fn set_output(&mut self, value: &MLDataTypeLimits) {
        self.inner.set("output", value);
    }
}
