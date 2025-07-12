use super::*;

#[derive(Clone, Debug)]
pub struct SpeechRecognitionPhrase {
    inner: emlite::Val,
}
impl FromVal for SpeechRecognitionPhrase {
    fn from_val(v: &emlite::Val) -> Self {
        SpeechRecognitionPhrase {
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
impl std::ops::Deref for SpeechRecognitionPhrase {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for SpeechRecognitionPhrase {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<SpeechRecognitionPhrase> for emlite::Val {
    fn from(s: SpeechRecognitionPhrase) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl SpeechRecognitionPhrase {
    pub fn new0(phrase: jsbind::DOMString) -> SpeechRecognitionPhrase {
        Self {
            inner: emlite::Val::global("SpeechRecognitionPhrase")
                .new(&[phrase.into()])
                .as_::<emlite::Val>(),
        }
    }

    pub fn new1(phrase: jsbind::DOMString, boost: f32) -> SpeechRecognitionPhrase {
        Self {
            inner: emlite::Val::global("SpeechRecognitionPhrase")
                .new(&[phrase.into(), boost.into()])
                .as_::<emlite::Val>(),
        }
    }
}
impl SpeechRecognitionPhrase {
    pub fn phrase(&self) -> jsbind::DOMString {
        self.inner.get("phrase").as_::<jsbind::DOMString>()
    }
}
impl SpeechRecognitionPhrase {
    pub fn boost(&self) -> f32 {
        self.inner.get("boost").as_::<f32>()
    }
}
