use super::*;




/// The MLInstanceNormalizationOptions dictionary.
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct MLInstanceNormalizationOptions {
    inner: Any,
}

impl FromVal for MLInstanceNormalizationOptions {
    fn from_val(v: &Any) -> Self {
        MLInstanceNormalizationOptions { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for MLInstanceNormalizationOptions {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for MLInstanceNormalizationOptions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for MLInstanceNormalizationOptions {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for MLInstanceNormalizationOptions {
    fn as_mut(&mut self) -> &mut Any {
      &mut self.inner
  }
}

impl From<MLInstanceNormalizationOptions> for Any {
    fn from(s: MLInstanceNormalizationOptions) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&MLInstanceNormalizationOptions> for Any {
    fn from(s: &MLInstanceNormalizationOptions) -> Any {
        s.inner.clone()
    }
}

impl MLInstanceNormalizationOptions {
    /// Getter of the `scale` attribute.
    pub fn scale(&self) -> MLOperand {
        self.inner.get("scale").as_::<MLOperand>()
    }

    /// Setter of the `scale` attribute.
    pub fn set_scale(&mut self, value: &MLOperand) {
        self.inner.set("scale", value);
    }
}
impl MLInstanceNormalizationOptions {
    /// Getter of the `bias` attribute.
    pub fn bias(&self) -> MLOperand {
        self.inner.get("bias").as_::<MLOperand>()
    }

    /// Setter of the `bias` attribute.
    pub fn set_bias(&mut self, value: &MLOperand) {
        self.inner.set("bias", value);
    }
}
impl MLInstanceNormalizationOptions {
    /// Getter of the `epsilon` attribute.
    pub fn epsilon(&self) -> f64 {
        self.inner.get("epsilon").as_::<f64>()
    }

    /// Setter of the `epsilon` attribute.
    pub fn set_epsilon(&mut self, value: f64) {
        self.inner.set("epsilon", value);
    }
}
impl MLInstanceNormalizationOptions {
    /// Getter of the `layout` attribute.
    pub fn layout(&self) -> MLInputOperandLayout {
        self.inner.get("layout").as_::<MLInputOperandLayout>()
    }

    /// Setter of the `layout` attribute.
    pub fn set_layout(&mut self, value: &MLInputOperandLayout) {
        self.inner.set("layout", value);
    }
}
