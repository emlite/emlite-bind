use super::*;




/// The MLQuantizeDequantizeLinearSupportLimits dictionary.
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct MLQuantizeDequantizeLinearSupportLimits {
    inner: Any,
}

impl FromVal for MLQuantizeDequantizeLinearSupportLimits {
    fn from_val(v: &Any) -> Self {
        MLQuantizeDequantizeLinearSupportLimits { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for MLQuantizeDequantizeLinearSupportLimits {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for MLQuantizeDequantizeLinearSupportLimits {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for MLQuantizeDequantizeLinearSupportLimits {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for MLQuantizeDequantizeLinearSupportLimits {
    fn as_mut(&mut self) -> &mut Any {
      &mut self.inner
  }
}

impl From<MLQuantizeDequantizeLinearSupportLimits> for Any {
    fn from(s: MLQuantizeDequantizeLinearSupportLimits) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&MLQuantizeDequantizeLinearSupportLimits> for Any {
    fn from(s: &MLQuantizeDequantizeLinearSupportLimits) -> Any {
        s.inner.clone()
    }
}

impl MLQuantizeDequantizeLinearSupportLimits {
    /// Getter of the `input` attribute.
    pub fn input(&self) -> MLTensorLimits {
        self.inner.get("input").as_::<MLTensorLimits>()
    }

    /// Setter of the `input` attribute.
    pub fn set_input(&mut self, value: &MLTensorLimits) {
        self.inner.set("input", value);
    }
}
impl MLQuantizeDequantizeLinearSupportLimits {
    /// Getter of the `scale` attribute.
    pub fn scale(&self) -> MLTensorLimits {
        self.inner.get("scale").as_::<MLTensorLimits>()
    }

    /// Setter of the `scale` attribute.
    pub fn set_scale(&mut self, value: &MLTensorLimits) {
        self.inner.set("scale", value);
    }
}
impl MLQuantizeDequantizeLinearSupportLimits {
    /// Getter of the `zeroPoint` attribute.
    pub fn zero_point(&self) -> MLTensorLimits {
        self.inner.get("zeroPoint").as_::<MLTensorLimits>()
    }

    /// Setter of the `zeroPoint` attribute.
    pub fn set_zero_point(&mut self, value: &MLTensorLimits) {
        self.inner.set("zeroPoint", value);
    }
}
impl MLQuantizeDequantizeLinearSupportLimits {
    /// Getter of the `output` attribute.
    pub fn output(&self) -> MLDataTypeLimits {
        self.inner.get("output").as_::<MLDataTypeLimits>()
    }

    /// Setter of the `output` attribute.
    pub fn set_output(&mut self, value: &MLDataTypeLimits) {
        self.inner.set("output", value);
    }
}
