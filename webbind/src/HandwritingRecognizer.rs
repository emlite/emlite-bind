use super::*;

#[derive(Clone, Debug)]
pub struct HandwritingHints {
    inner: emlite::Val,
}
impl FromVal for HandwritingHints {
    fn from_val(v: &emlite::Val) -> Self {
        HandwritingHints { inner: v.clone() }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl std::ops::Deref for HandwritingHints {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for HandwritingHints {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<HandwritingHints> for emlite::Val {
    fn from(s: HandwritingHints) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl HandwritingHints {
    pub fn recognition_type(&self) -> jsbind::DOMString {
        self.inner.get("recognitionType").as_::<jsbind::DOMString>()
    }

    pub fn set_recognition_type(&mut self, value: jsbind::DOMString) {
        self.inner.set("recognitionType", value);
    }
}
impl HandwritingHints {
    pub fn input_type(&self) -> jsbind::DOMString {
        self.inner.get("inputType").as_::<jsbind::DOMString>()
    }

    pub fn set_input_type(&mut self, value: jsbind::DOMString) {
        self.inner.set("inputType", value);
    }
}
impl HandwritingHints {
    pub fn text_context(&self) -> jsbind::DOMString {
        self.inner.get("textContext").as_::<jsbind::DOMString>()
    }

    pub fn set_text_context(&mut self, value: jsbind::DOMString) {
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
#[derive(Clone, Debug)]
pub struct HandwritingRecognizer {
    inner: emlite::Val,
}
impl FromVal for HandwritingRecognizer {
    fn from_val(v: &emlite::Val) -> Self {
        HandwritingRecognizer {
            inner: emlite::Val::from_val(v),
        }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl std::ops::Deref for HandwritingRecognizer {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for HandwritingRecognizer {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<HandwritingRecognizer> for emlite::Val {
    fn from(s: HandwritingRecognizer) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl HandwritingRecognizer {
    pub fn start_drawing0(&self) -> HandwritingDrawing {
        self.inner
            .call("startDrawing", &[])
            .as_::<HandwritingDrawing>()
    }

    pub fn start_drawing1(&self, hints: HandwritingHints) -> HandwritingDrawing {
        self.inner
            .call("startDrawing", &[hints.into()])
            .as_::<HandwritingDrawing>()
    }
}
impl HandwritingRecognizer {
    pub fn finish(&self) -> jsbind::Undefined {
        self.inner.call("finish", &[]).as_::<jsbind::Undefined>()
    }
}
