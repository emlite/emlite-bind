use super::*;




/// The MLArgMinMaxOptions dictionary.
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct MLArgMinMaxOptions {
    inner: Any,
}

impl FromVal for MLArgMinMaxOptions {
    fn from_val(v: &Any) -> Self {
        MLArgMinMaxOptions { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for MLArgMinMaxOptions {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for MLArgMinMaxOptions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for MLArgMinMaxOptions {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for MLArgMinMaxOptions {
    fn as_mut(&mut self) -> &mut Any {
      &mut self.inner
  }
}

impl From<MLArgMinMaxOptions> for Any {
    fn from(s: MLArgMinMaxOptions) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&MLArgMinMaxOptions> for Any {
    fn from(s: &MLArgMinMaxOptions) -> Any {
        s.inner.clone()
    }
}

impl MLArgMinMaxOptions {
    /// Getter of the `keepDimensions` attribute.
    pub fn keep_dimensions(&self) -> bool {
        self.inner.get("keepDimensions").as_::<bool>()
    }

    /// Setter of the `keepDimensions` attribute.
    pub fn set_keep_dimensions(&mut self, value: bool) {
        self.inner.set("keepDimensions", value);
    }
}
impl MLArgMinMaxOptions {
    /// Getter of the `outputDataType` attribute.
    pub fn output_data_type(&self) -> MLOperandDataType {
        self.inner.get("outputDataType").as_::<MLOperandDataType>()
    }

    /// Setter of the `outputDataType` attribute.
    pub fn set_output_data_type(&mut self, value: &MLOperandDataType) {
        self.inner.set("outputDataType", value);
    }
}
