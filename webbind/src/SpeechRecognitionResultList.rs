use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct SpeechRecognitionResultList {
    inner: emlite::Val,
}
impl FromVal for SpeechRecognitionResultList {
    fn from_val(v: &emlite::Val) -> Self {
        SpeechRecognitionResultList {
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
impl core::ops::Deref for SpeechRecognitionResultList {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for SpeechRecognitionResultList {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<SpeechRecognitionResultList> for emlite::Val {
    fn from(s: SpeechRecognitionResultList) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl SpeechRecognitionResultList {
    pub fn length(&self) -> u32 {
        self.inner.get("length").as_::<u32>()
    }
}
impl SpeechRecognitionResultList {
    pub fn item(&self, index: u32) -> SpeechRecognitionResult {
        self.inner
            .call("item", &[index.into()])
            .as_::<SpeechRecognitionResult>()
    }
}
