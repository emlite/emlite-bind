use super::*;

/// The CSSCustomMediaRule class.
/// [`CSSCustomMediaRule`](https://developer.mozilla.org/en-US/docs/Web/API/CSSCustomMediaRule)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct CSSCustomMediaRule {
    inner: CSSRule,
}

impl FromVal for CSSCustomMediaRule {
    fn from_val(v: &Any) -> Self {
        CSSCustomMediaRule {
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

impl core::ops::Deref for CSSCustomMediaRule {
    type Target = CSSRule;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for CSSCustomMediaRule {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for CSSCustomMediaRule {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for CSSCustomMediaRule {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}

impl From<CSSCustomMediaRule> for Any {
    fn from(s: CSSCustomMediaRule) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&CSSCustomMediaRule> for Any {
    fn from(s: &CSSCustomMediaRule) -> Any {
        s.inner.clone().into()
    }
}

jsbind::utils::impl_dyn_cast!(CSSCustomMediaRule);

impl CSSCustomMediaRule {
    /// Getter of the `name` attribute.
    /// [`CSSCustomMediaRule.name`](https://developer.mozilla.org/en-US/docs/Web/API/CSSCustomMediaRule/name)
    pub fn name(&self) -> JsString {
        self.inner.get("name").as_::<JsString>()
    }
}
impl CSSCustomMediaRule {
    /// Getter of the `query` attribute.
    /// [`CSSCustomMediaRule.query`](https://developer.mozilla.org/en-US/docs/Web/API/CSSCustomMediaRule/query)
    pub fn query(&self) -> Any {
        self.inner.get("query").as_::<Any>()
    }
}
