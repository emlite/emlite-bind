use super::*;




/// The MLSliceOptions dictionary.
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct MLSliceOptions {
    inner: Any,
}

impl FromVal for MLSliceOptions {
    fn from_val(v: &Any) -> Self {
        MLSliceOptions { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for MLSliceOptions {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for MLSliceOptions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for MLSliceOptions {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for MLSliceOptions {
    fn as_mut(&mut self) -> &mut Any {
      &mut self.inner
  }
}

impl From<MLSliceOptions> for Any {
    fn from(s: MLSliceOptions) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&MLSliceOptions> for Any {
    fn from(s: &MLSliceOptions) -> Any {
        s.inner.clone()
    }
}

impl MLSliceOptions {
    /// Getter of the `strides` attribute.
    pub fn strides(&self) -> TypedArray<u32> {
        self.inner.get("strides").as_::<TypedArray<u32>>()
    }

    /// Setter of the `strides` attribute.
    pub fn set_strides(&mut self, value: TypedArray<u32>) {
        self.inner.set("strides", value);
    }
}
