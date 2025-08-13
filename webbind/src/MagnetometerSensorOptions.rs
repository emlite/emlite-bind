use super::*;




/// The MagnetometerSensorOptions dictionary.
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct MagnetometerSensorOptions {
    inner: Any,
}

impl FromVal for MagnetometerSensorOptions {
    fn from_val(v: &Any) -> Self {
        MagnetometerSensorOptions { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for MagnetometerSensorOptions {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for MagnetometerSensorOptions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for MagnetometerSensorOptions {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for MagnetometerSensorOptions {
    fn as_mut(&mut self) -> &mut Any {
      &mut self.inner
  }
}

impl From<MagnetometerSensorOptions> for Any {
    fn from(s: MagnetometerSensorOptions) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&MagnetometerSensorOptions> for Any {
    fn from(s: &MagnetometerSensorOptions) -> Any {
        s.inner.clone()
    }
}

impl MagnetometerSensorOptions {
    /// Getter of the `referenceFrame` attribute.
    pub fn reference_frame(&self) -> MagnetometerLocalCoordinateSystem {
        self.inner.get("referenceFrame").as_::<MagnetometerLocalCoordinateSystem>()
    }

    /// Setter of the `referenceFrame` attribute.
    pub fn set_reference_frame(&mut self, value: &MagnetometerLocalCoordinateSystem) {
        self.inner.set("referenceFrame", value);
    }
}
