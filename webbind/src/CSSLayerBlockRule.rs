use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct CSSLayerBlockRule {
    inner: CSSGroupingRule,
}
impl FromVal for CSSLayerBlockRule {
    fn from_val(v: &emlite::Val) -> Self {
        CSSLayerBlockRule {
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
impl core::ops::Deref for CSSLayerBlockRule {
    type Target = CSSGroupingRule;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for CSSLayerBlockRule {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<CSSLayerBlockRule> for emlite::Val {
    fn from(s: CSSLayerBlockRule) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl CSSLayerBlockRule {
    pub fn name(&self) -> jsbind::CSSOMString {
        self.inner.get("name").as_::<jsbind::CSSOMString>()
    }
}
