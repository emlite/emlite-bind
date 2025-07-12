use super::*;

#[derive(Clone, Debug)]
pub struct CSSStartingStyleRule {
    inner: CSSGroupingRule,
}
impl FromVal for CSSStartingStyleRule {
    fn from_val(v: &emlite::Val) -> Self {
        CSSStartingStyleRule {
            inner: CSSGroupingRule::from_val(v),
        }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl std::ops::Deref for CSSStartingStyleRule {
    type Target = CSSGroupingRule;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for CSSStartingStyleRule {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<CSSStartingStyleRule> for emlite::Val {
    fn from(s: CSSStartingStyleRule) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}
