use super::*;

/// The SpeechRecognitionErrorEventInit dictionary.
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct SpeechRecognitionErrorEventInit {
    inner: Any,
}

impl FromVal for SpeechRecognitionErrorEventInit {
    fn from_val(v: &Any) -> Self {
        SpeechRecognitionErrorEventInit { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for SpeechRecognitionErrorEventInit {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for SpeechRecognitionErrorEventInit {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for SpeechRecognitionErrorEventInit {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for SpeechRecognitionErrorEventInit {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}

impl From<SpeechRecognitionErrorEventInit> for Any {
    fn from(s: SpeechRecognitionErrorEventInit) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&SpeechRecognitionErrorEventInit> for Any {
    fn from(s: &SpeechRecognitionErrorEventInit) -> Any {
        s.inner.clone()
    }
}

impl SpeechRecognitionErrorEventInit {
    /// Getter of the `error` attribute.
    pub fn error(&self) -> SpeechRecognitionErrorCode {
        self.inner.get("error").as_::<SpeechRecognitionErrorCode>()
    }

    /// Setter of the `error` attribute.
    pub fn set_error(&mut self, value: &SpeechRecognitionErrorCode) {
        self.inner.set("error", value);
    }
}
impl SpeechRecognitionErrorEventInit {
    /// Getter of the `message` attribute.
    pub fn message(&self) -> JsString {
        self.inner.get("message").as_::<JsString>()
    }

    /// Setter of the `message` attribute.
    pub fn set_message(&mut self, value: &JsString) {
        self.inner.set("message", value);
    }
}
