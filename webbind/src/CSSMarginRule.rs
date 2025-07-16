use super::*;

/// The CSSMarginRule class.
/// [`CSSMarginRule`](https://developer.mozilla.org/en-US/docs/Web/API/CSSMarginRule)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct CSSMarginRule {
    inner: CSSRule,
}
impl FromVal for CSSMarginRule {
    fn from_val(v: &Any) -> Self {
        CSSMarginRule {
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
impl core::ops::Deref for CSSMarginRule {
    type Target = CSSRule;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for CSSMarginRule {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for CSSMarginRule {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for CSSMarginRule {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<CSSMarginRule> for Any {
    fn from(s: CSSMarginRule) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&CSSMarginRule> for Any {
    fn from(s: &CSSMarginRule) -> Any {
        s.inner.clone().into()
    }
}
jsbind::utils::impl_dyn_cast!(CSSMarginRule);

impl CSSMarginRule {
    /// Getter of the `name` attribute.
    /// [`CSSMarginRule.name`](https://developer.mozilla.org/en-US/docs/Web/API/CSSMarginRule/name)
    pub fn name(&self) -> String {
        self.inner.get("name").as_::<String>()
    }
}
impl CSSMarginRule {
    /// Getter of the `style` attribute.
    /// [`CSSMarginRule.style`](https://developer.mozilla.org/en-US/docs/Web/API/CSSMarginRule/style)
    pub fn style(&self) -> CSSStyleDeclaration {
        self.inner.get("style").as_::<CSSStyleDeclaration>()
    }
}
