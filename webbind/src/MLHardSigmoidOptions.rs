use super::*;




/// The MLHardSigmoidOptions dictionary.
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct MLHardSigmoidOptions {
    inner: Any,
}

impl FromVal for MLHardSigmoidOptions {
    fn from_val(v: &Any) -> Self {
        MLHardSigmoidOptions { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for MLHardSigmoidOptions {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for MLHardSigmoidOptions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for MLHardSigmoidOptions {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for MLHardSigmoidOptions {
    fn as_mut(&mut self) -> &mut Any {
      &mut self.inner
  }
}

impl From<MLHardSigmoidOptions> for Any {
    fn from(s: MLHardSigmoidOptions) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&MLHardSigmoidOptions> for Any {
    fn from(s: &MLHardSigmoidOptions) -> Any {
        s.inner.clone()
    }
}

impl MLHardSigmoidOptions {
    /// Getter of the `alpha` attribute.
    pub fn alpha(&self) -> f64 {
        self.inner.get("alpha").as_::<f64>()
    }

    /// Setter of the `alpha` attribute.
    pub fn set_alpha(&mut self, value: f64) {
        self.inner.set("alpha", value);
    }
}
impl MLHardSigmoidOptions {
    /// Getter of the `beta` attribute.
    pub fn beta(&self) -> f64 {
        self.inner.get("beta").as_::<f64>()
    }

    /// Setter of the `beta` attribute.
    pub fn set_beta(&mut self, value: f64) {
        self.inner.set("beta", value);
    }
}
