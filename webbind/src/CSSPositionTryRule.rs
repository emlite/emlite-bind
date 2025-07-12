use super::*;

#[derive(Clone, Debug)]
pub struct CSSPositionTryRule {
    inner: CSSRule,
}
impl FromVal for CSSPositionTryRule {
    fn from_val(v: &emlite::Val) -> Self {
        CSSPositionTryRule {
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
impl std::ops::Deref for CSSPositionTryRule {
    type Target = CSSRule;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for CSSPositionTryRule {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<CSSPositionTryRule> for emlite::Val {
    fn from(s: CSSPositionTryRule) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl CSSPositionTryRule {
    pub fn name(&self) -> jsbind::CSSOMString {
        self.inner.get("name").as_::<jsbind::CSSOMString>()
    }
}
impl CSSPositionTryRule {
    pub fn style(&self) -> CSSPositionTryDescriptors {
        self.inner.get("style").as_::<CSSPositionTryDescriptors>()
    }
}
