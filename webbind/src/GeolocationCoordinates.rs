use super::*;




/// The GeolocationCoordinates class.
/// [`GeolocationCoordinates`](https://developer.mozilla.org/en-US/docs/Web/API/GeolocationCoordinates)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct GeolocationCoordinates {
    inner: Any,
}

impl FromVal for GeolocationCoordinates {
    fn from_val(v: &Any) -> Self {
        GeolocationCoordinates { inner: Any::from_val(v) }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for GeolocationCoordinates {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for GeolocationCoordinates {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for GeolocationCoordinates {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for GeolocationCoordinates {
    fn as_mut(&mut self) -> &mut Any {
      &mut self.inner
  }
}

impl From<GeolocationCoordinates> for Any {
    fn from(s: GeolocationCoordinates) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&GeolocationCoordinates> for Any {
    fn from(s: &GeolocationCoordinates) -> Any {
        s.inner.clone().into()
    }
}

jsbind::utils::impl_dyn_cast!(GeolocationCoordinates);


impl GeolocationCoordinates {
    /// Getter of the `accuracy` attribute.
    /// [`GeolocationCoordinates.accuracy`](https://developer.mozilla.org/en-US/docs/Web/API/GeolocationCoordinates/accuracy)
    pub fn accuracy(&self) -> f64 {
        self.inner.get("accuracy").as_::<f64>()
    }

}
impl GeolocationCoordinates {
    /// Getter of the `latitude` attribute.
    /// [`GeolocationCoordinates.latitude`](https://developer.mozilla.org/en-US/docs/Web/API/GeolocationCoordinates/latitude)
    pub fn latitude(&self) -> f64 {
        self.inner.get("latitude").as_::<f64>()
    }

}
impl GeolocationCoordinates {
    /// Getter of the `longitude` attribute.
    /// [`GeolocationCoordinates.longitude`](https://developer.mozilla.org/en-US/docs/Web/API/GeolocationCoordinates/longitude)
    pub fn longitude(&self) -> f64 {
        self.inner.get("longitude").as_::<f64>()
    }

}
impl GeolocationCoordinates {
    /// Getter of the `altitude` attribute.
    /// [`GeolocationCoordinates.altitude`](https://developer.mozilla.org/en-US/docs/Web/API/GeolocationCoordinates/altitude)
    pub fn altitude(&self) -> f64 {
        self.inner.get("altitude").as_::<f64>()
    }

}
impl GeolocationCoordinates {
    /// Getter of the `altitudeAccuracy` attribute.
    /// [`GeolocationCoordinates.altitudeAccuracy`](https://developer.mozilla.org/en-US/docs/Web/API/GeolocationCoordinates/altitudeAccuracy)
    pub fn altitude_accuracy(&self) -> f64 {
        self.inner.get("altitudeAccuracy").as_::<f64>()
    }

}
impl GeolocationCoordinates {
    /// Getter of the `heading` attribute.
    /// [`GeolocationCoordinates.heading`](https://developer.mozilla.org/en-US/docs/Web/API/GeolocationCoordinates/heading)
    pub fn heading(&self) -> f64 {
        self.inner.get("heading").as_::<f64>()
    }

}
impl GeolocationCoordinates {
    /// Getter of the `speed` attribute.
    /// [`GeolocationCoordinates.speed`](https://developer.mozilla.org/en-US/docs/Web/API/GeolocationCoordinates/speed)
    pub fn speed(&self) -> f64 {
        self.inner.get("speed").as_::<f64>()
    }

}
impl GeolocationCoordinates {
    /// The toJSON method.
    /// [`GeolocationCoordinates.toJSON`](https://developer.mozilla.org/en-US/docs/Web/API/GeolocationCoordinates/toJSON)
    pub fn to_json(&self, ) -> Object {
        self.inner.call("toJSON", &[]).as_::<Object>()
    }
}
