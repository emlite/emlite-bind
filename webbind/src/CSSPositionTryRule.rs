use super::*;




/// The CSSPositionTryRule class.
/// [`CSSPositionTryRule`](https://developer.mozilla.org/en-US/docs/Web/API/CSSPositionTryRule)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct CSSPositionTryRule {
    inner: CSSRule,
}

impl FromVal for CSSPositionTryRule {
    fn from_val(v: &Any) -> Self {
        CSSPositionTryRule { inner: CSSRule::from_val(v) }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for CSSPositionTryRule {
    type Target = CSSRule;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for CSSPositionTryRule {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for CSSPositionTryRule {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for CSSPositionTryRule {
    fn as_mut(&mut self) -> &mut Any {
      &mut self.inner
  }
}

impl From<CSSPositionTryRule> for Any {
    fn from(s: CSSPositionTryRule) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&CSSPositionTryRule> for Any {
    fn from(s: &CSSPositionTryRule) -> Any {
        s.inner.clone().into()
    }
}

jsbind::utils::impl_dyn_cast!(CSSPositionTryRule);


impl CSSPositionTryRule {
    /// Getter of the `name` attribute.
    /// [`CSSPositionTryRule.name`](https://developer.mozilla.org/en-US/docs/Web/API/CSSPositionTryRule/name)
    pub fn name(&self) -> JsString {
        self.inner.get("name").as_::<JsString>()
    }

}
impl CSSPositionTryRule {
    /// Getter of the `style` attribute.
    /// [`CSSPositionTryRule.style`](https://developer.mozilla.org/en-US/docs/Web/API/CSSPositionTryRule/style)
    pub fn style(&self) -> CSSPositionTryDescriptors {
        self.inner.get("style").as_::<CSSPositionTryDescriptors>()
    }

}
