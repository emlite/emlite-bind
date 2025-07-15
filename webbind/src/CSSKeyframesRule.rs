use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct CSSKeyframesRule {
    inner: CSSRule,
}
impl FromVal for CSSKeyframesRule {
    fn from_val(v: &emlite::Val) -> Self {
        CSSKeyframesRule {
            inner: CSSRule::from_val(v),
        }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
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
impl AsRef<emlite::Val> for CSSKeyframesRule {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for CSSKeyframesRule {
    fn as_mut(&mut self) -> &mut emlite::Val {
        &mut self.inner
    }
}
impl From<CSSKeyframesRule> for emlite::Val {
    fn from(s: CSSKeyframesRule) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}
impl From<&CSSKeyframesRule> for emlite::Val {
    fn from(s: &CSSKeyframesRule) -> emlite::Val {
        s.inner.clone().into()
    }
}
jsbind::utils::impl_dyn_cast!(CSSKeyframesRule);

impl CSSKeyframesRule {
    pub fn name(&self) -> String {
        self.inner.get("name").as_::<String>()
    }

    pub fn set_name(&mut self, value: &str) {
        self.inner.set("name", value);
    }
}
impl CSSKeyframesRule {
    pub fn css_rules(&self) -> CSSRuleList {
        self.inner.get("cssRules").as_::<CSSRuleList>()
    }
}
impl CSSKeyframesRule {
    pub fn length(&self) -> u32 {
        self.inner.get("length").as_::<u32>()
    }
}
impl CSSKeyframesRule {
    pub fn append_rule(&self, rule: &str) -> Undefined {
        self.inner
            .call("appendRule", &[rule.into()])
            .as_::<Undefined>()
    }
}
impl CSSKeyframesRule {
    pub fn delete_rule(&self, select: &str) -> Undefined {
        self.inner
            .call("deleteRule", &[select.into()])
            .as_::<Undefined>()
    }
}
impl CSSKeyframesRule {
    pub fn find_rule(&self, select: &str) -> CSSKeyframeRule {
        self.inner
            .call("findRule", &[select.into()])
            .as_::<CSSKeyframeRule>()
    }
}
