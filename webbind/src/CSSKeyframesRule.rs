use super::*;

/// The CSSKeyframesRule class.
/// [`CSSKeyframesRule`](https://developer.mozilla.org/en-US/docs/Web/API/CSSKeyframesRule)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct CSSKeyframesRule {
    inner: CSSRule,
}
impl FromVal for CSSKeyframesRule {
    fn from_val(v: &Any) -> Self {
        CSSKeyframesRule {
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
impl core::ops::Deref for CSSKeyframesRule {
    type Target = CSSRule;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for CSSKeyframesRule {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for CSSKeyframesRule {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for CSSKeyframesRule {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<CSSKeyframesRule> for Any {
    fn from(s: CSSKeyframesRule) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&CSSKeyframesRule> for Any {
    fn from(s: &CSSKeyframesRule) -> Any {
        s.inner.clone().into()
    }
}
jsbind::utils::impl_dyn_cast!(CSSKeyframesRule);

impl CSSKeyframesRule {
    /// Getter of the `name` attribute.
    /// [`CSSKeyframesRule.name`](https://developer.mozilla.org/en-US/docs/Web/API/CSSKeyframesRule/name)
    pub fn name(&self) -> JsString {
        self.inner.get("name").as_::<JsString>()
    }

    /// Setter of the `name` attribute.
    /// [`CSSKeyframesRule.name`](https://developer.mozilla.org/en-US/docs/Web/API/CSSKeyframesRule/name)
    pub fn set_name(&mut self, value: &JsString) {
        self.inner.set("name", value);
    }
}
impl CSSKeyframesRule {
    /// Getter of the `cssRules` attribute.
    /// [`CSSKeyframesRule.cssRules`](https://developer.mozilla.org/en-US/docs/Web/API/CSSKeyframesRule/cssRules)
    pub fn css_rules(&self) -> CSSRuleList {
        self.inner.get("cssRules").as_::<CSSRuleList>()
    }
}
impl CSSKeyframesRule {
    /// Getter of the `length` attribute.
    /// [`CSSKeyframesRule.length`](https://developer.mozilla.org/en-US/docs/Web/API/CSSKeyframesRule/length)
    pub fn length(&self) -> u32 {
        self.inner.get("length").as_::<u32>()
    }
}
impl CSSKeyframesRule {
    /// The appendRule method.
    /// [`CSSKeyframesRule.appendRule`](https://developer.mozilla.org/en-US/docs/Web/API/CSSKeyframesRule/appendRule)
    pub fn append_rule(&self, rule: &JsString) -> Undefined {
        self.inner
            .call("appendRule", &[rule.into()])
            .as_::<Undefined>()
    }
}
impl CSSKeyframesRule {
    /// The deleteRule method.
    /// [`CSSKeyframesRule.deleteRule`](https://developer.mozilla.org/en-US/docs/Web/API/CSSKeyframesRule/deleteRule)
    pub fn delete_rule(&self, select: &JsString) -> Undefined {
        self.inner
            .call("deleteRule", &[select.into()])
            .as_::<Undefined>()
    }
}
impl CSSKeyframesRule {
    /// The findRule method.
    /// [`CSSKeyframesRule.findRule`](https://developer.mozilla.org/en-US/docs/Web/API/CSSKeyframesRule/findRule)
    pub fn find_rule(&self, select: &JsString) -> CSSKeyframeRule {
        self.inner
            .call("findRule", &[select.into()])
            .as_::<CSSKeyframeRule>()
    }
}
