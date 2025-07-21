use super::*;

/// The CSSStyleSheet class.
/// [`CSSStyleSheet`](https://developer.mozilla.org/en-US/docs/Web/API/CSSStyleSheet)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct CSSStyleSheet {
    inner: StyleSheet,
}
impl FromVal for CSSStyleSheet {
    fn from_val(v: &Any) -> Self {
        CSSStyleSheet {
            inner: StyleSheet::from_val(v),
        }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
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
impl AsRef<Any> for CSSStyleSheet {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for CSSStyleSheet {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<CSSStyleSheet> for Any {
    fn from(s: CSSStyleSheet) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&CSSStyleSheet> for Any {
    fn from(s: &CSSStyleSheet) -> Any {
        s.inner.clone().into()
    }
}
jsbind::utils::impl_dyn_cast!(CSSStyleSheet);

impl CSSStyleSheet {
    /// The `new CSSStyleSheet(..)` constructor, creating a new CSSStyleSheet instance
    pub fn new0() -> CSSStyleSheet {
        Self {
            inner: Any::global("CSSStyleSheet").new(&[]).as_::<StyleSheet>(),
        }
    }

    /// The `new CSSStyleSheet(..)` constructor, creating a new CSSStyleSheet instance
    pub fn new1(options: &Any) -> CSSStyleSheet {
        Self {
            inner: Any::global("CSSStyleSheet")
                .new(&[options.into()])
                .as_::<StyleSheet>(),
        }
    }
}
impl CSSStyleSheet {
    /// Getter of the `ownerRule` attribute.
    /// [`CSSStyleSheet.ownerRule`](https://developer.mozilla.org/en-US/docs/Web/API/CSSStyleSheet/ownerRule)
    pub fn owner_rule(&self) -> CSSRule {
        self.inner.get("ownerRule").as_::<CSSRule>()
    }
}
impl CSSStyleSheet {
    /// Getter of the `cssRules` attribute.
    /// [`CSSStyleSheet.cssRules`](https://developer.mozilla.org/en-US/docs/Web/API/CSSStyleSheet/cssRules)
    pub fn css_rules(&self) -> CSSRuleList {
        self.inner.get("cssRules").as_::<CSSRuleList>()
    }
}
impl CSSStyleSheet {
    /// The insertRule method.
    /// [`CSSStyleSheet.insertRule`](https://developer.mozilla.org/en-US/docs/Web/API/CSSStyleSheet/insertRule)
    pub fn insert_rule0(&self, rule: &str) -> u32 {
        self.inner.call("insertRule", &[rule.into()]).as_::<u32>()
    }
    /// The insertRule method.
    /// [`CSSStyleSheet.insertRule`](https://developer.mozilla.org/en-US/docs/Web/API/CSSStyleSheet/insertRule)
    pub fn insert_rule1(&self, rule: &str, index: u32) -> u32 {
        self.inner
            .call("insertRule", &[rule.into(), index.into()])
            .as_::<u32>()
    }
}
impl CSSStyleSheet {
    /// The deleteRule method.
    /// [`CSSStyleSheet.deleteRule`](https://developer.mozilla.org/en-US/docs/Web/API/CSSStyleSheet/deleteRule)
    pub fn delete_rule(&self, index: u32) -> Undefined {
        self.inner
            .call("deleteRule", &[index.into()])
            .as_::<Undefined>()
    }
}
impl CSSStyleSheet {
    /// The replace method.
    /// [`CSSStyleSheet.replace`](https://developer.mozilla.org/en-US/docs/Web/API/CSSStyleSheet/replace)
    pub fn replace(&self, text: &str) -> Promise<CSSStyleSheet> {
        self.inner
            .call("replace", &[text.into()])
            .as_::<Promise<CSSStyleSheet>>()
    }
}
impl CSSStyleSheet {
    /// The replaceSync method.
    /// [`CSSStyleSheet.replaceSync`](https://developer.mozilla.org/en-US/docs/Web/API/CSSStyleSheet/replaceSync)
    pub fn replace_sync(&self, text: &str) -> Undefined {
        self.inner
            .call("replaceSync", &[text.into()])
            .as_::<Undefined>()
    }
}
impl CSSStyleSheet {
    /// Getter of the `rules` attribute.
    /// [`CSSStyleSheet.rules`](https://developer.mozilla.org/en-US/docs/Web/API/CSSStyleSheet/rules)
    pub fn rules(&self) -> CSSRuleList {
        self.inner.get("rules").as_::<CSSRuleList>()
    }
}
impl CSSStyleSheet {
    /// The addRule method.
    /// [`CSSStyleSheet.addRule`](https://developer.mozilla.org/en-US/docs/Web/API/CSSStyleSheet/addRule)
    pub fn add_rule0(&self) -> i32 {
        self.inner.call("addRule", &[]).as_::<i32>()
    }
    /// The addRule method.
    /// [`CSSStyleSheet.addRule`](https://developer.mozilla.org/en-US/docs/Web/API/CSSStyleSheet/addRule)
    pub fn add_rule1(&self, selector: &str) -> i32 {
        self.inner.call("addRule", &[selector.into()]).as_::<i32>()
    }
    /// The addRule method.
    /// [`CSSStyleSheet.addRule`](https://developer.mozilla.org/en-US/docs/Web/API/CSSStyleSheet/addRule)
    pub fn add_rule2(&self, selector: &str, style: &str) -> i32 {
        self.inner
            .call("addRule", &[selector.into(), style.into()])
            .as_::<i32>()
    }
    /// The addRule method.
    /// [`CSSStyleSheet.addRule`](https://developer.mozilla.org/en-US/docs/Web/API/CSSStyleSheet/addRule)
    pub fn add_rule3(&self, selector: &str, style: &str, index: u32) -> i32 {
        self.inner
            .call("addRule", &[selector.into(), style.into(), index.into()])
            .as_::<i32>()
    }
}
impl CSSStyleSheet {
    /// The removeRule method.
    /// [`CSSStyleSheet.removeRule`](https://developer.mozilla.org/en-US/docs/Web/API/CSSStyleSheet/removeRule)
    pub fn remove_rule0(&self) -> Undefined {
        self.inner.call("removeRule", &[]).as_::<Undefined>()
    }
    /// The removeRule method.
    /// [`CSSStyleSheet.removeRule`](https://developer.mozilla.org/en-US/docs/Web/API/CSSStyleSheet/removeRule)
    pub fn remove_rule1(&self, index: u32) -> Undefined {
        self.inner
            .call("removeRule", &[index.into()])
            .as_::<Undefined>()
    }
}
