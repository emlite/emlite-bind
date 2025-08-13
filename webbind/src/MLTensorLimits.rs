use super::*;




/// The MLTensorLimits dictionary.
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct MLTensorLimits {
    inner: Any,
}

impl FromVal for MLTensorLimits {
    fn from_val(v: &Any) -> Self {
        MLTensorLimits { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for MLTensorLimits {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for MLTensorLimits {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for MLTensorLimits {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for MLTensorLimits {
    fn as_mut(&mut self) -> &mut Any {
      &mut self.inner
  }
}

impl From<MLTensorLimits> for Any {
    fn from(s: MLTensorLimits) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&MLTensorLimits> for Any {
    fn from(s: &MLTensorLimits) -> Any {
        s.inner.clone()
    }
}

impl MLTensorLimits {
    /// Getter of the `dataTypes` attribute.
    pub fn data_types(&self) -> Any {
        self.inner.get("dataTypes").as_::<Any>()
    }

    /// Setter of the `dataTypes` attribute.
    pub fn set_data_types(&mut self, value: &Any) {
        self.inner.set("dataTypes", value);
    }
}
impl MLTensorLimits {
    /// Getter of the `rankRange` attribute.
    pub fn rank_range(&self) -> MLRankRange {
        self.inner.get("rankRange").as_::<MLRankRange>()
    }

    /// Setter of the `rankRange` attribute.
    pub fn set_rank_range(&mut self, value: &MLRankRange) {
        self.inner.set("rankRange", value);
    }
}
