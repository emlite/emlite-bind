use super::*;

/// The SpeechRecognitionAlternative class.
/// [`SpeechRecognitionAlternative`](https://developer.mozilla.org/en-US/docs/Web/API/SpeechRecognitionAlternative)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct SpeechRecognitionAlternative {
    inner: Any,
}

impl FromVal for SpeechRecognitionAlternative {
    fn from_val(v: &Any) -> Self {
        SpeechRecognitionAlternative {
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

impl core::ops::Deref for SpeechRecognitionAlternative {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for SpeechRecognitionAlternative {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for SpeechRecognitionAlternative {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for SpeechRecognitionAlternative {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}

impl From<SpeechRecognitionAlternative> for Any {
    fn from(s: SpeechRecognitionAlternative) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&SpeechRecognitionAlternative> for Any {
    fn from(s: &SpeechRecognitionAlternative) -> Any {
        s.inner.clone().into()
    }
}

jsbind::utils::impl_dyn_cast!(SpeechRecognitionAlternative);

impl SpeechRecognitionAlternative {
    /// Getter of the `transcript` attribute.
    /// [`SpeechRecognitionAlternative.transcript`](https://developer.mozilla.org/en-US/docs/Web/API/SpeechRecognitionAlternative/transcript)
    pub fn transcript(&self) -> JsString {
        self.inner.get("transcript").as_::<JsString>()
    }
}
impl SpeechRecognitionAlternative {
    /// Getter of the `confidence` attribute.
    /// [`SpeechRecognitionAlternative.confidence`](https://developer.mozilla.org/en-US/docs/Web/API/SpeechRecognitionAlternative/confidence)
    pub fn confidence(&self) -> f32 {
        self.inner.get("confidence").as_::<f32>()
    }
}
