use super::*;




/// The CSSStyleValue class.
/// [`CSSStyleValue`](https://developer.mozilla.org/en-US/docs/Web/API/CSSStyleValue)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct CSSStyleValue {
    inner: Any,
}

impl FromVal for CSSStyleValue {
    fn from_val(v: &Any) -> Self {
        CSSStyleValue { inner: Any::from_val(v) }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for CSSStyleValue {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for CSSStyleValue {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for CSSStyleValue {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for CSSStyleValue {
    fn as_mut(&mut self) -> &mut Any {
      &mut self.inner
  }
}

impl From<CSSStyleValue> for Any {
    fn from(s: CSSStyleValue) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&CSSStyleValue> for Any {
    fn from(s: &CSSStyleValue) -> Any {
        s.inner.clone().into()
    }
}

jsbind::utils::impl_dyn_cast!(CSSStyleValue);


impl CSSStyleValue {
    /// The parse method.
    /// [`CSSStyleValue.parse`](https://developer.mozilla.org/en-US/docs/Web/API/CSSStyleValue/parse)
    pub fn parse(property: &JsString, css_text: &JsString) -> CSSStyleValue {
        Any::global("CSSStyleValue").call("parse", &[property.into(), css_text.into(), ]).as_::<CSSStyleValue>()
    }
}
impl CSSStyleValue {
    /// The parseAll method.
    /// [`CSSStyleValue.parseAll`](https://developer.mozilla.org/en-US/docs/Web/API/CSSStyleValue/parseAll)
    pub fn parse_all(property: &JsString, css_text: &JsString) -> TypedArray<CSSStyleValue> {
        Any::global("CSSStyleValue").call("parseAll", &[property.into(), css_text.into(), ]).as_::<TypedArray<CSSStyleValue>>()
    }
}
