use super::*;




/// The AccelerometerSensorOptions dictionary.
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct AccelerometerSensorOptions {
    inner: Any,
}

impl FromVal for AccelerometerSensorOptions {
    fn from_val(v: &Any) -> Self {
        AccelerometerSensorOptions { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for AccelerometerSensorOptions {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for AccelerometerSensorOptions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for AccelerometerSensorOptions {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for AccelerometerSensorOptions {
    fn as_mut(&mut self) -> &mut Any {
      &mut self.inner
  }
}

impl From<AccelerometerSensorOptions> for Any {
    fn from(s: AccelerometerSensorOptions) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&AccelerometerSensorOptions> for Any {
    fn from(s: &AccelerometerSensorOptions) -> Any {
        s.inner.clone()
    }
}

impl AccelerometerSensorOptions {
    /// Getter of the `referenceFrame` attribute.
    pub fn reference_frame(&self) -> AccelerometerLocalCoordinateSystem {
        self.inner.get("referenceFrame").as_::<AccelerometerLocalCoordinateSystem>()
    }

    /// Setter of the `referenceFrame` attribute.
    pub fn set_reference_frame(&mut self, value: &AccelerometerLocalCoordinateSystem) {
        self.inner.set("referenceFrame", value);
    }
}
