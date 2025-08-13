use super::*;




/// The WheelEventInit dictionary.
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct WheelEventInit {
    inner: Any,
}

impl FromVal for WheelEventInit {
    fn from_val(v: &Any) -> Self {
        WheelEventInit { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for WheelEventInit {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for WheelEventInit {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for WheelEventInit {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for WheelEventInit {
    fn as_mut(&mut self) -> &mut Any {
      &mut self.inner
  }
}

impl From<WheelEventInit> for Any {
    fn from(s: WheelEventInit) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&WheelEventInit> for Any {
    fn from(s: &WheelEventInit) -> Any {
        s.inner.clone()
    }
}

impl WheelEventInit {
    /// Getter of the `deltaX` attribute.
    pub fn delta_x(&self) -> f64 {
        self.inner.get("deltaX").as_::<f64>()
    }

    /// Setter of the `deltaX` attribute.
    pub fn set_delta_x(&mut self, value: f64) {
        self.inner.set("deltaX", value);
    }
}
impl WheelEventInit {
    /// Getter of the `deltaY` attribute.
    pub fn delta_y(&self) -> f64 {
        self.inner.get("deltaY").as_::<f64>()
    }

    /// Setter of the `deltaY` attribute.
    pub fn set_delta_y(&mut self, value: f64) {
        self.inner.set("deltaY", value);
    }
}
impl WheelEventInit {
    /// Getter of the `deltaZ` attribute.
    pub fn delta_z(&self) -> f64 {
        self.inner.get("deltaZ").as_::<f64>()
    }

    /// Setter of the `deltaZ` attribute.
    pub fn set_delta_z(&mut self, value: f64) {
        self.inner.set("deltaZ", value);
    }
}
impl WheelEventInit {
    /// Getter of the `deltaMode` attribute.
    pub fn delta_mode(&self) -> u32 {
        self.inner.get("deltaMode").as_::<u32>()
    }

    /// Setter of the `deltaMode` attribute.
    pub fn set_delta_mode(&mut self, value: u32) {
        self.inner.set("deltaMode", value);
    }
}
