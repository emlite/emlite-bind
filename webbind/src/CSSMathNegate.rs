use super::*;




/// The CSSMathNegate class.
/// [`CSSMathNegate`](https://developer.mozilla.org/en-US/docs/Web/API/CSSMathNegate)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct CSSMathNegate {
    inner: CSSMathValue,
}

impl FromVal for CSSMathNegate {
    fn from_val(v: &Any) -> Self {
        CSSMathNegate { inner: CSSMathValue::from_val(v) }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for CSSMathNegate {
    type Target = CSSMathValue;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for CSSMathNegate {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for CSSMathNegate {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for CSSMathNegate {
    fn as_mut(&mut self) -> &mut Any {
      &mut self.inner
  }
}

impl From<CSSMathNegate> for Any {
    fn from(s: CSSMathNegate) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&CSSMathNegate> for Any {
    fn from(s: &CSSMathNegate) -> Any {
        s.inner.clone().into()
    }
}

jsbind::utils::impl_dyn_cast!(CSSMathNegate);



impl CSSMathNegate {
    /// The `new CSSMathNegate(..)` constructor, creating a new CSSMathNegate instance
    pub fn new(arg: &Any) -> CSSMathNegate {
        Self {
            inner: Any::global("CSSMathNegate").new(&[arg.into()]).as_::<CSSMathValue>(),
        }
    }

}
impl CSSMathNegate {
    /// Getter of the `value` attribute.
    /// [`CSSMathNegate.value`](https://developer.mozilla.org/en-US/docs/Web/API/CSSMathNegate/value)
    pub fn value(&self) -> CSSNumericValue {
        self.inner.get("value").as_::<CSSNumericValue>()
    }

}
