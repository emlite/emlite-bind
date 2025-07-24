use super::*;

/// The CSSUnitValue class.
/// [`CSSUnitValue`](https://developer.mozilla.org/en-US/docs/Web/API/CSSUnitValue)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct CSSUnitValue {
    inner: CSSNumericValue,
}
impl FromVal for CSSUnitValue {
    fn from_val(v: &Any) -> Self {
        CSSUnitValue {
            inner: CSSNumericValue::from_val(v),
        }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for CSSUnitValue {
    type Target = CSSNumericValue;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for CSSUnitValue {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for CSSUnitValue {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for CSSUnitValue {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<CSSUnitValue> for Any {
    fn from(s: CSSUnitValue) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&CSSUnitValue> for Any {
    fn from(s: &CSSUnitValue) -> Any {
        s.inner.clone().into()
    }
}
jsbind::utils::impl_dyn_cast!(CSSUnitValue);

impl CSSUnitValue {
    /// The `new CSSUnitValue(..)` constructor, creating a new CSSUnitValue instance
    pub fn new(value: f64, unit: &USVString) -> CSSUnitValue {
        Self {
            inner: Any::global("CSSUnitValue")
                .new(&[value.into(), unit.into()])
                .as_::<CSSNumericValue>(),
        }
    }
}
impl CSSUnitValue {
    /// Getter of the `value` attribute.
    /// [`CSSUnitValue.value`](https://developer.mozilla.org/en-US/docs/Web/API/CSSUnitValue/value)
    pub fn value(&self) -> f64 {
        self.inner.get("value").as_::<f64>()
    }

    /// Setter of the `value` attribute.
    /// [`CSSUnitValue.value`](https://developer.mozilla.org/en-US/docs/Web/API/CSSUnitValue/value)
    pub fn set_value(&mut self, value: f64) {
        self.inner.set("value", value);
    }
}
impl CSSUnitValue {
    /// Getter of the `unit` attribute.
    /// [`CSSUnitValue.unit`](https://developer.mozilla.org/en-US/docs/Web/API/CSSUnitValue/unit)
    pub fn unit(&self) -> USVString {
        self.inner.get("unit").as_::<USVString>()
    }
}
