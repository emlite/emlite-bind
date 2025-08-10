use super::*;

/// The GetComposedRangesOptions dictionary.
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct GetComposedRangesOptions {
    inner: Any,
}

impl FromVal for GetComposedRangesOptions {
    fn from_val(v: &Any) -> Self {
        GetComposedRangesOptions { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for GetComposedRangesOptions {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for GetComposedRangesOptions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for GetComposedRangesOptions {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for GetComposedRangesOptions {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}

impl From<GetComposedRangesOptions> for Any {
    fn from(s: GetComposedRangesOptions) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&GetComposedRangesOptions> for Any {
    fn from(s: &GetComposedRangesOptions) -> Any {
        s.inner.clone()
    }
}

impl GetComposedRangesOptions {
    /// Getter of the `shadowRoots` attribute.
    pub fn shadow_roots(&self) -> TypedArray<ShadowRoot> {
        self.inner
            .get("shadowRoots")
            .as_::<TypedArray<ShadowRoot>>()
    }

    /// Setter of the `shadowRoots` attribute.
    pub fn set_shadow_roots(&mut self, value: &TypedArray<ShadowRoot>) {
        self.inner.set("shadowRoots", value);
    }
}
