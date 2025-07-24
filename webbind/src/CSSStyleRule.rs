use super::*;

/// The CSSStyleRule class.
/// [`CSSStyleRule`](https://developer.mozilla.org/en-US/docs/Web/API/CSSStyleRule)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct CSSStyleRule {
    inner: CSSGroupingRule,
}
impl FromVal for CSSStyleRule {
    fn from_val(v: &Any) -> Self {
        CSSStyleRule {
            inner: CSSGroupingRule::from_val(v),
        }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for CSSStyleRule {
    type Target = CSSGroupingRule;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for CSSStyleRule {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for CSSStyleRule {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for CSSStyleRule {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<CSSStyleRule> for Any {
    fn from(s: CSSStyleRule) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&CSSStyleRule> for Any {
    fn from(s: &CSSStyleRule) -> Any {
        s.inner.clone().into()
    }
}
jsbind::utils::impl_dyn_cast!(CSSStyleRule);

impl CSSStyleRule {
    /// Getter of the `selectorText` attribute.
    /// [`CSSStyleRule.selectorText`](https://developer.mozilla.org/en-US/docs/Web/API/CSSStyleRule/selectorText)
    pub fn selector_text(&self) -> CSSOMString {
        self.inner.get("selectorText").as_::<CSSOMString>()
    }

    /// Setter of the `selectorText` attribute.
    /// [`CSSStyleRule.selectorText`](https://developer.mozilla.org/en-US/docs/Web/API/CSSStyleRule/selectorText)
    pub fn set_selector_text(&mut self, value: &CSSOMString) {
        self.inner.set("selectorText", value);
    }
}
impl CSSStyleRule {
    /// Getter of the `style` attribute.
    /// [`CSSStyleRule.style`](https://developer.mozilla.org/en-US/docs/Web/API/CSSStyleRule/style)
    pub fn style(&self) -> CSSStyleProperties {
        self.inner.get("style").as_::<CSSStyleProperties>()
    }
}
impl CSSStyleRule {
    /// Getter of the `styleMap` attribute.
    /// [`CSSStyleRule.styleMap`](https://developer.mozilla.org/en-US/docs/Web/API/CSSStyleRule/styleMap)
    pub fn style_map(&self) -> StylePropertyMap {
        self.inner.get("styleMap").as_::<StylePropertyMap>()
    }
}
