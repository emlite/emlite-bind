use super::*;

#[derive(Clone, Debug)]
pub struct CSSNamespaceRule {
    inner: CSSRule,
}
impl FromVal for CSSNamespaceRule {
    fn from_val(v: &emlite::Val) -> Self {
        CSSNamespaceRule {
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
impl std::ops::Deref for CSSNamespaceRule {
    type Target = CSSRule;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for CSSNamespaceRule {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<CSSNamespaceRule> for emlite::Val {
    fn from(s: CSSNamespaceRule) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl CSSNamespaceRule {
    pub fn namespace_uri(&self) -> jsbind::CSSOMString {
        self.inner.get("namespaceURI").as_::<jsbind::CSSOMString>()
    }
}
impl CSSNamespaceRule {
    pub fn prefix(&self) -> jsbind::CSSOMString {
        self.inner.get("prefix").as_::<jsbind::CSSOMString>()
    }
}
