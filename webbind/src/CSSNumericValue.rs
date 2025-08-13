use super::*;




/// The CSSNumericValue class.
/// [`CSSNumericValue`](https://developer.mozilla.org/en-US/docs/Web/API/CSSNumericValue)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct CSSNumericValue {
    inner: CSSStyleValue,
}

impl FromVal for CSSNumericValue {
    fn from_val(v: &Any) -> Self {
        CSSNumericValue { inner: CSSStyleValue::from_val(v) }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for CSSNumericValue {
    type Target = CSSStyleValue;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for CSSNumericValue {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for CSSNumericValue {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for CSSNumericValue {
    fn as_mut(&mut self) -> &mut Any {
      &mut self.inner
  }
}

impl From<CSSNumericValue> for Any {
    fn from(s: CSSNumericValue) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&CSSNumericValue> for Any {
    fn from(s: &CSSNumericValue) -> Any {
        s.inner.clone().into()
    }
}

jsbind::utils::impl_dyn_cast!(CSSNumericValue);


impl CSSNumericValue {
    /// The add method.
    /// [`CSSNumericValue.add`](https://developer.mozilla.org/en-US/docs/Web/API/CSSNumericValue/add)
    pub fn add(&self, values: &Any) -> CSSNumericValue {
        self.inner.call("add", &[values.into(), ]).as_::<CSSNumericValue>()
    }
}
impl CSSNumericValue {
    /// The sub method.
    /// [`CSSNumericValue.sub`](https://developer.mozilla.org/en-US/docs/Web/API/CSSNumericValue/sub)
    pub fn sub(&self, values: &Any) -> CSSNumericValue {
        self.inner.call("sub", &[values.into(), ]).as_::<CSSNumericValue>()
    }
}
impl CSSNumericValue {
    /// The mul method.
    /// [`CSSNumericValue.mul`](https://developer.mozilla.org/en-US/docs/Web/API/CSSNumericValue/mul)
    pub fn mul(&self, values: &Any) -> CSSNumericValue {
        self.inner.call("mul", &[values.into(), ]).as_::<CSSNumericValue>()
    }
}
impl CSSNumericValue {
    /// The div method.
    /// [`CSSNumericValue.div`](https://developer.mozilla.org/en-US/docs/Web/API/CSSNumericValue/div)
    pub fn div(&self, values: &Any) -> CSSNumericValue {
        self.inner.call("div", &[values.into(), ]).as_::<CSSNumericValue>()
    }
}
impl CSSNumericValue {
    /// The min method.
    /// [`CSSNumericValue.min`](https://developer.mozilla.org/en-US/docs/Web/API/CSSNumericValue/min)
    pub fn min(&self, values: &Any) -> CSSNumericValue {
        self.inner.call("min", &[values.into(), ]).as_::<CSSNumericValue>()
    }
}
impl CSSNumericValue {
    /// The max method.
    /// [`CSSNumericValue.max`](https://developer.mozilla.org/en-US/docs/Web/API/CSSNumericValue/max)
    pub fn max(&self, values: &Any) -> CSSNumericValue {
        self.inner.call("max", &[values.into(), ]).as_::<CSSNumericValue>()
    }
}
impl CSSNumericValue {
    /// The equals method.
    /// [`CSSNumericValue.equals`](https://developer.mozilla.org/en-US/docs/Web/API/CSSNumericValue/equals)
    pub fn equals(&self, value: &Any) -> bool {
        self.inner.call("equals", &[value.into(), ]).as_::<bool>()
    }
}
impl CSSNumericValue {
    /// The to method.
    /// [`CSSNumericValue.to`](https://developer.mozilla.org/en-US/docs/Web/API/CSSNumericValue/to)
    pub fn to(&self, unit: &JsString) -> CSSUnitValue {
        self.inner.call("to", &[unit.into(), ]).as_::<CSSUnitValue>()
    }
}
impl CSSNumericValue {
    /// The toSum method.
    /// [`CSSNumericValue.toSum`](https://developer.mozilla.org/en-US/docs/Web/API/CSSNumericValue/toSum)
    pub fn to_sum(&self, units: &JsString) -> CSSMathSum {
        self.inner.call("toSum", &[units.into(), ]).as_::<CSSMathSum>()
    }
}
impl CSSNumericValue {
    /// The type method.
    /// [`CSSNumericValue.type`](https://developer.mozilla.org/en-US/docs/Web/API/CSSNumericValue/type)
    pub fn type_(&self, ) -> CSSNumericType {
        self.inner.call("type", &[]).as_::<CSSNumericType>()
    }
}
impl CSSNumericValue {
    /// The parse method.
    /// [`CSSNumericValue.parse`](https://developer.mozilla.org/en-US/docs/Web/API/CSSNumericValue/parse)
    pub fn parse(css_text: &JsString) -> CSSNumericValue {
        Any::global("CSSNumericValue").call("parse", &[css_text.into(), ]).as_::<CSSNumericValue>()
    }
}
