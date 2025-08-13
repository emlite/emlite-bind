use super::*;




/// The HandwritingHints dictionary.
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct HandwritingHints {
    inner: Any,
}

impl FromVal for HandwritingHints {
    fn from_val(v: &Any) -> Self {
        HandwritingHints { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for HandwritingHints {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for HandwritingHints {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for HandwritingHints {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for HandwritingHints {
    fn as_mut(&mut self) -> &mut Any {
      &mut self.inner
  }
}

impl From<HandwritingHints> for Any {
    fn from(s: HandwritingHints) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&HandwritingHints> for Any {
    fn from(s: &HandwritingHints) -> Any {
        s.inner.clone()
    }
}

impl HandwritingHints {
    /// Getter of the `recognitionType` attribute.
    pub fn recognition_type(&self) -> JsString {
        self.inner.get("recognitionType").as_::<JsString>()
    }

    /// Setter of the `recognitionType` attribute.
    pub fn set_recognition_type(&mut self, value: &JsString) {
        self.inner.set("recognitionType", value);
    }
}
impl HandwritingHints {
    /// Getter of the `inputType` attribute.
    pub fn input_type(&self) -> JsString {
        self.inner.get("inputType").as_::<JsString>()
    }

    /// Setter of the `inputType` attribute.
    pub fn set_input_type(&mut self, value: &JsString) {
        self.inner.set("inputType", value);
    }
}
impl HandwritingHints {
    /// Getter of the `textContext` attribute.
    pub fn text_context(&self) -> JsString {
        self.inner.get("textContext").as_::<JsString>()
    }

    /// Setter of the `textContext` attribute.
    pub fn set_text_context(&mut self, value: &JsString) {
        self.inner.set("textContext", value);
    }
}
impl HandwritingHints {
    /// Getter of the `alternatives` attribute.
    pub fn alternatives(&self) -> u32 {
        self.inner.get("alternatives").as_::<u32>()
    }

    /// Setter of the `alternatives` attribute.
    pub fn set_alternatives(&mut self, value: u32) {
        self.inner.set("alternatives", value);
    }
}
