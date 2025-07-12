use super::*;

#[derive(Clone, Debug)]
pub struct CSSConditionRule {
    inner: CSSGroupingRule,
}
impl FromVal for CSSConditionRule {
    fn from_val(v: &emlite::Val) -> Self {
        CSSConditionRule {
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
impl std::ops::Deref for CSSConditionRule {
    type Target = CSSGroupingRule;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for CSSConditionRule {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<CSSConditionRule> for emlite::Val {
    fn from(s: CSSConditionRule) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl CSSConditionRule {
    pub fn condition_text(&self) -> jsbind::CSSOMString {
        self.inner.get("conditionText").as_::<jsbind::CSSOMString>()
    }
}
