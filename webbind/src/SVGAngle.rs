use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct SVGAngle {
    inner: emlite::Val,
}
impl FromVal for SVGAngle {
    fn from_val(v: &emlite::Val) -> Self {
        SVGAngle {
            inner: emlite::Val::from_val(v),
        }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for SVGAngle {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for SVGAngle {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for SVGAngle {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for SVGAngle {
    fn as_mut(&mut self) -> &mut emlite::Val {
        &mut self.inner
    }
}
impl From<SVGAngle> for emlite::Val {
    fn from(s: SVGAngle) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}
jsbind::utils::impl_dyn_cast!(SVGAngle);

impl SVGAngle {
    pub fn unit_type(&self) -> u16 {
        self.inner.get("unitType").as_::<u16>()
    }
}
impl SVGAngle {
    pub fn value(&self) -> f32 {
        self.inner.get("value").as_::<f32>()
    }

    pub fn set_value(&mut self, value: f32) {
        self.inner.set("value", value);
    }
}
impl SVGAngle {
    pub fn value_in_specified_units(&self) -> f32 {
        self.inner.get("valueInSpecifiedUnits").as_::<f32>()
    }

    pub fn set_value_in_specified_units(&mut self, value: f32) {
        self.inner.set("valueInSpecifiedUnits", value);
    }
}
impl SVGAngle {
    pub fn value_as_string(&self) -> jsbind::DOMString {
        self.inner.get("valueAsString").as_::<jsbind::DOMString>()
    }

    pub fn set_value_as_string(&mut self, value: jsbind::DOMString) {
        self.inner.set("valueAsString", value);
    }
}
impl SVGAngle {
    pub fn new_value_specified_units(
        &self,
        unit_type: u16,
        value_in_specified_units: f32,
    ) -> jsbind::Undefined {
        self.inner
            .call(
                "newValueSpecifiedUnits",
                &[unit_type.into(), value_in_specified_units.into()],
            )
            .as_::<jsbind::Undefined>()
    }
}
impl SVGAngle {
    pub fn convert_to_specified_units(&self, unit_type: u16) -> jsbind::Undefined {
        self.inner
            .call("convertToSpecifiedUnits", &[unit_type.into()])
            .as_::<jsbind::Undefined>()
    }
}
