use super::*;




/// The MLSplitSupportLimits dictionary.
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct MLSplitSupportLimits {
    inner: Any,
}

impl FromVal for MLSplitSupportLimits {
    fn from_val(v: &Any) -> Self {
        MLSplitSupportLimits { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for MLSplitSupportLimits {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for MLSplitSupportLimits {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for MLSplitSupportLimits {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for MLSplitSupportLimits {
    fn as_mut(&mut self) -> &mut Any {
      &mut self.inner
  }
}

impl From<MLSplitSupportLimits> for Any {
    fn from(s: MLSplitSupportLimits) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&MLSplitSupportLimits> for Any {
    fn from(s: &MLSplitSupportLimits) -> Any {
        s.inner.clone()
    }
}

impl MLSplitSupportLimits {
    /// Getter of the `input` attribute.
    pub fn input(&self) -> MLTensorLimits {
        self.inner.get("input").as_::<MLTensorLimits>()
    }

    /// Setter of the `input` attribute.
    pub fn set_input(&mut self, value: &MLTensorLimits) {
        self.inner.set("input", value);
    }
}
impl MLSplitSupportLimits {
    /// Getter of the `outputs` attribute.
    pub fn outputs(&self) -> MLDataTypeLimits {
        self.inner.get("outputs").as_::<MLDataTypeLimits>()
    }

    /// Setter of the `outputs` attribute.
    pub fn set_outputs(&mut self, value: &MLDataTypeLimits) {
        self.inner.set("outputs", value);
    }
}
