use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct SpeechRecognitionAlternative {
    inner: emlite::Val,
}
impl FromVal for SpeechRecognitionAlternative {
    fn from_val(v: &emlite::Val) -> Self {
        SpeechRecognitionAlternative {
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
impl core::ops::Deref for SpeechRecognitionAlternative {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for SpeechRecognitionAlternative {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<SpeechRecognitionAlternative> for emlite::Val {
    fn from(s: SpeechRecognitionAlternative) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl SpeechRecognitionAlternative {
    pub fn transcript(&self) -> jsbind::DOMString {
        self.inner.get("transcript").as_::<jsbind::DOMString>()
    }
}
impl SpeechRecognitionAlternative {
    pub fn confidence(&self) -> f32 {
        self.inner.get("confidence").as_::<f32>()
    }
}
