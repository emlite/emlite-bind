use super::*;




/// The HighlightHitResult dictionary.
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct HighlightHitResult {
    inner: Any,
}

impl FromVal for HighlightHitResult {
    fn from_val(v: &Any) -> Self {
        HighlightHitResult { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for HighlightHitResult {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for HighlightHitResult {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for HighlightHitResult {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for HighlightHitResult {
    fn as_mut(&mut self) -> &mut Any {
      &mut self.inner
  }
}

impl From<HighlightHitResult> for Any {
    fn from(s: HighlightHitResult) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&HighlightHitResult> for Any {
    fn from(s: &HighlightHitResult) -> Any {
        s.inner.clone()
    }
}

impl HighlightHitResult {
    /// Getter of the `highlight` attribute.
    pub fn highlight(&self) -> Highlight {
        self.inner.get("highlight").as_::<Highlight>()
    }

    /// Setter of the `highlight` attribute.
    pub fn set_highlight(&mut self, value: &Highlight) {
        self.inner.set("highlight", value);
    }
}
impl HighlightHitResult {
    /// Getter of the `ranges` attribute.
    pub fn ranges(&self) -> TypedArray<AbstractRange> {
        self.inner.get("ranges").as_::<TypedArray<AbstractRange>>()
    }

    /// Setter of the `ranges` attribute.
    pub fn set_ranges(&mut self, value: &TypedArray<AbstractRange>) {
        self.inner.set("ranges", value);
    }
}
