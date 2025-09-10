use super::*;

/// The HandwritingRecognizerQueryResult dictionary.
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct HandwritingRecognizerQueryResult {
    inner: Any,
}

impl FromVal for HandwritingRecognizerQueryResult {
    fn from_val(v: &Any) -> Self {
        HandwritingRecognizerQueryResult { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for HandwritingRecognizerQueryResult {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for HandwritingRecognizerQueryResult {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for HandwritingRecognizerQueryResult {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for HandwritingRecognizerQueryResult {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}

impl From<HandwritingRecognizerQueryResult> for Any {
    fn from(s: HandwritingRecognizerQueryResult) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&HandwritingRecognizerQueryResult> for Any {
    fn from(s: &HandwritingRecognizerQueryResult) -> Any {
        s.inner.clone()
    }
}

impl HandwritingRecognizerQueryResult {
    /// Getter of the `textAlternatives` attribute.
    pub fn text_alternatives(&self) -> bool {
        self.inner.get("textAlternatives").as_::<bool>()
    }

    /// Setter of the `textAlternatives` attribute.
    pub fn set_text_alternatives(&mut self, value: bool) {
        self.inner.set("textAlternatives", value);
    }
}
impl HandwritingRecognizerQueryResult {
    /// Getter of the `textSegmentation` attribute.
    pub fn text_segmentation(&self) -> bool {
        self.inner.get("textSegmentation").as_::<bool>()
    }

    /// Setter of the `textSegmentation` attribute.
    pub fn set_text_segmentation(&mut self, value: bool) {
        self.inner.set("textSegmentation", value);
    }
}
impl HandwritingRecognizerQueryResult {
    /// Getter of the `hints` attribute.
    pub fn hints(&self) -> HandwritingHintsQueryResult {
        self.inner.get("hints").as_::<HandwritingHintsQueryResult>()
    }

    /// Setter of the `hints` attribute.
    pub fn set_hints(&mut self, value: &HandwritingHintsQueryResult) {
        self.inner.set("hints", value);
    }
}
