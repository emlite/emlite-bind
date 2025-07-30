use super::*;

/// The CSSLayerBlockRule class.
/// [`CSSLayerBlockRule`](https://developer.mozilla.org/en-US/docs/Web/API/CSSLayerBlockRule)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct CSSLayerBlockRule {
    inner: CSSGroupingRule,
}
impl FromVal for CSSLayerBlockRule {
    fn from_val(v: &Any) -> Self {
        CSSLayerBlockRule {
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
impl core::ops::Deref for CSSLayerBlockRule {
    type Target = CSSGroupingRule;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for CSSLayerBlockRule {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for CSSLayerBlockRule {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for CSSLayerBlockRule {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<CSSLayerBlockRule> for Any {
    fn from(s: CSSLayerBlockRule) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&CSSLayerBlockRule> for Any {
    fn from(s: &CSSLayerBlockRule) -> Any {
        s.inner.clone().into()
    }
}
jsbind::utils::impl_dyn_cast!(CSSLayerBlockRule);

impl CSSLayerBlockRule {
    /// Getter of the `name` attribute.
    /// [`CSSLayerBlockRule.name`](https://developer.mozilla.org/en-US/docs/Web/API/CSSLayerBlockRule/name)
    pub fn name(&self) -> JsString {
        self.inner.get("name").as_::<JsString>()
    }
}
