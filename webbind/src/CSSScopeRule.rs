use super::*;




/// The CSSScopeRule class.
/// [`CSSScopeRule`](https://developer.mozilla.org/en-US/docs/Web/API/CSSScopeRule)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct CSSScopeRule {
    inner: CSSGroupingRule,
}

impl FromVal for CSSScopeRule {
    fn from_val(v: &Any) -> Self {
        CSSScopeRule { inner: CSSGroupingRule::from_val(v) }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for CSSScopeRule {
    type Target = CSSGroupingRule;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for CSSScopeRule {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for CSSScopeRule {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for CSSScopeRule {
    fn as_mut(&mut self) -> &mut Any {
      &mut self.inner
  }
}

impl From<CSSScopeRule> for Any {
    fn from(s: CSSScopeRule) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&CSSScopeRule> for Any {
    fn from(s: &CSSScopeRule) -> Any {
        s.inner.clone().into()
    }
}

jsbind::utils::impl_dyn_cast!(CSSScopeRule);


impl CSSScopeRule {
    /// Getter of the `start` attribute.
    /// [`CSSScopeRule.start`](https://developer.mozilla.org/en-US/docs/Web/API/CSSScopeRule/start)
    pub fn start(&self) -> JsString {
        self.inner.get("start").as_::<JsString>()
    }

}
impl CSSScopeRule {
    /// Getter of the `end` attribute.
    /// [`CSSScopeRule.end`](https://developer.mozilla.org/en-US/docs/Web/API/CSSScopeRule/end)
    pub fn end(&self) -> JsString {
        self.inner.get("end").as_::<JsString>()
    }

}
