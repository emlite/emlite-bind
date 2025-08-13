use super::*;




/// The CSSKeyframeRule class.
/// [`CSSKeyframeRule`](https://developer.mozilla.org/en-US/docs/Web/API/CSSKeyframeRule)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct CSSKeyframeRule {
    inner: CSSRule,
}

impl FromVal for CSSKeyframeRule {
    fn from_val(v: &Any) -> Self {
        CSSKeyframeRule { inner: CSSRule::from_val(v) }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for CSSKeyframeRule {
    type Target = CSSRule;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for CSSKeyframeRule {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for CSSKeyframeRule {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for CSSKeyframeRule {
    fn as_mut(&mut self) -> &mut Any {
      &mut self.inner
  }
}

impl From<CSSKeyframeRule> for Any {
    fn from(s: CSSKeyframeRule) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&CSSKeyframeRule> for Any {
    fn from(s: &CSSKeyframeRule) -> Any {
        s.inner.clone().into()
    }
}

jsbind::utils::impl_dyn_cast!(CSSKeyframeRule);


impl CSSKeyframeRule {
    /// Getter of the `keyText` attribute.
    /// [`CSSKeyframeRule.keyText`](https://developer.mozilla.org/en-US/docs/Web/API/CSSKeyframeRule/keyText)
    pub fn key_text(&self) -> JsString {
        self.inner.get("keyText").as_::<JsString>()
    }

    /// Setter of the `keyText` attribute.
    /// [`CSSKeyframeRule.keyText`](https://developer.mozilla.org/en-US/docs/Web/API/CSSKeyframeRule/keyText)
    pub fn set_key_text(&mut self, value: &JsString) {
        self.inner.set("keyText", value);
    }
}
impl CSSKeyframeRule {
    /// Getter of the `style` attribute.
    /// [`CSSKeyframeRule.style`](https://developer.mozilla.org/en-US/docs/Web/API/CSSKeyframeRule/style)
    pub fn style(&self) -> CSSStyleProperties {
        self.inner.get("style").as_::<CSSStyleProperties>()
    }

}
