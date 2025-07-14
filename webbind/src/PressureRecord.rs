use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct PressureRecord {
    inner: emlite::Val,
}
impl FromVal for PressureRecord {
    fn from_val(v: &emlite::Val) -> Self {
        PressureRecord {
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
impl core::ops::Deref for PressureRecord {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for PressureRecord {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<PressureRecord> for emlite::Val {
    fn from(s: PressureRecord) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl PressureRecord {
    pub fn source(&self) -> PressureSource {
        self.inner.get("source").as_::<PressureSource>()
    }
}
impl PressureRecord {
    pub fn state(&self) -> PressureState {
        self.inner.get("state").as_::<PressureState>()
    }
}
impl PressureRecord {
    pub fn time(&self) -> jsbind::Any {
        self.inner.get("time").as_::<jsbind::Any>()
    }
}
impl PressureRecord {
    pub fn to_json(&self) -> jsbind::Object {
        self.inner.call("toJSON", &[]).as_::<jsbind::Object>()
    }
}
