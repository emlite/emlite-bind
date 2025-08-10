use super::*;

/// The CSSColorValue class.
/// [`CSSColorValue`](https://developer.mozilla.org/en-US/docs/Web/API/CSSColorValue)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct CSSColorValue {
    inner: CSSStyleValue,
}

impl FromVal for CSSColorValue {
    fn from_val(v: &Any) -> Self {
        CSSColorValue {
            inner: CSSStyleValue::from_val(v),
        }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for CSSColorValue {
    type Target = CSSStyleValue;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for CSSColorValue {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for CSSColorValue {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for CSSColorValue {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}

impl From<CSSColorValue> for Any {
    fn from(s: CSSColorValue) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&CSSColorValue> for Any {
    fn from(s: &CSSColorValue) -> Any {
        s.inner.clone().into()
    }
}

jsbind::utils::impl_dyn_cast!(CSSColorValue);

impl CSSColorValue {
    /// The parse method.
    /// [`CSSColorValue.parse`](https://developer.mozilla.org/en-US/docs/Web/API/CSSColorValue/parse)
    pub fn parse(css_text: &JsString) -> Any {
        Any::global("CSSColorValue")
            .call("parse", &[css_text.into()])
            .as_::<Any>()
    }
}
