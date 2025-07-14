use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct SpeechGrammarList {
    inner: emlite::Val,
}
impl FromVal for SpeechGrammarList {
    fn from_val(v: &emlite::Val) -> Self {
        SpeechGrammarList {
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
impl core::ops::Deref for SpeechGrammarList {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for SpeechGrammarList {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<SpeechGrammarList> for emlite::Val {
    fn from(s: SpeechGrammarList) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl SpeechGrammarList {
    pub fn new() -> SpeechGrammarList {
        Self {
            inner: emlite::Val::global("SpeechGrammarList")
                .new(&[])
                .as_::<emlite::Val>(),
        }
    }
}
impl SpeechGrammarList {
    pub fn length(&self) -> u32 {
        self.inner.get("length").as_::<u32>()
    }
}
impl SpeechGrammarList {
    pub fn item(&self, index: u32) -> SpeechGrammar {
        self.inner
            .call("item", &[index.into()])
            .as_::<SpeechGrammar>()
    }
}
impl SpeechGrammarList {
    pub fn add_from_uri0(&self, src: jsbind::DOMString) -> jsbind::Undefined {
        self.inner
            .call("addFromURI", &[src.into()])
            .as_::<jsbind::Undefined>()
    }

    pub fn add_from_uri1(&self, src: jsbind::DOMString, weight: f32) -> jsbind::Undefined {
        self.inner
            .call("addFromURI", &[src.into(), weight.into()])
            .as_::<jsbind::Undefined>()
    }
}
impl SpeechGrammarList {
    pub fn add_from_string0(&self, string: jsbind::DOMString) -> jsbind::Undefined {
        self.inner
            .call("addFromString", &[string.into()])
            .as_::<jsbind::Undefined>()
    }

    pub fn add_from_string1(&self, string: jsbind::DOMString, weight: f32) -> jsbind::Undefined {
        self.inner
            .call("addFromString", &[string.into(), weight.into()])
            .as_::<jsbind::Undefined>()
    }
}
