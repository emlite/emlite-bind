use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct CSSNumericType {
    inner: emlite::Val,
}
impl FromVal for CSSNumericType {
    fn from_val(v: &emlite::Val) -> Self {
        CSSNumericType { inner: v.clone() }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for CSSNumericType {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for CSSNumericType {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<CSSNumericType> for emlite::Val {
    fn from(s: CSSNumericType) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl CSSNumericType {
    pub fn length(&self) -> i32 {
        self.inner.get("length").as_::<i32>()
    }

    pub fn set_length(&mut self, value: i32) {
        self.inner.set("length", value);
    }
}
impl CSSNumericType {
    pub fn angle(&self) -> i32 {
        self.inner.get("angle").as_::<i32>()
    }

    pub fn set_angle(&mut self, value: i32) {
        self.inner.set("angle", value);
    }
}
impl CSSNumericType {
    pub fn time(&self) -> i32 {
        self.inner.get("time").as_::<i32>()
    }

    pub fn set_time(&mut self, value: i32) {
        self.inner.set("time", value);
    }
}
impl CSSNumericType {
    pub fn frequency(&self) -> i32 {
        self.inner.get("frequency").as_::<i32>()
    }

    pub fn set_frequency(&mut self, value: i32) {
        self.inner.set("frequency", value);
    }
}
impl CSSNumericType {
    pub fn resolution(&self) -> i32 {
        self.inner.get("resolution").as_::<i32>()
    }

    pub fn set_resolution(&mut self, value: i32) {
        self.inner.set("resolution", value);
    }
}
impl CSSNumericType {
    pub fn flex(&self) -> i32 {
        self.inner.get("flex").as_::<i32>()
    }

    pub fn set_flex(&mut self, value: i32) {
        self.inner.set("flex", value);
    }
}
impl CSSNumericType {
    pub fn percent(&self) -> i32 {
        self.inner.get("percent").as_::<i32>()
    }

    pub fn set_percent(&mut self, value: i32) {
        self.inner.set("percent", value);
    }
}
impl CSSNumericType {
    pub fn percent_hint(&self) -> CSSNumericBaseType {
        self.inner.get("percentHint").as_::<CSSNumericBaseType>()
    }

    pub fn set_percent_hint(&mut self, value: CSSNumericBaseType) {
        self.inner.set("percentHint", value);
    }
}
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct CSSNumericValue {
    inner: CSSStyleValue,
}
impl FromVal for CSSNumericValue {
    fn from_val(v: &emlite::Val) -> Self {
        CSSNumericValue {
            inner: CSSStyleValue::from_val(v),
        }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
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
impl From<CSSNumericValue> for emlite::Val {
    fn from(s: CSSNumericValue) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl CSSNumericValue {
    pub fn add(&self, values: jsbind::Any) -> CSSNumericValue {
        self.inner
            .call("add", &[values.into()])
            .as_::<CSSNumericValue>()
    }
}
impl CSSNumericValue {
    pub fn sub(&self, values: jsbind::Any) -> CSSNumericValue {
        self.inner
            .call("sub", &[values.into()])
            .as_::<CSSNumericValue>()
    }
}
impl CSSNumericValue {
    pub fn mul(&self, values: jsbind::Any) -> CSSNumericValue {
        self.inner
            .call("mul", &[values.into()])
            .as_::<CSSNumericValue>()
    }
}
impl CSSNumericValue {
    pub fn div(&self, values: jsbind::Any) -> CSSNumericValue {
        self.inner
            .call("div", &[values.into()])
            .as_::<CSSNumericValue>()
    }
}
impl CSSNumericValue {
    pub fn min(&self, values: jsbind::Any) -> CSSNumericValue {
        self.inner
            .call("min", &[values.into()])
            .as_::<CSSNumericValue>()
    }
}
impl CSSNumericValue {
    pub fn max(&self, values: jsbind::Any) -> CSSNumericValue {
        self.inner
            .call("max", &[values.into()])
            .as_::<CSSNumericValue>()
    }
}
impl CSSNumericValue {
    pub fn equals(&self, value: jsbind::Any) -> bool {
        self.inner.call("equals", &[value.into()]).as_::<bool>()
    }
}
impl CSSNumericValue {
    pub fn to(&self, unit: jsbind::USVString) -> CSSUnitValue {
        self.inner.call("to", &[unit.into()]).as_::<CSSUnitValue>()
    }
}
impl CSSNumericValue {
    pub fn to_sum(&self, units: jsbind::USVString) -> CSSMathSum {
        self.inner
            .call("toSum", &[units.into()])
            .as_::<CSSMathSum>()
    }
}
impl CSSNumericValue {
    pub fn type_(&self) -> CSSNumericType {
        self.inner.call("type", &[]).as_::<CSSNumericType>()
    }
}
impl CSSNumericValue {
    pub fn parse(css_text: jsbind::USVString) -> CSSNumericValue {
        emlite::Val::global("cssnumericvalue")
            .call("parse", &[css_text.into()])
            .as_::<CSSNumericValue>()
    }
}
