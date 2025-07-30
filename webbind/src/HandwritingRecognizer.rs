use super::*;

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
    pub fn recognition_type(&self) -> JsString {
        self.inner.get("recognitionType").as_::<JsString>()
    }

    pub fn set_recognition_type(&mut self, value: &JsString) {
        self.inner.set("recognitionType", value);
    }
}
impl HandwritingHints {
    pub fn input_type(&self) -> JsString {
        self.inner.get("inputType").as_::<JsString>()
    }

    pub fn set_input_type(&mut self, value: &JsString) {
        self.inner.set("inputType", value);
    }
}
impl HandwritingHints {
    pub fn text_context(&self) -> JsString {
        self.inner.get("textContext").as_::<JsString>()
    }

    pub fn set_text_context(&mut self, value: &JsString) {
        self.inner.set("textContext", value);
    }
}
impl HandwritingHints {
    pub fn alternatives(&self) -> u32 {
        self.inner.get("alternatives").as_::<u32>()
    }

    pub fn set_alternatives(&mut self, value: u32) {
        self.inner.set("alternatives", value);
    }
}
/// The HandwritingRecognizer class.
/// [`HandwritingRecognizer`](https://developer.mozilla.org/en-US/docs/Web/API/HandwritingRecognizer)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct HandwritingRecognizer {
    inner: Any,
}
impl FromVal for HandwritingRecognizer {
    fn from_val(v: &Any) -> Self {
        HandwritingRecognizer {
            inner: Any::from_val(v),
        }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for HandwritingRecognizer {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for HandwritingRecognizer {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for HandwritingRecognizer {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for HandwritingRecognizer {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<HandwritingRecognizer> for Any {
    fn from(s: HandwritingRecognizer) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&HandwritingRecognizer> for Any {
    fn from(s: &HandwritingRecognizer) -> Any {
        s.inner.clone().into()
    }
}
jsbind::utils::impl_dyn_cast!(HandwritingRecognizer);

impl HandwritingRecognizer {
    /// The startDrawing method.
    /// [`HandwritingRecognizer.startDrawing`](https://developer.mozilla.org/en-US/docs/Web/API/HandwritingRecognizer/startDrawing)
    pub fn start_drawing0(&self) -> HandwritingDrawing {
        self.inner
            .call("startDrawing", &[])
            .as_::<HandwritingDrawing>()
    }
    /// The startDrawing method.
    /// [`HandwritingRecognizer.startDrawing`](https://developer.mozilla.org/en-US/docs/Web/API/HandwritingRecognizer/startDrawing)
    pub fn start_drawing1(&self, hints: &HandwritingHints) -> HandwritingDrawing {
        self.inner
            .call("startDrawing", &[hints.into()])
            .as_::<HandwritingDrawing>()
    }
}
impl HandwritingRecognizer {
    /// The finish method.
    /// [`HandwritingRecognizer.finish`](https://developer.mozilla.org/en-US/docs/Web/API/HandwritingRecognizer/finish)
    pub fn finish(&self) -> Undefined {
        self.inner.call("finish", &[]).as_::<Undefined>()
    }
}
