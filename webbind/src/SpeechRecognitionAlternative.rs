use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
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
impl AsRef<emlite::Val> for SpeechRecognitionAlternative {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for SpeechRecognitionAlternative {
    fn as_mut(&mut self) -> &mut emlite::Val {
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
impl From<&SpeechRecognitionAlternative> for emlite::Val {
    fn from(s: &SpeechRecognitionAlternative) -> emlite::Val {
        s.inner.clone().into()
    }
}
jsbind::utils::impl_dyn_cast!(SpeechRecognitionAlternative);

impl SpeechRecognitionAlternative {
    pub fn transcript(&self) -> DOMString {
        self.inner.get("transcript").as_::<DOMString>()
    }
}
impl SpeechRecognitionAlternative {
    pub fn confidence(&self) -> f32 {
        self.inner.get("confidence").as_::<f32>()
    }
}
