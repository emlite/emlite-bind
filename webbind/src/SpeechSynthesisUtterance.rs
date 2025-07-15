use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
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
impl core::ops::Deref for SpeechSynthesisUtterance {
    type Target = EventTarget;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for SpeechSynthesisUtterance {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for SpeechSynthesisUtterance {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for SpeechSynthesisUtterance {
    fn as_mut(&mut self) -> &mut emlite::Val {
        &mut self.inner
    }
}
impl From<SpeechSynthesisUtterance> for emlite::Val {
    fn from(s: SpeechSynthesisUtterance) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}
impl From<&SpeechSynthesisUtterance> for emlite::Val {
    fn from(s: &SpeechSynthesisUtterance) -> emlite::Val {
        s.inner.clone().into()
    }
}
jsbind::utils::impl_dyn_cast!(SpeechSynthesisUtterance);

impl SpeechSynthesisUtterance {
    pub fn new0() -> SpeechSynthesisUtterance {
        Self {
            inner: emlite::Val::global("SpeechSynthesisUtterance")
                .new(&[])
                .as_::<EventTarget>(),
        }
    }

    pub fn new1(text: &str) -> SpeechSynthesisUtterance {
        Self {
            inner: emlite::Val::global("SpeechSynthesisUtterance")
                .new(&[text.into()])
                .as_::<EventTarget>(),
        }
    }
}
impl SpeechSynthesisUtterance {
    pub fn text(&self) -> String {
        self.inner.get("text").as_::<String>()
    }

    pub fn set_text(&mut self, value: &str) {
        self.inner.set("text", value);
    }
}
impl SpeechSynthesisUtterance {
    pub fn lang(&self) -> String {
        self.inner.get("lang").as_::<String>()
    }

    pub fn set_lang(&mut self, value: &str) {
        self.inner.set("lang", value);
    }
}
impl SpeechSynthesisUtterance {
    pub fn voice(&self) -> SpeechSynthesisVoice {
        self.inner.get("voice").as_::<SpeechSynthesisVoice>()
    }

    pub fn set_voice(&mut self, value: &SpeechSynthesisVoice) {
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
    pub fn onstart(&self) -> Any {
        self.inner.get("onstart").as_::<Any>()
    }

    pub fn set_onstart(&mut self, value: &Any) {
        self.inner.set("onstart", value);
    }
}
impl SpeechSynthesisUtterance {
    pub fn onend(&self) -> Any {
        self.inner.get("onend").as_::<Any>()
    }

    pub fn set_onend(&mut self, value: &Any) {
        self.inner.set("onend", value);
    }
}
impl SpeechSynthesisUtterance {
    pub fn onerror(&self) -> Any {
        self.inner.get("onerror").as_::<Any>()
    }

    pub fn set_onerror(&mut self, value: &Any) {
        self.inner.set("onerror", value);
    }
}
impl SpeechSynthesisUtterance {
    pub fn onpause(&self) -> Any {
        self.inner.get("onpause").as_::<Any>()
    }

    pub fn set_onpause(&mut self, value: &Any) {
        self.inner.set("onpause", value);
    }
}
impl SpeechSynthesisUtterance {
    pub fn onresume(&self) -> Any {
        self.inner.get("onresume").as_::<Any>()
    }

    pub fn set_onresume(&mut self, value: &Any) {
        self.inner.set("onresume", value);
    }
}
impl SpeechSynthesisUtterance {
    pub fn onmark(&self) -> Any {
        self.inner.get("onmark").as_::<Any>()
    }

    pub fn set_onmark(&mut self, value: &Any) {
        self.inner.set("onmark", value);
    }
}
impl SpeechSynthesisUtterance {
    pub fn onboundary(&self) -> Any {
        self.inner.get("onboundary").as_::<Any>()
    }

    pub fn set_onboundary(&mut self, value: &Any) {
        self.inner.set("onboundary", value);
    }
}
