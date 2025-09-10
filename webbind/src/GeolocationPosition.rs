use super::*;

/// The GeolocationPosition class.
/// [`GeolocationPosition`](https://developer.mozilla.org/en-US/docs/Web/API/GeolocationPosition)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct GeolocationPosition {
    inner: Any,
}

impl FromVal for GeolocationPosition {
    fn from_val(v: &Any) -> Self {
        GeolocationPosition {
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

impl core::ops::Deref for GeolocationPosition {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for GeolocationPosition {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for GeolocationPosition {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for GeolocationPosition {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}

impl From<GeolocationPosition> for Any {
    fn from(s: GeolocationPosition) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&GeolocationPosition> for Any {
    fn from(s: &GeolocationPosition) -> Any {
        s.inner.clone().into()
    }
}

jsbind::utils::impl_dyn_cast!(GeolocationPosition);

impl GeolocationPosition {
    /// Getter of the `coords` attribute.
    /// [`GeolocationPosition.coords`](https://developer.mozilla.org/en-US/docs/Web/API/GeolocationPosition/coords)
    pub fn coords(&self) -> GeolocationCoordinates {
        self.inner.get("coords").as_::<GeolocationCoordinates>()
    }
}
impl GeolocationPosition {
    /// Getter of the `timestamp` attribute.
    /// [`GeolocationPosition.timestamp`](https://developer.mozilla.org/en-US/docs/Web/API/GeolocationPosition/timestamp)
    pub fn timestamp(&self) -> Any {
        self.inner.get("timestamp").as_::<Any>()
    }
}
impl GeolocationPosition {
    /// The toJSON method.
    /// [`GeolocationPosition.toJSON`](https://developer.mozilla.org/en-US/docs/Web/API/GeolocationPosition/toJSON)
    pub fn to_json(&self) -> Object {
        self.inner.call("toJSON", &[]).as_::<Object>()
    }
}
