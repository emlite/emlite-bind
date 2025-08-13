use super::*;




/// The CSSKeywordValue class.
/// [`CSSKeywordValue`](https://developer.mozilla.org/en-US/docs/Web/API/CSSKeywordValue)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct CSSKeywordValue {
    inner: CSSStyleValue,
}

impl FromVal for CSSKeywordValue {
    fn from_val(v: &Any) -> Self {
        CSSKeywordValue { inner: CSSStyleValue::from_val(v) }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for CSSKeywordValue {
    type Target = CSSStyleValue;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for CSSKeywordValue {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for CSSKeywordValue {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for CSSKeywordValue {
    fn as_mut(&mut self) -> &mut Any {
      &mut self.inner
  }
}

impl From<CSSKeywordValue> for Any {
    fn from(s: CSSKeywordValue) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&CSSKeywordValue> for Any {
    fn from(s: &CSSKeywordValue) -> Any {
        s.inner.clone().into()
    }
}

jsbind::utils::impl_dyn_cast!(CSSKeywordValue);



impl CSSKeywordValue {
    /// The `new CSSKeywordValue(..)` constructor, creating a new CSSKeywordValue instance
    pub fn new(value: &JsString) -> CSSKeywordValue {
        Self {
            inner: Any::global("CSSKeywordValue").new(&[value.into()]).as_::<CSSStyleValue>(),
        }
    }

}
impl CSSKeywordValue {
    /// Getter of the `value` attribute.
    /// [`CSSKeywordValue.value`](https://developer.mozilla.org/en-US/docs/Web/API/CSSKeywordValue/value)
    pub fn value(&self) -> JsString {
        self.inner.get("value").as_::<JsString>()
    }

    /// Setter of the `value` attribute.
    /// [`CSSKeywordValue.value`](https://developer.mozilla.org/en-US/docs/Web/API/CSSKeywordValue/value)
    pub fn set_value(&mut self, value: &JsString) {
        self.inner.set("value", value);
    }
}
