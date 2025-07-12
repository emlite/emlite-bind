use super::*;

#[derive(Clone, Debug)]
pub struct CSSMediaRule {
    inner: CSSConditionRule,
}
impl FromVal for CSSMediaRule {
    fn from_val(v: &emlite::Val) -> Self {
        CSSMediaRule {
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
impl std::ops::Deref for CSSMediaRule {
    type Target = CSSConditionRule;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for CSSMediaRule {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<CSSMediaRule> for emlite::Val {
    fn from(s: CSSMediaRule) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl CSSMediaRule {
    pub fn media(&self) -> MediaList {
        self.inner.get("media").as_::<MediaList>()
    }
}
impl CSSMediaRule {
    pub fn matches(&self) -> bool {
        self.inner.get("matches").as_::<bool>()
    }
}
