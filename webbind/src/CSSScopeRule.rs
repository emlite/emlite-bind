use super::*;




#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct CSSScopeRule {
    inner: CSSGroupingRule,
}
impl FromVal for CSSScopeRule {
    fn from_val(v: &emlite::Val) -> Self {
        CSSScopeRule { inner: CSSGroupingRule::from_val(v) }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for CSSScopeRule {
    type Target = CSSGroupingRule;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for CSSScopeRule {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for CSSScopeRule {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for CSSScopeRule {
    fn as_mut(&mut self) -> &mut emlite::Val {
      &mut self.inner
  }
}
impl From<CSSScopeRule> for emlite::Val {
    fn from(s: CSSScopeRule) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}
jsbind::utils::impl_dyn_cast!(CSSScopeRule);


impl CSSScopeRule {
    pub fn start(&self) -> CSSOMString {
        self.inner.get("start").as_::<CSSOMString>()
    }

}
impl CSSScopeRule {
    pub fn end(&self) -> CSSOMString {
        self.inner.get("end").as_::<CSSOMString>()
    }

}
