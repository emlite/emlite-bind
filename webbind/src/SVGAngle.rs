use super::*;

/// The SVGAngle class.
/// [`SVGAngle`](https://developer.mozilla.org/en-US/docs/Web/API/SVGAngle)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct SVGAngle {
    inner: Any,
}
impl FromVal for SVGAngle {
    fn from_val(v: &Any) -> Self {
        SVGAngle {
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
impl core::ops::Deref for SVGAngle {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for SVGAngle {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for SVGAngle {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for SVGAngle {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<SVGAngle> for Any {
    fn from(s: SVGAngle) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&SVGAngle> for Any {
    fn from(s: &SVGAngle) -> Any {
        s.inner.clone().into()
    }
}
jsbind::utils::impl_dyn_cast!(SVGAngle);

impl SVGAngle {
    /// Getter of the `unitType` attribute.
    /// [`SVGAngle.unitType`](https://developer.mozilla.org/en-US/docs/Web/API/SVGAngle/unitType)
    pub fn unit_type(&self) -> u16 {
        self.inner.get("unitType").as_::<u16>()
    }
}
impl SVGAngle {
    /// Getter of the `value` attribute.
    /// [`SVGAngle.value`](https://developer.mozilla.org/en-US/docs/Web/API/SVGAngle/value)
    pub fn value(&self) -> f32 {
        self.inner.get("value").as_::<f32>()
    }

    /// Setter of the `value` attribute.
    /// [`SVGAngle.value`](https://developer.mozilla.org/en-US/docs/Web/API/SVGAngle/value)
    pub fn set_value(&mut self, value: f32) {
        self.inner.set("value", value);
    }
}
impl SVGAngle {
    /// Getter of the `valueInSpecifiedUnits` attribute.
    /// [`SVGAngle.valueInSpecifiedUnits`](https://developer.mozilla.org/en-US/docs/Web/API/SVGAngle/valueInSpecifiedUnits)
    pub fn value_in_specified_units(&self) -> f32 {
        self.inner.get("valueInSpecifiedUnits").as_::<f32>()
    }

    /// Setter of the `valueInSpecifiedUnits` attribute.
    /// [`SVGAngle.valueInSpecifiedUnits`](https://developer.mozilla.org/en-US/docs/Web/API/SVGAngle/valueInSpecifiedUnits)
    pub fn set_value_in_specified_units(&mut self, value: f32) {
        self.inner.set("valueInSpecifiedUnits", value);
    }
}
impl SVGAngle {
    /// Getter of the `valueAsString` attribute.
    /// [`SVGAngle.valueAsString`](https://developer.mozilla.org/en-US/docs/Web/API/SVGAngle/valueAsString)
    pub fn value_as_string(&self) -> String {
        self.inner.get("valueAsString").as_::<String>()
    }

    /// Setter of the `valueAsString` attribute.
    /// [`SVGAngle.valueAsString`](https://developer.mozilla.org/en-US/docs/Web/API/SVGAngle/valueAsString)
    pub fn set_value_as_string(&mut self, value: &str) {
        self.inner.set("valueAsString", value);
    }
}
impl SVGAngle {
    /// The newValueSpecifiedUnits method.
    /// [`SVGAngle.newValueSpecifiedUnits`](https://developer.mozilla.org/en-US/docs/Web/API/SVGAngle/newValueSpecifiedUnits)
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
impl SVGAngle {
    /// The convertToSpecifiedUnits method.
    /// [`SVGAngle.convertToSpecifiedUnits`](https://developer.mozilla.org/en-US/docs/Web/API/SVGAngle/convertToSpecifiedUnits)
    pub fn convert_to_specified_units(&self, unit_type: u16) -> Undefined {
        self.inner
            .call("convertToSpecifiedUnits", &[unit_type.into()])
            .as_::<Undefined>()
    }
}
