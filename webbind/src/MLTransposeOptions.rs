use super::*;

/// The MLTransposeOptions dictionary.
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct MLTransposeOptions {
    inner: Any,
}

impl FromVal for MLTransposeOptions {
    fn from_val(v: &Any) -> Self {
        MLTransposeOptions { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for MLTransposeOptions {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for MLTransposeOptions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for MLTransposeOptions {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for MLTransposeOptions {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}

impl From<MLTransposeOptions> for Any {
    fn from(s: MLTransposeOptions) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&MLTransposeOptions> for Any {
    fn from(s: &MLTransposeOptions) -> Any {
        s.inner.clone()
    }
}

impl MLTransposeOptions {
    /// Getter of the `permutation` attribute.
    pub fn permutation(&self) -> TypedArray<u32> {
        self.inner.get("permutation").as_::<TypedArray<u32>>()
    }

    /// Setter of the `permutation` attribute.
    pub fn set_permutation(&mut self, value: TypedArray<u32>) {
        self.inner.set("permutation", value);
    }
}
