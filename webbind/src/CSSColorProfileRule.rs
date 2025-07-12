use super::*;

#[derive(Clone, Debug)]
pub struct CSSColorProfileRule {
    inner: CSSRule,
}
impl FromVal for CSSColorProfileRule {
    fn from_val(v: &emlite::Val) -> Self {
        CSSColorProfileRule {
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
impl std::ops::Deref for CSSColorProfileRule {
    type Target = CSSRule;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for CSSColorProfileRule {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<CSSColorProfileRule> for emlite::Val {
    fn from(s: CSSColorProfileRule) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl CSSColorProfileRule {
    pub fn name(&self) -> jsbind::CSSOMString {
        self.inner.get("name").as_::<jsbind::CSSOMString>()
    }
}
impl CSSColorProfileRule {
    pub fn src(&self) -> jsbind::CSSOMString {
        self.inner.get("src").as_::<jsbind::CSSOMString>()
    }
}
impl CSSColorProfileRule {
    pub fn rendering_intent(&self) -> jsbind::CSSOMString {
        self.inner
            .get("renderingIntent")
            .as_::<jsbind::CSSOMString>()
    }
}
impl CSSColorProfileRule {
    pub fn components(&self) -> jsbind::CSSOMString {
        self.inner.get("components").as_::<jsbind::CSSOMString>()
    }
}
