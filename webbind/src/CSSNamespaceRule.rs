use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
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
impl core::ops::Deref for CSSNamespaceRule {
    type Target = CSSRule;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for CSSNamespaceRule {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for CSSNamespaceRule {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for CSSNamespaceRule {
    fn as_mut(&mut self) -> &mut emlite::Val {
        &mut self.inner
    }
}
impl From<CSSNamespaceRule> for emlite::Val {
    fn from(s: CSSNamespaceRule) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}
jsbind::utils::impl_dyn_cast!(CSSNamespaceRule);

impl CSSNamespaceRule {
    pub fn namespace_uri(&self) -> CSSOMString {
        self.inner.get("namespaceURI").as_::<CSSOMString>()
    }
}
impl CSSNamespaceRule {
    pub fn prefix(&self) -> CSSOMString {
        self.inner.get("prefix").as_::<CSSOMString>()
    }
}
