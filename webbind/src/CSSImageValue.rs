use super::*;




/// The CSSImageValue class.
/// [`CSSImageValue`](https://developer.mozilla.org/en-US/docs/Web/API/CSSImageValue)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct CSSImageValue {
    inner: CSSStyleValue,
}

impl FromVal for CSSImageValue {
    fn from_val(v: &Any) -> Self {
        CSSImageValue { inner: CSSStyleValue::from_val(v) }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for CSSImageValue {
    type Target = CSSStyleValue;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for CSSImageValue {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for CSSImageValue {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for CSSImageValue {
    fn as_mut(&mut self) -> &mut Any {
      &mut self.inner
  }
}

impl From<CSSImageValue> for Any {
    fn from(s: CSSImageValue) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&CSSImageValue> for Any {
    fn from(s: &CSSImageValue) -> Any {
        s.inner.clone().into()
    }
}

jsbind::utils::impl_dyn_cast!(CSSImageValue);


