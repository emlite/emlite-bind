use super::*;




/// The CSSMathValue class.
/// [`CSSMathValue`](https://developer.mozilla.org/en-US/docs/Web/API/CSSMathValue)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct CSSMathValue {
    inner: CSSNumericValue,
}

impl FromVal for CSSMathValue {
    fn from_val(v: &Any) -> Self {
        CSSMathValue { inner: CSSNumericValue::from_val(v) }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for CSSMathValue {
    type Target = CSSNumericValue;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for CSSMathValue {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for CSSMathValue {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for CSSMathValue {
    fn as_mut(&mut self) -> &mut Any {
      &mut self.inner
  }
}

impl From<CSSMathValue> for Any {
    fn from(s: CSSMathValue) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&CSSMathValue> for Any {
    fn from(s: &CSSMathValue) -> Any {
        s.inner.clone().into()
    }
}

jsbind::utils::impl_dyn_cast!(CSSMathValue);


impl CSSMathValue {
    /// Getter of the `operator` attribute.
    /// [`CSSMathValue.operator`](https://developer.mozilla.org/en-US/docs/Web/API/CSSMathValue/operator)
    pub fn operator(&self) -> CSSMathOperator {
        self.inner.get("operator").as_::<CSSMathOperator>()
    }

}
