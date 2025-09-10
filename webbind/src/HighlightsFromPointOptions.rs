use super::*;

/// The HighlightsFromPointOptions dictionary.
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct HighlightsFromPointOptions {
    inner: Any,
}

impl FromVal for HighlightsFromPointOptions {
    fn from_val(v: &Any) -> Self {
        HighlightsFromPointOptions { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for HighlightsFromPointOptions {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for HighlightsFromPointOptions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for HighlightsFromPointOptions {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for HighlightsFromPointOptions {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}

impl From<HighlightsFromPointOptions> for Any {
    fn from(s: HighlightsFromPointOptions) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&HighlightsFromPointOptions> for Any {
    fn from(s: &HighlightsFromPointOptions) -> Any {
        s.inner.clone()
    }
}

impl HighlightsFromPointOptions {
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
