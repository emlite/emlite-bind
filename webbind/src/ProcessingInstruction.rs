use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct ProcessingInstruction {
    inner: CharacterData,
}
impl FromVal for ProcessingInstruction {
    fn from_val(v: &emlite::Val) -> Self {
        ProcessingInstruction {
            inner: CharacterData::from_val(v),
        }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for ProcessingInstruction {
    type Target = CharacterData;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for ProcessingInstruction {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<ProcessingInstruction> for emlite::Val {
    fn from(s: ProcessingInstruction) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl ProcessingInstruction {
    pub fn target(&self) -> jsbind::DOMString {
        self.inner.get("target").as_::<jsbind::DOMString>()
    }
}
impl ProcessingInstruction {
    pub fn sheet(&self) -> CSSStyleSheet {
        self.inner.get("sheet").as_::<CSSStyleSheet>()
    }
}
