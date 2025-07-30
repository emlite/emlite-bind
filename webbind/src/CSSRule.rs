use super::*;

/// The CSSRule class.
/// [`CSSRule`](https://developer.mozilla.org/en-US/docs/Web/API/CSSRule)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct CSSRule {
    inner: Any,
}
impl FromVal for CSSRule {
    fn from_val(v: &Any) -> Self {
        CSSRule {
            inner: Any::from_val(v),
        }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for CSSRule {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for CSSRule {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for CSSRule {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for CSSRule {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<CSSRule> for Any {
    fn from(s: CSSRule) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&CSSRule> for Any {
    fn from(s: &CSSRule) -> Any {
        s.inner.clone().into()
    }
}
jsbind::utils::impl_dyn_cast!(CSSRule);

impl CSSRule {
    /// Getter of the `cssText` attribute.
    /// [`CSSRule.cssText`](https://developer.mozilla.org/en-US/docs/Web/API/CSSRule/cssText)
    pub fn css_text(&self) -> JsString {
        self.inner.get("cssText").as_::<JsString>()
    }

    /// Setter of the `cssText` attribute.
    /// [`CSSRule.cssText`](https://developer.mozilla.org/en-US/docs/Web/API/CSSRule/cssText)
    pub fn set_css_text(&mut self, value: &JsString) {
        self.inner.set("cssText", value);
    }
}
impl CSSRule {
    /// Getter of the `parentRule` attribute.
    /// [`CSSRule.parentRule`](https://developer.mozilla.org/en-US/docs/Web/API/CSSRule/parentRule)
    pub fn parent_rule(&self) -> CSSRule {
        self.inner.get("parentRule").as_::<CSSRule>()
    }
}
impl CSSRule {
    /// Getter of the `parentStyleSheet` attribute.
    /// [`CSSRule.parentStyleSheet`](https://developer.mozilla.org/en-US/docs/Web/API/CSSRule/parentStyleSheet)
    pub fn parent_style_sheet(&self) -> CSSStyleSheet {
        self.inner.get("parentStyleSheet").as_::<CSSStyleSheet>()
    }
}
impl CSSRule {
    /// Getter of the `type` attribute.
    /// [`CSSRule.type`](https://developer.mozilla.org/en-US/docs/Web/API/CSSRule/type)
    pub fn type_(&self) -> u16 {
        self.inner.get("type").as_::<u16>()
    }
}
