use super::*;




/// The MLBatchNormalizationOptions dictionary.
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct MLBatchNormalizationOptions {
    inner: Any,
}

impl FromVal for MLBatchNormalizationOptions {
    fn from_val(v: &Any) -> Self {
        MLBatchNormalizationOptions { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for MLBatchNormalizationOptions {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for MLBatchNormalizationOptions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for MLBatchNormalizationOptions {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for MLBatchNormalizationOptions {
    fn as_mut(&mut self) -> &mut Any {
      &mut self.inner
  }
}

impl From<MLBatchNormalizationOptions> for Any {
    fn from(s: MLBatchNormalizationOptions) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&MLBatchNormalizationOptions> for Any {
    fn from(s: &MLBatchNormalizationOptions) -> Any {
        s.inner.clone()
    }
}

impl MLBatchNormalizationOptions {
    /// Getter of the `scale` attribute.
    pub fn scale(&self) -> MLOperand {
        self.inner.get("scale").as_::<MLOperand>()
    }

    /// Setter of the `scale` attribute.
    pub fn set_scale(&mut self, value: &MLOperand) {
        self.inner.set("scale", value);
    }
}
impl MLBatchNormalizationOptions {
    /// Getter of the `bias` attribute.
    pub fn bias(&self) -> MLOperand {
        self.inner.get("bias").as_::<MLOperand>()
    }

    /// Setter of the `bias` attribute.
    pub fn set_bias(&mut self, value: &MLOperand) {
        self.inner.set("bias", value);
    }
}
impl MLBatchNormalizationOptions {
    /// Getter of the `axis` attribute.
    pub fn axis(&self) -> u32 {
        self.inner.get("axis").as_::<u32>()
    }

    /// Setter of the `axis` attribute.
    pub fn set_axis(&mut self, value: u32) {
        self.inner.set("axis", value);
    }
}
impl MLBatchNormalizationOptions {
    /// Getter of the `epsilon` attribute.
    pub fn epsilon(&self) -> f64 {
        self.inner.get("epsilon").as_::<f64>()
    }

    /// Setter of the `epsilon` attribute.
    pub fn set_epsilon(&mut self, value: f64) {
        self.inner.set("epsilon", value);
    }
}
