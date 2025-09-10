use super::*;

/// The MLReverseOptions dictionary.
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct MLReverseOptions {
    inner: Any,
}

impl FromVal for MLReverseOptions {
    fn from_val(v: &Any) -> Self {
        MLReverseOptions { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for MLReverseOptions {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for MLReverseOptions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for MLReverseOptions {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for MLReverseOptions {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}

impl From<MLReverseOptions> for Any {
    fn from(s: MLReverseOptions) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&MLReverseOptions> for Any {
    fn from(s: &MLReverseOptions) -> Any {
        s.inner.clone()
    }
}

impl MLReverseOptions {
    /// Getter of the `axes` attribute.
    pub fn axes(&self) -> TypedArray<u32> {
        self.inner.get("axes").as_::<TypedArray<u32>>()
    }

    /// Setter of the `axes` attribute.
    pub fn set_axes(&mut self, value: TypedArray<u32>) {
        self.inner.set("axes", value);
    }
}
