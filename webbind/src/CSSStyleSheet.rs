use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct CSSStyleSheet {
    inner: StyleSheet,
}
impl FromVal for CSSStyleSheet {
    fn from_val(v: &emlite::Val) -> Self {
        CSSStyleSheet {
            inner: StyleSheet::from_val(v),
        }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for CSSStyleSheet {
    type Target = StyleSheet;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for CSSStyleSheet {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for CSSStyleSheet {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for CSSStyleSheet {
    fn as_mut(&mut self) -> &mut emlite::Val {
        &mut self.inner
    }
}
impl From<CSSStyleSheet> for emlite::Val {
    fn from(s: CSSStyleSheet) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}
impl From<&CSSStyleSheet> for emlite::Val {
    fn from(s: &CSSStyleSheet) -> emlite::Val {
        s.inner.clone().into()
    }
}
jsbind::utils::impl_dyn_cast!(CSSStyleSheet);

impl CSSStyleSheet {
    pub fn new0() -> CSSStyleSheet {
        Self {
            inner: emlite::Val::global("CSSStyleSheet")
                .new(&[])
                .as_::<StyleSheet>(),
        }
    }

    pub fn new1(options: Any) -> CSSStyleSheet {
        Self {
            inner: emlite::Val::global("CSSStyleSheet")
                .new(&[options.into()])
                .as_::<StyleSheet>(),
        }
    }
}
impl CSSStyleSheet {
    pub fn owner_rule(&self) -> CSSRule {
        self.inner.get("ownerRule").as_::<CSSRule>()
    }
}
impl CSSStyleSheet {
    pub fn css_rules(&self) -> CSSRuleList {
        self.inner.get("cssRules").as_::<CSSRuleList>()
    }
}
impl CSSStyleSheet {
    pub fn insert_rule0(&self, rule: CSSOMString) -> u32 {
        self.inner.call("insertRule", &[rule.into()]).as_::<u32>()
    }

    pub fn insert_rule1(&self, rule: CSSOMString, index: u32) -> u32 {
        self.inner
            .call("insertRule", &[rule.into(), index.into()])
            .as_::<u32>()
    }
}
impl CSSStyleSheet {
    pub fn delete_rule(&self, index: u32) -> Undefined {
        self.inner
            .call("deleteRule", &[index.into()])
            .as_::<Undefined>()
    }
}
impl CSSStyleSheet {
    pub fn replace(&self, text: USVString) -> Promise {
        self.inner.call("replace", &[text.into()]).as_::<Promise>()
    }
}
impl CSSStyleSheet {
    pub fn replace_sync(&self, text: USVString) -> Undefined {
        self.inner
            .call("replaceSync", &[text.into()])
            .as_::<Undefined>()
    }
}
impl CSSStyleSheet {
    pub fn rules(&self) -> CSSRuleList {
        self.inner.get("rules").as_::<CSSRuleList>()
    }
}
impl CSSStyleSheet {
    pub fn add_rule0(&self) -> i32 {
        self.inner.call("addRule", &[]).as_::<i32>()
    }

    pub fn add_rule1(&self, selector: DOMString) -> i32 {
        self.inner.call("addRule", &[selector.into()]).as_::<i32>()
    }

    pub fn add_rule2(&self, selector: DOMString, style: DOMString) -> i32 {
        self.inner
            .call("addRule", &[selector.into(), style.into()])
            .as_::<i32>()
    }

    pub fn add_rule3(&self, selector: DOMString, style: DOMString, index: u32) -> i32 {
        self.inner
            .call("addRule", &[selector.into(), style.into(), index.into()])
            .as_::<i32>()
    }
}
impl CSSStyleSheet {
    pub fn remove_rule0(&self) -> Undefined {
        self.inner.call("removeRule", &[]).as_::<Undefined>()
    }

    pub fn remove_rule1(&self, index: u32) -> Undefined {
        self.inner
            .call("removeRule", &[index.into()])
            .as_::<Undefined>()
    }
}
