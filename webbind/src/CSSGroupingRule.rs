use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct CSSGroupingRule {
    inner: CSSRule,
}
impl FromVal for CSSGroupingRule {
    fn from_val(v: &emlite::Val) -> Self {
        CSSGroupingRule {
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
impl AsRef<emlite::Val> for CSSGroupingRule {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for CSSGroupingRule {
    fn as_mut(&mut self) -> &mut emlite::Val {
        &mut self.inner
    }
}
impl From<CSSGroupingRule> for emlite::Val {
    fn from(s: CSSGroupingRule) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}
impl From<&CSSGroupingRule> for emlite::Val {
    fn from(s: &CSSGroupingRule) -> emlite::Val {
        s.inner.clone().into()
    }
}
jsbind::utils::impl_dyn_cast!(CSSGroupingRule);

impl CSSGroupingRule {
    pub fn css_rules(&self) -> CSSRuleList {
        self.inner.get("cssRules").as_::<CSSRuleList>()
    }
}
impl CSSGroupingRule {
    pub fn insert_rule0(&self, rule: CSSOMString) -> u32 {
        self.inner.call("insertRule", &[rule.into()]).as_::<u32>()
    }

    pub fn insert_rule1(&self, rule: CSSOMString, index: u32) -> u32 {
        self.inner
            .call("insertRule", &[rule.into(), index.into()])
            .as_::<u32>()
    }
}
impl CSSGroupingRule {
    pub fn delete_rule(&self, index: u32) -> Undefined {
        self.inner
            .call("deleteRule", &[index.into()])
            .as_::<Undefined>()
    }
}
