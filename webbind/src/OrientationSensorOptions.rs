use super::*;




/// The OrientationSensorOptions dictionary.
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct OrientationSensorOptions {
    inner: Any,
}

impl FromVal for OrientationSensorOptions {
    fn from_val(v: &Any) -> Self {
        OrientationSensorOptions { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for OrientationSensorOptions {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for OrientationSensorOptions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for OrientationSensorOptions {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for OrientationSensorOptions {
    fn as_mut(&mut self) -> &mut Any {
      &mut self.inner
  }
}

impl From<OrientationSensorOptions> for Any {
    fn from(s: OrientationSensorOptions) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&OrientationSensorOptions> for Any {
    fn from(s: &OrientationSensorOptions) -> Any {
        s.inner.clone()
    }
}

impl OrientationSensorOptions {
    /// Getter of the `referenceFrame` attribute.
    pub fn reference_frame(&self) -> OrientationSensorLocalCoordinateSystem {
        self.inner.get("referenceFrame").as_::<OrientationSensorLocalCoordinateSystem>()
    }

    /// Setter of the `referenceFrame` attribute.
    pub fn set_reference_frame(&mut self, value: &OrientationSensorLocalCoordinateSystem) {
        self.inner.set("referenceFrame", value);
    }
}
