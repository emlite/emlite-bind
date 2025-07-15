use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct SpeechGrammar {
    inner: emlite::Val,
}
impl FromVal for SpeechGrammar {
    fn from_val(v: &emlite::Val) -> Self {
        SpeechGrammar {
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
impl core::ops::Deref for SpeechGrammar {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for SpeechGrammar {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for SpeechGrammar {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for SpeechGrammar {
    fn as_mut(&mut self) -> &mut emlite::Val {
        &mut self.inner
    }
}
impl From<SpeechGrammar> for emlite::Val {
    fn from(s: SpeechGrammar) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}
jsbind::utils::impl_dyn_cast!(SpeechGrammar);

impl SpeechGrammar {
    pub fn src(&self) -> DOMString {
        self.inner.get("src").as_::<DOMString>()
    }

    pub fn set_src(&mut self, value: DOMString) {
        self.inner.set("src", value);
    }
}
impl SpeechGrammar {
    pub fn weight(&self) -> f32 {
        self.inner.get("weight").as_::<f32>()
    }

    pub fn set_weight(&mut self, value: f32) {
        self.inner.set("weight", value);
    }
}
