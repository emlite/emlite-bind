use super::*;

/// The CSSPageRule class.
/// [`CSSPageRule`](https://developer.mozilla.org/en-US/docs/Web/API/CSSPageRule)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct CSSPageRule {
    inner: CSSGroupingRule,
}
impl FromVal for CSSPageRule {
    fn from_val(v: &Any) -> Self {
        CSSPageRule {
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
impl core::ops::Deref for CSSPageRule {
    type Target = CSSGroupingRule;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for CSSPageRule {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for CSSPageRule {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for CSSPageRule {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<CSSPageRule> for Any {
    fn from(s: CSSPageRule) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&CSSPageRule> for Any {
    fn from(s: &CSSPageRule) -> Any {
        s.inner.clone().into()
    }
}
jsbind::utils::impl_dyn_cast!(CSSPageRule);

impl CSSPageRule {
    /// Getter of the `selectorText` attribute.
    /// [`CSSPageRule.selectorText`](https://developer.mozilla.org/en-US/docs/Web/API/CSSPageRule/selectorText)
    pub fn selector_text(&self) -> CSSOMString {
        self.inner.get("selectorText").as_::<CSSOMString>()
    }

    /// Setter of the `selectorText` attribute.
    /// [`CSSPageRule.selectorText`](https://developer.mozilla.org/en-US/docs/Web/API/CSSPageRule/selectorText)
    pub fn set_selector_text(&mut self, value: &CSSOMString) {
        self.inner.set("selectorText", value);
    }
}
impl CSSPageRule {
    /// Getter of the `style` attribute.
    /// [`CSSPageRule.style`](https://developer.mozilla.org/en-US/docs/Web/API/CSSPageRule/style)
    pub fn style(&self) -> CSSPageDescriptors {
        self.inner.get("style").as_::<CSSPageDescriptors>()
    }
}
