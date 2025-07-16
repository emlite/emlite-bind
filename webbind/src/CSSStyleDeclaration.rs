use super::*;

/// The CSSStyleDeclaration class.
/// [`CSSStyleDeclaration`](https://developer.mozilla.org/en-US/docs/Web/API/CSSStyleDeclaration)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct CSSStyleDeclaration {
    inner: Any,
}
impl FromVal for CSSStyleDeclaration {
    fn from_val(v: &Any) -> Self {
        CSSStyleDeclaration {
            inner: Any::from_val(v),
        }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for CSSStyleDeclaration {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for CSSStyleDeclaration {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for CSSStyleDeclaration {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for CSSStyleDeclaration {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<CSSStyleDeclaration> for Any {
    fn from(s: CSSStyleDeclaration) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&CSSStyleDeclaration> for Any {
    fn from(s: &CSSStyleDeclaration) -> Any {
        s.inner.clone().into()
    }
}
jsbind::utils::impl_dyn_cast!(CSSStyleDeclaration);

impl CSSStyleDeclaration {
    /// Getter of the `cssText` attribute.
    /// [`CSSStyleDeclaration.cssText`](https://developer.mozilla.org/en-US/docs/Web/API/CSSStyleDeclaration/cssText)
    pub fn css_text(&self) -> String {
        self.inner.get("cssText").as_::<String>()
    }

    /// Setter of the `cssText` attribute.
    /// [`CSSStyleDeclaration.cssText`](https://developer.mozilla.org/en-US/docs/Web/API/CSSStyleDeclaration/cssText)
    pub fn set_css_text(&mut self, value: &str) {
        self.inner.set("cssText", value);
    }
}
impl CSSStyleDeclaration {
    /// Getter of the `length` attribute.
    /// [`CSSStyleDeclaration.length`](https://developer.mozilla.org/en-US/docs/Web/API/CSSStyleDeclaration/length)
    pub fn length(&self) -> u32 {
        self.inner.get("length").as_::<u32>()
    }
}
impl CSSStyleDeclaration {
    /// The item method.
    /// [`CSSStyleDeclaration.item`](https://developer.mozilla.org/en-US/docs/Web/API/CSSStyleDeclaration/item)
    pub fn item(&self, index: u32) -> String {
        self.inner.call("item", &[index.into()]).as_::<String>()
    }
}
impl CSSStyleDeclaration {
    /// The getPropertyValue method.
    /// [`CSSStyleDeclaration.getPropertyValue`](https://developer.mozilla.org/en-US/docs/Web/API/CSSStyleDeclaration/getPropertyValue)
    pub fn get_property_value(&self, property: &str) -> String {
        self.inner
            .call("getPropertyValue", &[property.into()])
            .as_::<String>()
    }
}
impl CSSStyleDeclaration {
    /// The getPropertyPriority method.
    /// [`CSSStyleDeclaration.getPropertyPriority`](https://developer.mozilla.org/en-US/docs/Web/API/CSSStyleDeclaration/getPropertyPriority)
    pub fn get_property_priority(&self, property: &str) -> String {
        self.inner
            .call("getPropertyPriority", &[property.into()])
            .as_::<String>()
    }
}
impl CSSStyleDeclaration {
    /// The setProperty method.
    /// [`CSSStyleDeclaration.setProperty`](https://developer.mozilla.org/en-US/docs/Web/API/CSSStyleDeclaration/setProperty)
    pub fn set_property0(&self, property: &str, value: &str) -> Undefined {
        self.inner
            .call("setProperty", &[property.into(), value.into()])
            .as_::<Undefined>()
    }
    /// The setProperty method.
    /// [`CSSStyleDeclaration.setProperty`](https://developer.mozilla.org/en-US/docs/Web/API/CSSStyleDeclaration/setProperty)
    pub fn set_property1(&self, property: &str, value: &str, priority: &str) -> Undefined {
        self.inner
            .call(
                "setProperty",
                &[property.into(), value.into(), priority.into()],
            )
            .as_::<Undefined>()
    }
}
impl CSSStyleDeclaration {
    /// The removeProperty method.
    /// [`CSSStyleDeclaration.removeProperty`](https://developer.mozilla.org/en-US/docs/Web/API/CSSStyleDeclaration/removeProperty)
    pub fn remove_property(&self, property: &str) -> String {
        self.inner
            .call("removeProperty", &[property.into()])
            .as_::<String>()
    }
}
impl CSSStyleDeclaration {
    /// Getter of the `parentRule` attribute.
    /// [`CSSStyleDeclaration.parentRule`](https://developer.mozilla.org/en-US/docs/Web/API/CSSStyleDeclaration/parentRule)
    pub fn parent_rule(&self) -> CSSRule {
        self.inner.get("parentRule").as_::<CSSRule>()
    }
}
