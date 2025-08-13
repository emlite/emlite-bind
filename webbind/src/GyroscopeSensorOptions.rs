use super::*;




/// The GyroscopeSensorOptions dictionary.
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct GyroscopeSensorOptions {
    inner: Any,
}

impl FromVal for GyroscopeSensorOptions {
    fn from_val(v: &Any) -> Self {
        GyroscopeSensorOptions { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for GyroscopeSensorOptions {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for GyroscopeSensorOptions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for GyroscopeSensorOptions {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for GyroscopeSensorOptions {
    fn as_mut(&mut self) -> &mut Any {
      &mut self.inner
  }
}

impl From<GyroscopeSensorOptions> for Any {
    fn from(s: GyroscopeSensorOptions) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&GyroscopeSensorOptions> for Any {
    fn from(s: &GyroscopeSensorOptions) -> Any {
        s.inner.clone()
    }
}

impl GyroscopeSensorOptions {
    /// Getter of the `referenceFrame` attribute.
    pub fn reference_frame(&self) -> GyroscopeLocalCoordinateSystem {
        self.inner.get("referenceFrame").as_::<GyroscopeLocalCoordinateSystem>()
    }

    /// Setter of the `referenceFrame` attribute.
    pub fn set_reference_frame(&mut self, value: &GyroscopeLocalCoordinateSystem) {
        self.inner.set("referenceFrame", value);
    }
}
