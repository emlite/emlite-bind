use super::*;

/// The SVGLength class.
/// [`SVGLength`](https://developer.mozilla.org/en-US/docs/Web/API/SVGLength)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct SVGLength {
    inner: Any,
}
impl FromVal for SVGLength {
    fn from_val(v: &Any) -> Self {
        SVGLength {
            inner: Any::from_val(v),
        }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for SVGLength {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for SVGLength {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for SVGLength {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for SVGLength {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<SVGLength> for Any {
    fn from(s: SVGLength) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&SVGLength> for Any {
    fn from(s: &SVGLength) -> Any {
        s.inner.clone().into()
    }
}
jsbind::utils::impl_dyn_cast!(SVGLength);

impl SVGLength {
    /// Getter of the `unitType` attribute.
    /// [`SVGLength.unitType`](https://developer.mozilla.org/en-US/docs/Web/API/SVGLength/unitType)
    pub fn unit_type(&self) -> u16 {
        self.inner.get("unitType").as_::<u16>()
    }
}
impl SVGLength {
    /// Getter of the `value` attribute.
    /// [`SVGLength.value`](https://developer.mozilla.org/en-US/docs/Web/API/SVGLength/value)
    pub fn value(&self) -> f32 {
        self.inner.get("value").as_::<f32>()
    }

    /// Setter of the `value` attribute.
    /// [`SVGLength.value`](https://developer.mozilla.org/en-US/docs/Web/API/SVGLength/value)
    pub fn set_value(&mut self, value: f32) {
        self.inner.set("value", value);
    }
}
impl SVGLength {
    /// Getter of the `valueInSpecifiedUnits` attribute.
    /// [`SVGLength.valueInSpecifiedUnits`](https://developer.mozilla.org/en-US/docs/Web/API/SVGLength/valueInSpecifiedUnits)
    pub fn value_in_specified_units(&self) -> f32 {
        self.inner.get("valueInSpecifiedUnits").as_::<f32>()
    }

    /// Setter of the `valueInSpecifiedUnits` attribute.
    /// [`SVGLength.valueInSpecifiedUnits`](https://developer.mozilla.org/en-US/docs/Web/API/SVGLength/valueInSpecifiedUnits)
    pub fn set_value_in_specified_units(&mut self, value: f32) {
        self.inner.set("valueInSpecifiedUnits", value);
    }
}
impl SVGLength {
    /// Getter of the `valueAsString` attribute.
    /// [`SVGLength.valueAsString`](https://developer.mozilla.org/en-US/docs/Web/API/SVGLength/valueAsString)
    pub fn value_as_string(&self) -> String {
        self.inner.get("valueAsString").as_::<String>()
    }

    /// Setter of the `valueAsString` attribute.
    /// [`SVGLength.valueAsString`](https://developer.mozilla.org/en-US/docs/Web/API/SVGLength/valueAsString)
    pub fn set_value_as_string(&mut self, value: &str) {
        self.inner.set("valueAsString", value);
    }
}
impl SVGLength {
    /// The newValueSpecifiedUnits method.
    /// [`SVGLength.newValueSpecifiedUnits`](https://developer.mozilla.org/en-US/docs/Web/API/SVGLength/newValueSpecifiedUnits)
    pub fn new_value_specified_units(
        &self,
        unit_type: u16,
        value_in_specified_units: f32,
    ) -> Undefined {
        self.inner
            .call(
                "newValueSpecifiedUnits",
                &[unit_type.into(), value_in_specified_units.into()],
            )
            .as_::<Undefined>()
    }
}
impl SVGLength {
    /// The convertToSpecifiedUnits method.
    /// [`SVGLength.convertToSpecifiedUnits`](https://developer.mozilla.org/en-US/docs/Web/API/SVGLength/convertToSpecifiedUnits)
    pub fn convert_to_specified_units(&self, unit_type: u16) -> Undefined {
        self.inner
            .call("convertToSpecifiedUnits", &[unit_type.into()])
            .as_::<Undefined>()
    }
}
