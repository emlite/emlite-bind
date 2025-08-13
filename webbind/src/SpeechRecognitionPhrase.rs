use super::*;




/// The SpeechRecognitionPhrase class.
/// [`SpeechRecognitionPhrase`](https://developer.mozilla.org/en-US/docs/Web/API/SpeechRecognitionPhrase)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct SpeechRecognitionPhrase {
    inner: Any,
}

impl FromVal for SpeechRecognitionPhrase {
    fn from_val(v: &Any) -> Self {
        SpeechRecognitionPhrase { inner: Any::from_val(v) }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for SpeechRecognitionPhrase {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for SpeechRecognitionPhrase {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for SpeechRecognitionPhrase {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for SpeechRecognitionPhrase {
    fn as_mut(&mut self) -> &mut Any {
      &mut self.inner
  }
}

impl From<SpeechRecognitionPhrase> for Any {
    fn from(s: SpeechRecognitionPhrase) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&SpeechRecognitionPhrase> for Any {
    fn from(s: &SpeechRecognitionPhrase) -> Any {
        s.inner.clone().into()
    }
}

jsbind::utils::impl_dyn_cast!(SpeechRecognitionPhrase);



impl SpeechRecognitionPhrase {
    /// The `new SpeechRecognitionPhrase(..)` constructor, creating a new SpeechRecognitionPhrase instance
    pub fn new0(phrase: &JsString) -> SpeechRecognitionPhrase {
        Self {
            inner: Any::global("SpeechRecognitionPhrase").new(&[phrase.into()]).as_::<Any>(),
        }
    }

    /// The `new SpeechRecognitionPhrase(..)` constructor, creating a new SpeechRecognitionPhrase instance
    pub fn new1(phrase: &JsString, boost: f32) -> SpeechRecognitionPhrase {
        Self {
            inner: Any::global("SpeechRecognitionPhrase").new(&[phrase.into(), boost.into()]).as_::<Any>(),
        }
    }

}
impl SpeechRecognitionPhrase {
    /// Getter of the `phrase` attribute.
    /// [`SpeechRecognitionPhrase.phrase`](https://developer.mozilla.org/en-US/docs/Web/API/SpeechRecognitionPhrase/phrase)
    pub fn phrase(&self) -> JsString {
        self.inner.get("phrase").as_::<JsString>()
    }

}
impl SpeechRecognitionPhrase {
    /// Getter of the `boost` attribute.
    /// [`SpeechRecognitionPhrase.boost`](https://developer.mozilla.org/en-US/docs/Web/API/SpeechRecognitionPhrase/boost)
    pub fn boost(&self) -> f32 {
        self.inner.get("boost").as_::<f32>()
    }

}
