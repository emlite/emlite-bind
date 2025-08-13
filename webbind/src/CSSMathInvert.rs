use super::*;




/// The CSSMathInvert class.
/// [`CSSMathInvert`](https://developer.mozilla.org/en-US/docs/Web/API/CSSMathInvert)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct CSSMathInvert {
    inner: CSSMathValue,
}

impl FromVal for CSSMathInvert {
    fn from_val(v: &Any) -> Self {
        CSSMathInvert { inner: CSSMathValue::from_val(v) }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for CSSMathInvert {
    type Target = CSSMathValue;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for CSSMathInvert {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for CSSMathInvert {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for CSSMathInvert {
    fn as_mut(&mut self) -> &mut Any {
      &mut self.inner
  }
}

impl From<CSSMathInvert> for Any {
    fn from(s: CSSMathInvert) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&CSSMathInvert> for Any {
    fn from(s: &CSSMathInvert) -> Any {
        s.inner.clone().into()
    }
}

jsbind::utils::impl_dyn_cast!(CSSMathInvert);



impl CSSMathInvert {
    /// The `new CSSMathInvert(..)` constructor, creating a new CSSMathInvert instance
    pub fn new(arg: &Any) -> CSSMathInvert {
        Self {
            inner: Any::global("CSSMathInvert").new(&[arg.into()]).as_::<CSSMathValue>(),
        }
    }

}
impl CSSMathInvert {
    /// Getter of the `value` attribute.
    /// [`CSSMathInvert.value`](https://developer.mozilla.org/en-US/docs/Web/API/CSSMathInvert/value)
    pub fn value(&self) -> CSSNumericValue {
        self.inner.get("value").as_::<CSSNumericValue>()
    }

}
