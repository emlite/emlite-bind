use super::*;




/// The BackgroundFetchEventInit dictionary.
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct BackgroundFetchEventInit {
    inner: Any,
}

impl FromVal for BackgroundFetchEventInit {
    fn from_val(v: &Any) -> Self {
        BackgroundFetchEventInit { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for BackgroundFetchEventInit {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for BackgroundFetchEventInit {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for BackgroundFetchEventInit {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for BackgroundFetchEventInit {
    fn as_mut(&mut self) -> &mut Any {
      &mut self.inner
  }
}

impl From<BackgroundFetchEventInit> for Any {
    fn from(s: BackgroundFetchEventInit) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&BackgroundFetchEventInit> for Any {
    fn from(s: &BackgroundFetchEventInit) -> Any {
        s.inner.clone()
    }
}

impl BackgroundFetchEventInit {
    /// Getter of the `registration` attribute.
    pub fn registration(&self) -> BackgroundFetchRegistration {
        self.inner.get("registration").as_::<BackgroundFetchRegistration>()
    }

    /// Setter of the `registration` attribute.
    pub fn set_registration(&mut self, value: &BackgroundFetchRegistration) {
        self.inner.set("registration", value);
    }
}
