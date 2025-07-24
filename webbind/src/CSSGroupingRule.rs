use super::*;

/// The CSSGroupingRule class.
/// [`CSSGroupingRule`](https://developer.mozilla.org/en-US/docs/Web/API/CSSGroupingRule)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct CSSGroupingRule {
    inner: CSSRule,
}
impl FromVal for CSSGroupingRule {
    fn from_val(v: &Any) -> Self {
        CSSGroupingRule {
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
impl core::ops::Deref for CSSGroupingRule {
    type Target = CSSRule;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for CSSGroupingRule {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for CSSGroupingRule {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for CSSGroupingRule {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<CSSGroupingRule> for Any {
    fn from(s: CSSGroupingRule) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&CSSGroupingRule> for Any {
    fn from(s: &CSSGroupingRule) -> Any {
        s.inner.clone().into()
    }
}
jsbind::utils::impl_dyn_cast!(CSSGroupingRule);

impl CSSGroupingRule {
    /// Getter of the `cssRules` attribute.
    /// [`CSSGroupingRule.cssRules`](https://developer.mozilla.org/en-US/docs/Web/API/CSSGroupingRule/cssRules)
    pub fn css_rules(&self) -> CSSRuleList {
        self.inner.get("cssRules").as_::<CSSRuleList>()
    }
}
impl CSSGroupingRule {
    /// The insertRule method.
    /// [`CSSGroupingRule.insertRule`](https://developer.mozilla.org/en-US/docs/Web/API/CSSGroupingRule/insertRule)
    pub fn insert_rule0(&self, rule: &CSSOMString) -> u32 {
        self.inner.call("insertRule", &[rule.into()]).as_::<u32>()
    }
    /// The insertRule method.
    /// [`CSSGroupingRule.insertRule`](https://developer.mozilla.org/en-US/docs/Web/API/CSSGroupingRule/insertRule)
    pub fn insert_rule1(&self, rule: &CSSOMString, index: u32) -> u32 {
        self.inner
            .call("insertRule", &[rule.into(), index.into()])
            .as_::<u32>()
    }
}
impl CSSGroupingRule {
    /// The deleteRule method.
    /// [`CSSGroupingRule.deleteRule`](https://developer.mozilla.org/en-US/docs/Web/API/CSSGroupingRule/deleteRule)
    pub fn delete_rule(&self, index: u32) -> Undefined {
        self.inner
            .call("deleteRule", &[index.into()])
            .as_::<Undefined>()
    }
}
