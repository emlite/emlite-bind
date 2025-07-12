use super::*;

#[derive(Clone, Debug)]
pub struct GeolocationPosition {
    inner: emlite::Val,
}
impl FromVal for GeolocationPosition {
    fn from_val(v: &emlite::Val) -> Self {
        GeolocationPosition {
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
impl std::ops::Deref for GeolocationPosition {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for GeolocationPosition {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<GeolocationPosition> for emlite::Val {
    fn from(s: GeolocationPosition) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl GeolocationPosition {
    pub fn coords(&self) -> GeolocationCoordinates {
        self.inner.get("coords").as_::<GeolocationCoordinates>()
    }
}
impl GeolocationPosition {
    pub fn timestamp(&self) -> jsbind::Any {
        self.inner.get("timestamp").as_::<jsbind::Any>()
    }
}
impl GeolocationPosition {
    pub fn to_json(&self) -> jsbind::Object {
        self.inner.call("toJSON", &[]).as_::<jsbind::Object>()
    }
}
