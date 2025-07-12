use super::*;

#[derive(Clone, Debug)]
pub struct CSSContainerRule {
    inner: CSSConditionRule,
}
impl FromVal for CSSContainerRule {
    fn from_val(v: &emlite::Val) -> Self {
        CSSContainerRule {
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
impl std::ops::Deref for CSSContainerRule {
    type Target = CSSConditionRule;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for CSSContainerRule {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<CSSContainerRule> for emlite::Val {
    fn from(s: CSSContainerRule) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl CSSContainerRule {
    pub fn container_name(&self) -> jsbind::CSSOMString {
        self.inner.get("containerName").as_::<jsbind::CSSOMString>()
    }
}
impl CSSContainerRule {
    pub fn container_query(&self) -> jsbind::CSSOMString {
        self.inner
            .get("containerQuery")
            .as_::<jsbind::CSSOMString>()
    }
}
