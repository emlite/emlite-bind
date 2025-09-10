use super::*;

/// The CSSNamespaceRule class.
/// [`CSSNamespaceRule`](https://developer.mozilla.org/en-US/docs/Web/API/CSSNamespaceRule)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct CSSNamespaceRule {
    inner: CSSRule,
}

impl FromVal for CSSNamespaceRule {
    fn from_val(v: &Any) -> Self {
        CSSNamespaceRule {
            inner: CSSRule::from_val(v),
        }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
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

impl AsRef<Any> for CSSNamespaceRule {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for CSSNamespaceRule {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}

impl From<CSSNamespaceRule> for Any {
    fn from(s: CSSNamespaceRule) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&CSSNamespaceRule> for Any {
    fn from(s: &CSSNamespaceRule) -> Any {
        s.inner.clone().into()
    }
}

jsbind::utils::impl_dyn_cast!(CSSNamespaceRule);

impl CSSNamespaceRule {
    /// Getter of the `namespaceURI` attribute.
    /// [`CSSNamespaceRule.namespaceURI`](https://developer.mozilla.org/en-US/docs/Web/API/CSSNamespaceRule/namespaceURI)
    pub fn namespace_uri(&self) -> JsString {
        self.inner.get("namespaceURI").as_::<JsString>()
    }
}
impl CSSNamespaceRule {
    /// Getter of the `prefix` attribute.
    /// [`CSSNamespaceRule.prefix`](https://developer.mozilla.org/en-US/docs/Web/API/CSSNamespaceRule/prefix)
    pub fn prefix(&self) -> JsString {
        self.inner.get("prefix").as_::<JsString>()
    }
}
