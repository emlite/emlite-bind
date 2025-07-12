use super::*;

#[derive(Clone, Debug)]
pub struct CSSSupportsRule {
    inner: CSSConditionRule,
}
impl FromVal for CSSSupportsRule {
    fn from_val(v: &emlite::Val) -> Self {
        CSSSupportsRule {
            inner: CSSConditionRule::from_val(v),
        }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl std::ops::Deref for CSSSupportsRule {
    type Target = CSSConditionRule;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for CSSSupportsRule {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<CSSSupportsRule> for emlite::Val {
    fn from(s: CSSSupportsRule) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl CSSSupportsRule {
    pub fn matches(&self) -> bool {
        self.inner.get("matches").as_::<bool>()
    }
}
