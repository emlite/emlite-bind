use super::*;

/// The CSSContainerRule class.
/// [`CSSContainerRule`](https://developer.mozilla.org/en-US/docs/Web/API/CSSContainerRule)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct CSSContainerRule {
    inner: CSSConditionRule,
}
impl FromVal for CSSContainerRule {
    fn from_val(v: &Any) -> Self {
        CSSContainerRule {
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
impl core::ops::Deref for CSSContainerRule {
    type Target = CSSConditionRule;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for CSSContainerRule {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for CSSContainerRule {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for CSSContainerRule {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<CSSContainerRule> for Any {
    fn from(s: CSSContainerRule) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&CSSContainerRule> for Any {
    fn from(s: &CSSContainerRule) -> Any {
        s.inner.clone().into()
    }
}
jsbind::utils::impl_dyn_cast!(CSSContainerRule);

impl CSSContainerRule {
    /// Getter of the `containerName` attribute.
    /// [`CSSContainerRule.containerName`](https://developer.mozilla.org/en-US/docs/Web/API/CSSContainerRule/containerName)
    pub fn container_name(&self) -> JsString {
        self.inner.get("containerName").as_::<JsString>()
    }
}
impl CSSContainerRule {
    /// Getter of the `containerQuery` attribute.
    /// [`CSSContainerRule.containerQuery`](https://developer.mozilla.org/en-US/docs/Web/API/CSSContainerRule/containerQuery)
    pub fn container_query(&self) -> JsString {
        self.inner.get("containerQuery").as_::<JsString>()
    }
}
