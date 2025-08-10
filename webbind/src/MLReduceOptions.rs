use super::*;

/// The MLReduceOptions dictionary.
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct MLReduceOptions {
    inner: Any,
}

impl FromVal for MLReduceOptions {
    fn from_val(v: &Any) -> Self {
        MLReduceOptions { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for MLReduceOptions {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for MLReduceOptions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for MLReduceOptions {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for MLReduceOptions {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}

impl From<MLReduceOptions> for Any {
    fn from(s: MLReduceOptions) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&MLReduceOptions> for Any {
    fn from(s: &MLReduceOptions) -> Any {
        s.inner.clone()
    }
}

impl MLReduceOptions {
    /// Getter of the `axes` attribute.
    pub fn axes(&self) -> TypedArray<u32> {
        self.inner.get("axes").as_::<TypedArray<u32>>()
    }

    /// Setter of the `axes` attribute.
    pub fn set_axes(&mut self, value: TypedArray<u32>) {
        self.inner.set("axes", value);
    }
}
impl MLReduceOptions {
    /// Getter of the `keepDimensions` attribute.
    pub fn keep_dimensions(&self) -> bool {
        self.inner.get("keepDimensions").as_::<bool>()
    }

    /// Setter of the `keepDimensions` attribute.
    pub fn set_keep_dimensions(&mut self, value: bool) {
        self.inner.set("keepDimensions", value);
    }
}
