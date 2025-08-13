use super::*;




/// The MLDataTypeLimits dictionary.
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct MLDataTypeLimits {
    inner: Any,
}

impl FromVal for MLDataTypeLimits {
    fn from_val(v: &Any) -> Self {
        MLDataTypeLimits { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for MLDataTypeLimits {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for MLDataTypeLimits {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for MLDataTypeLimits {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for MLDataTypeLimits {
    fn as_mut(&mut self) -> &mut Any {
      &mut self.inner
  }
}

impl From<MLDataTypeLimits> for Any {
    fn from(s: MLDataTypeLimits) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&MLDataTypeLimits> for Any {
    fn from(s: &MLDataTypeLimits) -> Any {
        s.inner.clone()
    }
}

impl MLDataTypeLimits {
    /// Getter of the `dataTypes` attribute.
    pub fn data_types(&self) -> Any {
        self.inner.get("dataTypes").as_::<Any>()
    }

    /// Setter of the `dataTypes` attribute.
    pub fn set_data_types(&mut self, value: &Any) {
        self.inner.set("dataTypes", value);
    }
}
