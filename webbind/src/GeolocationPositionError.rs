use super::*;




/// The GeolocationPositionError class.
/// [`GeolocationPositionError`](https://developer.mozilla.org/en-US/docs/Web/API/GeolocationPositionError)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct GeolocationPositionError {
    inner: Any,
}

impl FromVal for GeolocationPositionError {
    fn from_val(v: &Any) -> Self {
        GeolocationPositionError { inner: Any::from_val(v) }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for GeolocationPositionError {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for GeolocationPositionError {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for GeolocationPositionError {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for GeolocationPositionError {
    fn as_mut(&mut self) -> &mut Any {
      &mut self.inner
  }
}

impl From<GeolocationPositionError> for Any {
    fn from(s: GeolocationPositionError) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&GeolocationPositionError> for Any {
    fn from(s: &GeolocationPositionError) -> Any {
        s.inner.clone().into()
    }
}

jsbind::utils::impl_dyn_cast!(GeolocationPositionError);


impl GeolocationPositionError {
    /// Getter of the `code` attribute.
    /// [`GeolocationPositionError.code`](https://developer.mozilla.org/en-US/docs/Web/API/GeolocationPositionError/code)
    pub fn code(&self) -> u16 {
        self.inner.get("code").as_::<u16>()
    }

}
impl GeolocationPositionError {
    /// Getter of the `message` attribute.
    /// [`GeolocationPositionError.message`](https://developer.mozilla.org/en-US/docs/Web/API/GeolocationPositionError/message)
    pub fn message(&self) -> JsString {
        self.inner.get("message").as_::<JsString>()
    }

}
