use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct HandwritingHintsQueryResult {
    inner: Any,
}
impl FromVal for HandwritingHintsQueryResult {
    fn from_val(v: &Any) -> Self {
        HandwritingHintsQueryResult { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for HandwritingHintsQueryResult {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for HandwritingHintsQueryResult {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for HandwritingHintsQueryResult {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for HandwritingHintsQueryResult {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<HandwritingHintsQueryResult> for Any {
    fn from(s: HandwritingHintsQueryResult) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&HandwritingHintsQueryResult> for Any {
    fn from(s: &HandwritingHintsQueryResult) -> Any {
        s.inner.clone()
    }
}

impl HandwritingHintsQueryResult {
    pub fn recognition_type(&self) -> TypedArray<HandwritingRecognitionType> {
        self.inner
            .get("recognitionType")
            .as_::<TypedArray<HandwritingRecognitionType>>()
    }

    pub fn set_recognition_type(&mut self, value: &TypedArray<HandwritingRecognitionType>) {
        self.inner.set("recognitionType", value);
    }
}
impl HandwritingHintsQueryResult {
    pub fn input_type(&self) -> TypedArray<HandwritingInputType> {
        self.inner
            .get("inputType")
            .as_::<TypedArray<HandwritingInputType>>()
    }

    pub fn set_input_type(&mut self, value: &TypedArray<HandwritingInputType>) {
        self.inner.set("inputType", value);
    }
}
impl HandwritingHintsQueryResult {
    pub fn text_context(&self) -> bool {
        self.inner.get("textContext").as_::<bool>()
    }

    pub fn set_text_context(&mut self, value: bool) {
        self.inner.set("textContext", value);
    }
}
impl HandwritingHintsQueryResult {
    pub fn alternatives(&self) -> bool {
        self.inner.get("alternatives").as_::<bool>()
    }

    pub fn set_alternatives(&mut self, value: bool) {
        self.inner.set("alternatives", value);
    }
}
