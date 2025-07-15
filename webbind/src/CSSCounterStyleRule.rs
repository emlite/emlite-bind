use super::*;




#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct CSSCounterStyleRule {
    inner: CSSRule,
}
impl FromVal for CSSCounterStyleRule {
    fn from_val(v: &emlite::Val) -> Self {
        CSSCounterStyleRule { inner: CSSRule::from_val(v) }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for CSSCounterStyleRule {
    type Target = CSSRule;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for CSSCounterStyleRule {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for CSSCounterStyleRule {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for CSSCounterStyleRule {
    fn as_mut(&mut self) -> &mut emlite::Val {
      &mut self.inner
  }
}
impl From<CSSCounterStyleRule> for emlite::Val {
    fn from(s: CSSCounterStyleRule) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}
jsbind::utils::impl_dyn_cast!(CSSCounterStyleRule);


impl CSSCounterStyleRule {
    pub fn name(&self) -> CSSOMString {
        self.inner.get("name").as_::<CSSOMString>()
    }

    pub fn set_name(&mut self, value: CSSOMString) {
        self.inner.set("name", value);
    }

}
impl CSSCounterStyleRule {
    pub fn system(&self) -> CSSOMString {
        self.inner.get("system").as_::<CSSOMString>()
    }

    pub fn set_system(&mut self, value: CSSOMString) {
        self.inner.set("system", value);
    }

}
impl CSSCounterStyleRule {
    pub fn symbols(&self) -> CSSOMString {
        self.inner.get("symbols").as_::<CSSOMString>()
    }

    pub fn set_symbols(&mut self, value: CSSOMString) {
        self.inner.set("symbols", value);
    }

}
impl CSSCounterStyleRule {
    pub fn additive_symbols(&self) -> CSSOMString {
        self.inner.get("additiveSymbols").as_::<CSSOMString>()
    }

    pub fn set_additive_symbols(&mut self, value: CSSOMString) {
        self.inner.set("additiveSymbols", value);
    }

}
impl CSSCounterStyleRule {
    pub fn negative(&self) -> CSSOMString {
        self.inner.get("negative").as_::<CSSOMString>()
    }

    pub fn set_negative(&mut self, value: CSSOMString) {
        self.inner.set("negative", value);
    }

}
impl CSSCounterStyleRule {
    pub fn prefix(&self) -> CSSOMString {
        self.inner.get("prefix").as_::<CSSOMString>()
    }

    pub fn set_prefix(&mut self, value: CSSOMString) {
        self.inner.set("prefix", value);
    }

}
impl CSSCounterStyleRule {
    pub fn suffix(&self) -> CSSOMString {
        self.inner.get("suffix").as_::<CSSOMString>()
    }

    pub fn set_suffix(&mut self, value: CSSOMString) {
        self.inner.set("suffix", value);
    }

}
impl CSSCounterStyleRule {
    pub fn range(&self) -> CSSOMString {
        self.inner.get("range").as_::<CSSOMString>()
    }

    pub fn set_range(&mut self, value: CSSOMString) {
        self.inner.set("range", value);
    }

}
impl CSSCounterStyleRule {
    pub fn pad(&self) -> CSSOMString {
        self.inner.get("pad").as_::<CSSOMString>()
    }

    pub fn set_pad(&mut self, value: CSSOMString) {
        self.inner.set("pad", value);
    }

}
impl CSSCounterStyleRule {
    pub fn speak_as(&self) -> CSSOMString {
        self.inner.get("speakAs").as_::<CSSOMString>()
    }

    pub fn set_speak_as(&mut self, value: CSSOMString) {
        self.inner.set("speakAs", value);
    }

}
impl CSSCounterStyleRule {
    pub fn fallback(&self) -> CSSOMString {
        self.inner.get("fallback").as_::<CSSOMString>()
    }

    pub fn set_fallback(&mut self, value: CSSOMString) {
        self.inner.set("fallback", value);
    }

}
