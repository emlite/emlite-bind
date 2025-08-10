use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct SpeechSynthesisEventInit {
    inner: Any,
}
impl FromVal for SpeechSynthesisEventInit {
    fn from_val(v: &Any) -> Self {
        SpeechSynthesisEventInit { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for SpeechSynthesisEventInit {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for SpeechSynthesisEventInit {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for SpeechSynthesisEventInit {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for SpeechSynthesisEventInit {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<SpeechSynthesisEventInit> for Any {
    fn from(s: SpeechSynthesisEventInit) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&SpeechSynthesisEventInit> for Any {
    fn from(s: &SpeechSynthesisEventInit) -> Any {
        s.inner.clone()
    }
}

impl SpeechSynthesisEventInit {
    pub fn utterance(&self) -> SpeechSynthesisUtterance {
        self.inner
            .get("utterance")
            .as_::<SpeechSynthesisUtterance>()
    }

    pub fn set_utterance(&mut self, value: &SpeechSynthesisUtterance) {
        self.inner.set("utterance", value);
    }
}
impl SpeechSynthesisEventInit {
    pub fn char_index(&self) -> u32 {
        self.inner.get("charIndex").as_::<u32>()
    }

    pub fn set_char_index(&mut self, value: u32) {
        self.inner.set("charIndex", value);
    }
}
impl SpeechSynthesisEventInit {
    pub fn char_length(&self) -> u32 {
        self.inner.get("charLength").as_::<u32>()
    }

    pub fn set_char_length(&mut self, value: u32) {
        self.inner.set("charLength", value);
    }
}
impl SpeechSynthesisEventInit {
    pub fn elapsed_time(&self) -> f32 {
        self.inner.get("elapsedTime").as_::<f32>()
    }

    pub fn set_elapsed_time(&mut self, value: f32) {
        self.inner.set("elapsedTime", value);
    }
}
impl SpeechSynthesisEventInit {
    pub fn name(&self) -> JsString {
        self.inner.get("name").as_::<JsString>()
    }

    pub fn set_name(&mut self, value: &JsString) {
        self.inner.set("name", value);
    }
}
