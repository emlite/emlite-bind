use super::*;




/// The CSSMathClamp class.
/// [`CSSMathClamp`](https://developer.mozilla.org/en-US/docs/Web/API/CSSMathClamp)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct CSSMathClamp {
    inner: CSSMathValue,
}

impl FromVal for CSSMathClamp {
    fn from_val(v: &Any) -> Self {
        CSSMathClamp { inner: CSSMathValue::from_val(v) }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for CSSMathClamp {
    type Target = CSSMathValue;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for CSSMathClamp {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for CSSMathClamp {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for CSSMathClamp {
    fn as_mut(&mut self) -> &mut Any {
      &mut self.inner
  }
}

impl From<CSSMathClamp> for Any {
    fn from(s: CSSMathClamp) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&CSSMathClamp> for Any {
    fn from(s: &CSSMathClamp) -> Any {
        s.inner.clone().into()
    }
}

jsbind::utils::impl_dyn_cast!(CSSMathClamp);



impl CSSMathClamp {
    /// The `new CSSMathClamp(..)` constructor, creating a new CSSMathClamp instance
    pub fn new(lower: &Any, value: &Any, upper: &Any) -> CSSMathClamp {
        Self {
            inner: Any::global("CSSMathClamp").new(&[lower.into(), value.into(), upper.into()]).as_::<CSSMathValue>(),
        }
    }

}
impl CSSMathClamp {
    /// Getter of the `lower` attribute.
    /// [`CSSMathClamp.lower`](https://developer.mozilla.org/en-US/docs/Web/API/CSSMathClamp/lower)
    pub fn lower(&self) -> CSSNumericValue {
        self.inner.get("lower").as_::<CSSNumericValue>()
    }

}
impl CSSMathClamp {
    /// Getter of the `value` attribute.
    /// [`CSSMathClamp.value`](https://developer.mozilla.org/en-US/docs/Web/API/CSSMathClamp/value)
    pub fn value(&self) -> CSSNumericValue {
        self.inner.get("value").as_::<CSSNumericValue>()
    }

}
impl CSSMathClamp {
    /// Getter of the `upper` attribute.
    /// [`CSSMathClamp.upper`](https://developer.mozilla.org/en-US/docs/Web/API/CSSMathClamp/upper)
    pub fn upper(&self) -> CSSNumericValue {
        self.inner.get("upper").as_::<CSSNumericValue>()
    }

}
