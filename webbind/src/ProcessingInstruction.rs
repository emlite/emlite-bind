use super::*;




#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct ProcessingInstruction {
    inner: CharacterData,
}
impl FromVal for ProcessingInstruction {
    fn from_val(v: &emlite::Val) -> Self {
        ProcessingInstruction { inner: CharacterData::from_val(v) }
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
impl AsRef<emlite::Val> for ProcessingInstruction {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for ProcessingInstruction {
    fn as_mut(&mut self) -> &mut emlite::Val {
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
jsbind::utils::impl_dyn_cast!(ProcessingInstruction);


impl ProcessingInstruction {
    pub fn target(&self) -> DOMString {
        self.inner.get("target").as_::<DOMString>()
    }

}
impl ProcessingInstruction {
    pub fn sheet(&self) -> CSSStyleSheet {
        self.inner.get("sheet").as_::<CSSStyleSheet>()
    }

}
