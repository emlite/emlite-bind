use super::*;




#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct SpeechRecognitionResult {
    inner: emlite::Val,
}
impl FromVal for SpeechRecognitionResult {
    fn from_val(v: &emlite::Val) -> Self {
        SpeechRecognitionResult { inner: emlite::Val::from_val(v) }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for SpeechRecognitionResult {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for SpeechRecognitionResult {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for SpeechRecognitionResult {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for SpeechRecognitionResult {
    fn as_mut(&mut self) -> &mut emlite::Val {
      &mut self.inner
  }
}
impl From<SpeechRecognitionResult> for emlite::Val {
    fn from(s: SpeechRecognitionResult) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}
jsbind::utils::impl_dyn_cast!(SpeechRecognitionResult);


impl SpeechRecognitionResult {
    pub fn length(&self) -> u32 {
        self.inner.get("length").as_::<u32>()
    }

}
impl SpeechRecognitionResult {
    pub fn item(&self, index: u32) -> SpeechRecognitionAlternative {
        self.inner.call("item", &[index.into(), ]).as_::<SpeechRecognitionAlternative>()
    }

}
impl SpeechRecognitionResult {
    pub fn is_final(&self) -> bool {
        self.inner.get("isFinal").as_::<bool>()
    }

}
