use super::*;

#[derive(Clone, Debug)]
pub struct GeolocationCoordinates {
    inner: emlite::Val,
}
impl FromVal for GeolocationCoordinates {
    fn from_val(v: &emlite::Val) -> Self {
        GeolocationCoordinates {
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
impl std::ops::Deref for GeolocationCoordinates {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for GeolocationCoordinates {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<GeolocationCoordinates> for emlite::Val {
    fn from(s: GeolocationCoordinates) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl GeolocationCoordinates {
    pub fn accuracy(&self) -> f64 {
        self.inner.get("accuracy").as_::<f64>()
    }
}
impl GeolocationCoordinates {
    pub fn latitude(&self) -> f64 {
        self.inner.get("latitude").as_::<f64>()
    }
}
impl GeolocationCoordinates {
    pub fn longitude(&self) -> f64 {
        self.inner.get("longitude").as_::<f64>()
    }
}
impl GeolocationCoordinates {
    pub fn altitude(&self) -> f64 {
        self.inner.get("altitude").as_::<f64>()
    }
}
impl GeolocationCoordinates {
    pub fn altitude_accuracy(&self) -> f64 {
        self.inner.get("altitudeAccuracy").as_::<f64>()
    }
}
impl GeolocationCoordinates {
    pub fn heading(&self) -> f64 {
        self.inner.get("heading").as_::<f64>()
    }
}
impl GeolocationCoordinates {
    pub fn speed(&self) -> f64 {
        self.inner.get("speed").as_::<f64>()
    }
}
impl GeolocationCoordinates {
    pub fn to_json(&self) -> jsbind::Object {
        self.inner.call("toJSON", &[]).as_::<jsbind::Object>()
    }
}
