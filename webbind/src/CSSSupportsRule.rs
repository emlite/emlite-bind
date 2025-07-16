use super::*;

/// The CSSSupportsRule class.
/// [`CSSSupportsRule`](https://developer.mozilla.org/en-US/docs/Web/API/CSSSupportsRule)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct CSSSupportsRule {
    inner: CSSConditionRule,
}
impl FromVal for CSSSupportsRule {
    fn from_val(v: &Any) -> Self {
        CSSSupportsRule {
            inner: CSSConditionRule::from_val(v),
        }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for CSSSupportsRule {
    type Target = CSSConditionRule;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for CSSSupportsRule {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for CSSSupportsRule {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for CSSSupportsRule {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<CSSSupportsRule> for Any {
    fn from(s: CSSSupportsRule) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&CSSSupportsRule> for Any {
    fn from(s: &CSSSupportsRule) -> Any {
        s.inner.clone().into()
    }
}
jsbind::utils::impl_dyn_cast!(CSSSupportsRule);

impl CSSSupportsRule {
    /// Getter of the `matches` attribute.
    /// [`CSSSupportsRule.matches`](https://developer.mozilla.org/en-US/docs/Web/API/CSSSupportsRule/matches)
    pub fn matches(&self) -> bool {
        self.inner.get("matches").as_::<bool>()
    }
}
