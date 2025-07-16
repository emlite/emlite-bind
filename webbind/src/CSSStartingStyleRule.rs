use super::*;

/// The CSSStartingStyleRule class.
/// [`CSSStartingStyleRule`](https://developer.mozilla.org/en-US/docs/Web/API/CSSStartingStyleRule)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct CSSStartingStyleRule {
    inner: CSSGroupingRule,
}
impl FromVal for CSSStartingStyleRule {
    fn from_val(v: &Any) -> Self {
        CSSStartingStyleRule {
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
impl core::ops::Deref for CSSStartingStyleRule {
    type Target = CSSGroupingRule;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for CSSStartingStyleRule {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for CSSStartingStyleRule {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for CSSStartingStyleRule {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<CSSStartingStyleRule> for Any {
    fn from(s: CSSStartingStyleRule) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&CSSStartingStyleRule> for Any {
    fn from(s: &CSSStartingStyleRule) -> Any {
        s.inner.clone().into()
    }
}
jsbind::utils::impl_dyn_cast!(CSSStartingStyleRule);
