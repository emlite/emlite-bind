use super::*;

#[derive(Clone, Debug)]
pub struct SpeechSynthesisUtterance {
    inner: EventTarget,
}
impl FromVal for SpeechSynthesisUtterance {
    fn from_val(v: &emlite::Val) -> Self {
        SpeechSynthesisUtterance {
            inner: EventTarget::from_val(v),
        }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl std::ops::Deref for SpeechSynthesisUtterance {
    type Target = EventTarget;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for SpeechSynthesisUtterance {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<SpeechSynthesisUtterance> for emlite::Val {
    fn from(s: SpeechSynthesisUtterance) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl SpeechSynthesisUtterance {
    pub fn new0() -> SpeechSynthesisUtterance {
        Self {
            inner: emlite::Val::global("SpeechSynthesisUtterance")
                .new(&[])
                .as_::<EventTarget>(),
        }
    }

    pub fn new1(text: jsbind::DOMString) -> SpeechSynthesisUtterance {
        Self {
            inner: emlite::Val::global("SpeechSynthesisUtterance")
                .new(&[text.into()])
                .as_::<EventTarget>(),
        }
    }
}
impl SpeechSynthesisUtterance {
    pub fn text(&self) -> jsbind::DOMString {
        self.inner.get("text").as_::<jsbind::DOMString>()
    }

    pub fn set_text(&mut self, value: jsbind::DOMString) {
        self.inner.set("text", value);
    }
}
impl SpeechSynthesisUtterance {
    pub fn lang(&self) -> jsbind::DOMString {
        self.inner.get("lang").as_::<jsbind::DOMString>()
    }

    pub fn set_lang(&mut self, value: jsbind::DOMString) {
        self.inner.set("lang", value);
    }
}
impl SpeechSynthesisUtterance {
    pub fn voice(&self) -> SpeechSynthesisVoice {
        self.inner.get("voice").as_::<SpeechSynthesisVoice>()
    }

    pub fn set_voice(&mut self, value: SpeechSynthesisVoice) {
        self.inner.set("voice", value);
    }
}
impl SpeechSynthesisUtterance {
    pub fn volume(&self) -> f32 {
        self.inner.get("volume").as_::<f32>()
    }

    pub fn set_volume(&mut self, value: f32) {
        self.inner.set("volume", value);
    }
}
impl SpeechSynthesisUtterance {
    pub fn rate(&self) -> f32 {
        self.inner.get("rate").as_::<f32>()
    }

    pub fn set_rate(&mut self, value: f32) {
        self.inner.set("rate", value);
    }
}
impl SpeechSynthesisUtterance {
    pub fn pitch(&self) -> f32 {
        self.inner.get("pitch").as_::<f32>()
    }

    pub fn set_pitch(&mut self, value: f32) {
        self.inner.set("pitch", value);
    }
}
impl SpeechSynthesisUtterance {
    pub fn onstart(&self) -> jsbind::Any {
        self.inner.get("onstart").as_::<jsbind::Any>()
    }

    pub fn set_onstart(&mut self, value: jsbind::Any) {
        self.inner.set("onstart", value);
    }
}
impl SpeechSynthesisUtterance {
    pub fn onend(&self) -> jsbind::Any {
        self.inner.get("onend").as_::<jsbind::Any>()
    }

    pub fn set_onend(&mut self, value: jsbind::Any) {
        self.inner.set("onend", value);
    }
}
impl SpeechSynthesisUtterance {
    pub fn onerror(&self) -> jsbind::Any {
        self.inner.get("onerror").as_::<jsbind::Any>()
    }

    pub fn set_onerror(&mut self, value: jsbind::Any) {
        self.inner.set("onerror", value);
    }
}
impl SpeechSynthesisUtterance {
    pub fn onpause(&self) -> jsbind::Any {
        self.inner.get("onpause").as_::<jsbind::Any>()
    }

    pub fn set_onpause(&mut self, value: jsbind::Any) {
        self.inner.set("onpause", value);
    }
}
impl SpeechSynthesisUtterance {
    pub fn onresume(&self) -> jsbind::Any {
        self.inner.get("onresume").as_::<jsbind::Any>()
    }

    pub fn set_onresume(&mut self, value: jsbind::Any) {
        self.inner.set("onresume", value);
    }
}
impl SpeechSynthesisUtterance {
    pub fn onmark(&self) -> jsbind::Any {
        self.inner.get("onmark").as_::<jsbind::Any>()
    }

    pub fn set_onmark(&mut self, value: jsbind::Any) {
        self.inner.set("onmark", value);
    }
}
impl SpeechSynthesisUtterance {
    pub fn onboundary(&self) -> jsbind::Any {
        self.inner.get("onboundary").as_::<jsbind::Any>()
    }

    pub fn set_onboundary(&mut self, value: jsbind::Any) {
        self.inner.set("onboundary", value);
    }
}
