use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct CSSViewTransitionRule {
    inner: CSSRule,
}
impl FromVal for CSSViewTransitionRule {
    fn from_val(v: &emlite::Val) -> Self {
        CSSViewTransitionRule {
            inner: CSSRule::from_val(v),
        }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for CSSViewTransitionRule {
    type Target = CSSRule;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for CSSViewTransitionRule {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for CSSViewTransitionRule {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for CSSViewTransitionRule {
    fn as_mut(&mut self) -> &mut emlite::Val {
        &mut self.inner
    }
}
impl From<CSSViewTransitionRule> for emlite::Val {
    fn from(s: CSSViewTransitionRule) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}
impl From<&CSSViewTransitionRule> for emlite::Val {
    fn from(s: &CSSViewTransitionRule) -> emlite::Val {
        s.inner.clone().into()
    }
}
jsbind::utils::impl_dyn_cast!(CSSViewTransitionRule);

impl CSSViewTransitionRule {
    pub fn navigation(&self) -> CSSOMString {
        self.inner.get("navigation").as_::<CSSOMString>()
    }
}
impl CSSViewTransitionRule {
    pub fn types(&self) -> FrozenArray<CSSOMString> {
        self.inner.get("types").as_::<FrozenArray<CSSOMString>>()
    }
}
