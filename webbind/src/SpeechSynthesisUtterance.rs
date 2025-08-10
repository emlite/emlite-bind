use super::*;

/// The SpeechSynthesisUtterance class.
/// [`SpeechSynthesisUtterance`](https://developer.mozilla.org/en-US/docs/Web/API/SpeechSynthesisUtterance)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct SpeechSynthesisUtterance {
    inner: EventTarget,
}

impl FromVal for SpeechSynthesisUtterance {
    fn from_val(v: &Any) -> Self {
        SpeechSynthesisUtterance {
            inner: EventTarget::from_val(v),
        }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
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

impl AsRef<Any> for SpeechSynthesisUtterance {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for SpeechSynthesisUtterance {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}

impl From<SpeechSynthesisUtterance> for Any {
    fn from(s: SpeechSynthesisUtterance) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&SpeechSynthesisUtterance> for Any {
    fn from(s: &SpeechSynthesisUtterance) -> Any {
        s.inner.clone().into()
    }
}

jsbind::utils::impl_dyn_cast!(SpeechSynthesisUtterance);

impl SpeechSynthesisUtterance {
    /// The `new SpeechSynthesisUtterance(..)` constructor, creating a new SpeechSynthesisUtterance instance
    pub fn new0() -> SpeechSynthesisUtterance {
        Self {
            inner: Any::global("SpeechSynthesisUtterance")
                .new(&[])
                .as_::<EventTarget>(),
        }
    }

    /// The `new SpeechSynthesisUtterance(..)` constructor, creating a new SpeechSynthesisUtterance instance
    pub fn new1(text: &JsString) -> SpeechSynthesisUtterance {
        Self {
            inner: Any::global("SpeechSynthesisUtterance")
                .new(&[text.into()])
                .as_::<EventTarget>(),
        }
    }
}
impl SpeechSynthesisUtterance {
    /// Getter of the `text` attribute.
    /// [`SpeechSynthesisUtterance.text`](https://developer.mozilla.org/en-US/docs/Web/API/SpeechSynthesisUtterance/text)
    pub fn text(&self) -> JsString {
        self.inner.get("text").as_::<JsString>()
    }

    /// Setter of the `text` attribute.
    /// [`SpeechSynthesisUtterance.text`](https://developer.mozilla.org/en-US/docs/Web/API/SpeechSynthesisUtterance/text)
    pub fn set_text(&mut self, value: &JsString) {
        self.inner.set("text", value);
    }
}
impl SpeechSynthesisUtterance {
    /// Getter of the `lang` attribute.
    /// [`SpeechSynthesisUtterance.lang`](https://developer.mozilla.org/en-US/docs/Web/API/SpeechSynthesisUtterance/lang)
    pub fn lang(&self) -> JsString {
        self.inner.get("lang").as_::<JsString>()
    }

    /// Setter of the `lang` attribute.
    /// [`SpeechSynthesisUtterance.lang`](https://developer.mozilla.org/en-US/docs/Web/API/SpeechSynthesisUtterance/lang)
    pub fn set_lang(&mut self, value: &JsString) {
        self.inner.set("lang", value);
    }
}
impl SpeechSynthesisUtterance {
    /// Getter of the `voice` attribute.
    /// [`SpeechSynthesisUtterance.voice`](https://developer.mozilla.org/en-US/docs/Web/API/SpeechSynthesisUtterance/voice)
    pub fn voice(&self) -> SpeechSynthesisVoice {
        self.inner.get("voice").as_::<SpeechSynthesisVoice>()
    }

    /// Setter of the `voice` attribute.
    /// [`SpeechSynthesisUtterance.voice`](https://developer.mozilla.org/en-US/docs/Web/API/SpeechSynthesisUtterance/voice)
    pub fn set_voice(&mut self, value: &SpeechSynthesisVoice) {
        self.inner.set("voice", value);
    }
}
impl SpeechSynthesisUtterance {
    /// Getter of the `volume` attribute.
    /// [`SpeechSynthesisUtterance.volume`](https://developer.mozilla.org/en-US/docs/Web/API/SpeechSynthesisUtterance/volume)
    pub fn volume(&self) -> f32 {
        self.inner.get("volume").as_::<f32>()
    }

    /// Setter of the `volume` attribute.
    /// [`SpeechSynthesisUtterance.volume`](https://developer.mozilla.org/en-US/docs/Web/API/SpeechSynthesisUtterance/volume)
    pub fn set_volume(&mut self, value: f32) {
        self.inner.set("volume", value);
    }
}
impl SpeechSynthesisUtterance {
    /// Getter of the `rate` attribute.
    /// [`SpeechSynthesisUtterance.rate`](https://developer.mozilla.org/en-US/docs/Web/API/SpeechSynthesisUtterance/rate)
    pub fn rate(&self) -> f32 {
        self.inner.get("rate").as_::<f32>()
    }

    /// Setter of the `rate` attribute.
    /// [`SpeechSynthesisUtterance.rate`](https://developer.mozilla.org/en-US/docs/Web/API/SpeechSynthesisUtterance/rate)
    pub fn set_rate(&mut self, value: f32) {
        self.inner.set("rate", value);
    }
}
impl SpeechSynthesisUtterance {
    /// Getter of the `pitch` attribute.
    /// [`SpeechSynthesisUtterance.pitch`](https://developer.mozilla.org/en-US/docs/Web/API/SpeechSynthesisUtterance/pitch)
    pub fn pitch(&self) -> f32 {
        self.inner.get("pitch").as_::<f32>()
    }

    /// Setter of the `pitch` attribute.
    /// [`SpeechSynthesisUtterance.pitch`](https://developer.mozilla.org/en-US/docs/Web/API/SpeechSynthesisUtterance/pitch)
    pub fn set_pitch(&mut self, value: f32) {
        self.inner.set("pitch", value);
    }
}
impl SpeechSynthesisUtterance {
    /// Getter of the `onstart` attribute.
    /// [`SpeechSynthesisUtterance.onstart`](https://developer.mozilla.org/en-US/docs/Web/API/SpeechSynthesisUtterance/onstart)
    pub fn onstart(&self) -> Any {
        self.inner.get("onstart").as_::<Any>()
    }

    /// Setter of the `onstart` attribute.
    /// [`SpeechSynthesisUtterance.onstart`](https://developer.mozilla.org/en-US/docs/Web/API/SpeechSynthesisUtterance/onstart)
    pub fn set_onstart(&mut self, value: &Any) {
        self.inner.set("onstart", value);
    }
}
impl SpeechSynthesisUtterance {
    /// Getter of the `onend` attribute.
    /// [`SpeechSynthesisUtterance.onend`](https://developer.mozilla.org/en-US/docs/Web/API/SpeechSynthesisUtterance/onend)
    pub fn onend(&self) -> Any {
        self.inner.get("onend").as_::<Any>()
    }

    /// Setter of the `onend` attribute.
    /// [`SpeechSynthesisUtterance.onend`](https://developer.mozilla.org/en-US/docs/Web/API/SpeechSynthesisUtterance/onend)
    pub fn set_onend(&mut self, value: &Any) {
        self.inner.set("onend", value);
    }
}
impl SpeechSynthesisUtterance {
    /// Getter of the `onerror` attribute.
    /// [`SpeechSynthesisUtterance.onerror`](https://developer.mozilla.org/en-US/docs/Web/API/SpeechSynthesisUtterance/onerror)
    pub fn onerror(&self) -> Any {
        self.inner.get("onerror").as_::<Any>()
    }

    /// Setter of the `onerror` attribute.
    /// [`SpeechSynthesisUtterance.onerror`](https://developer.mozilla.org/en-US/docs/Web/API/SpeechSynthesisUtterance/onerror)
    pub fn set_onerror(&mut self, value: &Any) {
        self.inner.set("onerror", value);
    }
}
impl SpeechSynthesisUtterance {
    /// Getter of the `onpause` attribute.
    /// [`SpeechSynthesisUtterance.onpause`](https://developer.mozilla.org/en-US/docs/Web/API/SpeechSynthesisUtterance/onpause)
    pub fn onpause(&self) -> Any {
        self.inner.get("onpause").as_::<Any>()
    }

    /// Setter of the `onpause` attribute.
    /// [`SpeechSynthesisUtterance.onpause`](https://developer.mozilla.org/en-US/docs/Web/API/SpeechSynthesisUtterance/onpause)
    pub fn set_onpause(&mut self, value: &Any) {
        self.inner.set("onpause", value);
    }
}
impl SpeechSynthesisUtterance {
    /// Getter of the `onresume` attribute.
    /// [`SpeechSynthesisUtterance.onresume`](https://developer.mozilla.org/en-US/docs/Web/API/SpeechSynthesisUtterance/onresume)
    pub fn onresume(&self) -> Any {
        self.inner.get("onresume").as_::<Any>()
    }

    /// Setter of the `onresume` attribute.
    /// [`SpeechSynthesisUtterance.onresume`](https://developer.mozilla.org/en-US/docs/Web/API/SpeechSynthesisUtterance/onresume)
    pub fn set_onresume(&mut self, value: &Any) {
        self.inner.set("onresume", value);
    }
}
impl SpeechSynthesisUtterance {
    /// Getter of the `onmark` attribute.
    /// [`SpeechSynthesisUtterance.onmark`](https://developer.mozilla.org/en-US/docs/Web/API/SpeechSynthesisUtterance/onmark)
    pub fn onmark(&self) -> Any {
        self.inner.get("onmark").as_::<Any>()
    }

    /// Setter of the `onmark` attribute.
    /// [`SpeechSynthesisUtterance.onmark`](https://developer.mozilla.org/en-US/docs/Web/API/SpeechSynthesisUtterance/onmark)
    pub fn set_onmark(&mut self, value: &Any) {
        self.inner.set("onmark", value);
    }
}
impl SpeechSynthesisUtterance {
    /// Getter of the `onboundary` attribute.
    /// [`SpeechSynthesisUtterance.onboundary`](https://developer.mozilla.org/en-US/docs/Web/API/SpeechSynthesisUtterance/onboundary)
    pub fn onboundary(&self) -> Any {
        self.inner.get("onboundary").as_::<Any>()
    }

    /// Setter of the `onboundary` attribute.
    /// [`SpeechSynthesisUtterance.onboundary`](https://developer.mozilla.org/en-US/docs/Web/API/SpeechSynthesisUtterance/onboundary)
    pub fn set_onboundary(&mut self, value: &Any) {
        self.inner.set("onboundary", value);
    }
}
