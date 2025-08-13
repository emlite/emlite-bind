use super::*;




/// The MLPreluSupportLimits dictionary.
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct MLPreluSupportLimits {
    inner: Any,
}

impl FromVal for MLPreluSupportLimits {
    fn from_val(v: &Any) -> Self {
        MLPreluSupportLimits { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for MLPreluSupportLimits {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for MLPreluSupportLimits {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for MLPreluSupportLimits {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for MLPreluSupportLimits {
    fn as_mut(&mut self) -> &mut Any {
      &mut self.inner
  }
}

impl From<MLPreluSupportLimits> for Any {
    fn from(s: MLPreluSupportLimits) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&MLPreluSupportLimits> for Any {
    fn from(s: &MLPreluSupportLimits) -> Any {
        s.inner.clone()
    }
}

impl MLPreluSupportLimits {
    /// Getter of the `input` attribute.
    pub fn input(&self) -> MLTensorLimits {
        self.inner.get("input").as_::<MLTensorLimits>()
    }

    /// Setter of the `input` attribute.
    pub fn set_input(&mut self, value: &MLTensorLimits) {
        self.inner.set("input", value);
    }
}
impl MLPreluSupportLimits {
    /// Getter of the `slope` attribute.
    pub fn slope(&self) -> MLTensorLimits {
        self.inner.get("slope").as_::<MLTensorLimits>()
    }

    /// Setter of the `slope` attribute.
    pub fn set_slope(&mut self, value: &MLTensorLimits) {
        self.inner.set("slope", value);
    }
}
impl MLPreluSupportLimits {
    /// Getter of the `output` attribute.
    pub fn output(&self) -> MLDataTypeLimits {
        self.inner.get("output").as_::<MLDataTypeLimits>()
    }

    /// Setter of the `output` attribute.
    pub fn set_output(&mut self, value: &MLDataTypeLimits) {
        self.inner.set("output", value);
    }
}
