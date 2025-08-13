use super::*;




/// The SpeechSynthesisVoice class.
/// [`SpeechSynthesisVoice`](https://developer.mozilla.org/en-US/docs/Web/API/SpeechSynthesisVoice)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct SpeechSynthesisVoice {
    inner: Any,
}

impl FromVal for SpeechSynthesisVoice {
    fn from_val(v: &Any) -> Self {
        SpeechSynthesisVoice { inner: Any::from_val(v) }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for SpeechSynthesisVoice {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for SpeechSynthesisVoice {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for SpeechSynthesisVoice {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for SpeechSynthesisVoice {
    fn as_mut(&mut self) -> &mut Any {
      &mut self.inner
  }
}

impl From<SpeechSynthesisVoice> for Any {
    fn from(s: SpeechSynthesisVoice) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&SpeechSynthesisVoice> for Any {
    fn from(s: &SpeechSynthesisVoice) -> Any {
        s.inner.clone().into()
    }
}

jsbind::utils::impl_dyn_cast!(SpeechSynthesisVoice);


impl SpeechSynthesisVoice {
    /// Getter of the `voiceURI` attribute.
    /// [`SpeechSynthesisVoice.voiceURI`](https://developer.mozilla.org/en-US/docs/Web/API/SpeechSynthesisVoice/voiceURI)
    pub fn voice_uri(&self) -> JsString {
        self.inner.get("voiceURI").as_::<JsString>()
    }

}
impl SpeechSynthesisVoice {
    /// Getter of the `name` attribute.
    /// [`SpeechSynthesisVoice.name`](https://developer.mozilla.org/en-US/docs/Web/API/SpeechSynthesisVoice/name)
    pub fn name(&self) -> JsString {
        self.inner.get("name").as_::<JsString>()
    }

}
impl SpeechSynthesisVoice {
    /// Getter of the `lang` attribute.
    /// [`SpeechSynthesisVoice.lang`](https://developer.mozilla.org/en-US/docs/Web/API/SpeechSynthesisVoice/lang)
    pub fn lang(&self) -> JsString {
        self.inner.get("lang").as_::<JsString>()
    }

}
impl SpeechSynthesisVoice {
    /// Getter of the `localService` attribute.
    /// [`SpeechSynthesisVoice.localService`](https://developer.mozilla.org/en-US/docs/Web/API/SpeechSynthesisVoice/localService)
    pub fn local_service(&self) -> bool {
        self.inner.get("localService").as_::<bool>()
    }

}
impl SpeechSynthesisVoice {
    /// Getter of the `default` attribute.
    /// [`SpeechSynthesisVoice.default`](https://developer.mozilla.org/en-US/docs/Web/API/SpeechSynthesisVoice/default)
    pub fn default(&self) -> bool {
        self.inner.get("default").as_::<bool>()
    }

}
