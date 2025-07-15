use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct CSSUnitValue {
    inner: CSSNumericValue,
}
impl FromVal for CSSUnitValue {
    fn from_val(v: &emlite::Val) -> Self {
        CSSUnitValue {
            inner: CSSNumericValue::from_val(v),
        }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
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
impl AsRef<emlite::Val> for CSSUnitValue {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for CSSUnitValue {
    fn as_mut(&mut self) -> &mut emlite::Val {
        &mut self.inner
    }
}
impl From<CSSUnitValue> for emlite::Val {
    fn from(s: CSSUnitValue) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}
impl From<&CSSUnitValue> for emlite::Val {
    fn from(s: &CSSUnitValue) -> emlite::Val {
        s.inner.clone().into()
    }
}
jsbind::utils::impl_dyn_cast!(CSSUnitValue);

impl CSSUnitValue {
    pub fn new(value: f64, unit: USVString) -> CSSUnitValue {
        Self {
            inner: emlite::Val::global("CSSUnitValue")
                .new(&[value.into(), unit.into()])
                .as_::<CSSNumericValue>(),
        }
    }
}
impl CSSUnitValue {
    pub fn value(&self) -> f64 {
        self.inner.get("value").as_::<f64>()
    }

    pub fn set_value(&mut self, value: f64) {
        self.inner.set("value", value);
    }
}
impl CSSUnitValue {
    pub fn unit(&self) -> USVString {
        self.inner.get("unit").as_::<USVString>()
    }
}
