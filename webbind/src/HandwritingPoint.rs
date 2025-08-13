use super::*;




/// The HandwritingPoint dictionary.
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct HandwritingPoint {
    inner: Any,
}

impl FromVal for HandwritingPoint {
    fn from_val(v: &Any) -> Self {
        HandwritingPoint { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for HandwritingPoint {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for HandwritingPoint {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for HandwritingPoint {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for HandwritingPoint {
    fn as_mut(&mut self) -> &mut Any {
      &mut self.inner
  }
}

impl From<HandwritingPoint> for Any {
    fn from(s: HandwritingPoint) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&HandwritingPoint> for Any {
    fn from(s: &HandwritingPoint) -> Any {
        s.inner.clone()
    }
}

impl HandwritingPoint {
    /// Getter of the `x` attribute.
    pub fn x(&self) -> f64 {
        self.inner.get("x").as_::<f64>()
    }

    /// Setter of the `x` attribute.
    pub fn set_x(&mut self, value: f64) {
        self.inner.set("x", value);
    }
}
impl HandwritingPoint {
    /// Getter of the `y` attribute.
    pub fn y(&self) -> f64 {
        self.inner.get("y").as_::<f64>()
    }

    /// Setter of the `y` attribute.
    pub fn set_y(&mut self, value: f64) {
        self.inner.set("y", value);
    }
}
impl HandwritingPoint {
    /// Getter of the `t` attribute.
    pub fn t(&self) -> Any {
        self.inner.get("t").as_::<Any>()
    }

    /// Setter of the `t` attribute.
    pub fn set_t(&mut self, value: &Any) {
        self.inner.set("t", value);
    }
}
