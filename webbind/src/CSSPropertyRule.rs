use super::*;

/// The CSSPropertyRule class.
/// [`CSSPropertyRule`](https://developer.mozilla.org/en-US/docs/Web/API/CSSPropertyRule)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct CSSPropertyRule {
    inner: CSSRule,
}
impl FromVal for CSSPropertyRule {
    fn from_val(v: &Any) -> Self {
        CSSPropertyRule {
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
impl core::ops::Deref for CSSPropertyRule {
    type Target = CSSRule;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for CSSPropertyRule {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for CSSPropertyRule {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for CSSPropertyRule {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<CSSPropertyRule> for Any {
    fn from(s: CSSPropertyRule) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&CSSPropertyRule> for Any {
    fn from(s: &CSSPropertyRule) -> Any {
        s.inner.clone().into()
    }
}
jsbind::utils::impl_dyn_cast!(CSSPropertyRule);

impl CSSPropertyRule {
    /// Getter of the `name` attribute.
    /// [`CSSPropertyRule.name`](https://developer.mozilla.org/en-US/docs/Web/API/CSSPropertyRule/name)
    pub fn name(&self) -> JsString {
        self.inner.get("name").as_::<JsString>()
    }
}
impl CSSPropertyRule {
    /// Getter of the `syntax` attribute.
    /// [`CSSPropertyRule.syntax`](https://developer.mozilla.org/en-US/docs/Web/API/CSSPropertyRule/syntax)
    pub fn syntax(&self) -> JsString {
        self.inner.get("syntax").as_::<JsString>()
    }
}
impl CSSPropertyRule {
    /// Getter of the `inherits` attribute.
    /// [`CSSPropertyRule.inherits`](https://developer.mozilla.org/en-US/docs/Web/API/CSSPropertyRule/inherits)
    pub fn inherits(&self) -> bool {
        self.inner.get("inherits").as_::<bool>()
    }
}
impl CSSPropertyRule {
    /// Getter of the `initialValue` attribute.
    /// [`CSSPropertyRule.initialValue`](https://developer.mozilla.org/en-US/docs/Web/API/CSSPropertyRule/initialValue)
    pub fn initial_value(&self) -> JsString {
        self.inner.get("initialValue").as_::<JsString>()
    }
}
