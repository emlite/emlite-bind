use super::*;




/// The DOMRectInit dictionary.
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct DOMRectInit {
    inner: Any,
}

impl FromVal for DOMRectInit {
    fn from_val(v: &Any) -> Self {
        DOMRectInit { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for DOMRectInit {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for DOMRectInit {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for DOMRectInit {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for DOMRectInit {
    fn as_mut(&mut self) -> &mut Any {
      &mut self.inner
  }
}

impl From<DOMRectInit> for Any {
    fn from(s: DOMRectInit) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&DOMRectInit> for Any {
    fn from(s: &DOMRectInit) -> Any {
        s.inner.clone()
    }
}

impl DOMRectInit {
    /// Getter of the `x` attribute.
    pub fn x(&self) -> f64 {
        self.inner.get("x").as_::<f64>()
    }

    /// Setter of the `x` attribute.
    pub fn set_x(&mut self, value: f64) {
        self.inner.set("x", value);
    }
}
impl DOMRectInit {
    /// Getter of the `y` attribute.
    pub fn y(&self) -> f64 {
        self.inner.get("y").as_::<f64>()
    }

    /// Setter of the `y` attribute.
    pub fn set_y(&mut self, value: f64) {
        self.inner.set("y", value);
    }
}
impl DOMRectInit {
    /// Getter of the `width` attribute.
    pub fn width(&self) -> f64 {
        self.inner.get("width").as_::<f64>()
    }

    /// Setter of the `width` attribute.
    pub fn set_width(&mut self, value: f64) {
        self.inner.set("width", value);
    }
}
impl DOMRectInit {
    /// Getter of the `height` attribute.
    pub fn height(&self) -> f64 {
        self.inner.get("height").as_::<f64>()
    }

    /// Setter of the `height` attribute.
    pub fn set_height(&mut self, value: f64) {
        self.inner.set("height", value);
    }
}
