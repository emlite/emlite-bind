use super::*;

/// The CSSConditionRule class.
/// [`CSSConditionRule`](https://developer.mozilla.org/en-US/docs/Web/API/CSSConditionRule)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct CSSConditionRule {
    inner: CSSGroupingRule,
}

impl FromVal for CSSConditionRule {
    fn from_val(v: &Any) -> Self {
        CSSConditionRule {
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

impl core::ops::Deref for CSSConditionRule {
    type Target = CSSGroupingRule;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for CSSConditionRule {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for CSSConditionRule {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for CSSConditionRule {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}

impl From<CSSConditionRule> for Any {
    fn from(s: CSSConditionRule) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&CSSConditionRule> for Any {
    fn from(s: &CSSConditionRule) -> Any {
        s.inner.clone().into()
    }
}

jsbind::utils::impl_dyn_cast!(CSSConditionRule);

impl CSSConditionRule {
    /// Getter of the `conditionText` attribute.
    /// [`CSSConditionRule.conditionText`](https://developer.mozilla.org/en-US/docs/Web/API/CSSConditionRule/conditionText)
    pub fn condition_text(&self) -> JsString {
        self.inner.get("conditionText").as_::<JsString>()
    }
}
