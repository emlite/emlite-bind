use super::*;

#[derive(Clone, Debug)]
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
impl std::ops::Deref for CSSStyleSheet {
    type Target = StyleSheet;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for CSSStyleSheet {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<CSSStyleSheet> for emlite::Val {
    fn from(s: CSSStyleSheet) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl CSSStyleSheet {
    pub fn new0() -> CSSStyleSheet {
        Self {
            inner: emlite::Val::global("CSSStyleSheet")
                .new(&[])
                .as_::<StyleSheet>(),
        }
    }

    pub fn new1(options: jsbind::Any) -> CSSStyleSheet {
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
    pub fn insert_rule0(&self, rule: jsbind::CSSOMString) -> u32 {
        self.inner.call("insertRule", &[rule.into()]).as_::<u32>()
    }

    pub fn insert_rule1(&self, rule: jsbind::CSSOMString, index: u32) -> u32 {
        self.inner
            .call("insertRule", &[rule.into(), index.into()])
            .as_::<u32>()
    }
}
impl CSSStyleSheet {
    pub fn delete_rule(&self, index: u32) -> jsbind::Undefined {
        self.inner
            .call("deleteRule", &[index.into()])
            .as_::<jsbind::Undefined>()
    }
}
impl CSSStyleSheet {
    pub fn replace(&self, text: jsbind::USVString) -> jsbind::Promise {
        self.inner
            .call("replace", &[text.into()])
            .as_::<jsbind::Promise>()
    }
}
impl CSSStyleSheet {
    pub fn replace_sync(&self, text: jsbind::USVString) -> jsbind::Undefined {
        self.inner
            .call("replaceSync", &[text.into()])
            .as_::<jsbind::Undefined>()
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

    pub fn add_rule1(&self, selector: jsbind::DOMString) -> i32 {
        self.inner.call("addRule", &[selector.into()]).as_::<i32>()
    }

    pub fn add_rule2(&self, selector: jsbind::DOMString, style: jsbind::DOMString) -> i32 {
        self.inner
            .call("addRule", &[selector.into(), style.into()])
            .as_::<i32>()
    }

    pub fn add_rule3(
        &self,
        selector: jsbind::DOMString,
        style: jsbind::DOMString,
        index: u32,
    ) -> i32 {
        self.inner
            .call("addRule", &[selector.into(), style.into(), index.into()])
            .as_::<i32>()
    }
}
impl CSSStyleSheet {
    pub fn remove_rule0(&self) -> jsbind::Undefined {
        self.inner
            .call("removeRule", &[])
            .as_::<jsbind::Undefined>()
    }

    pub fn remove_rule1(&self, index: u32) -> jsbind::Undefined {
        self.inner
            .call("removeRule", &[index.into()])
            .as_::<jsbind::Undefined>()
    }
}
