use super::*;




/// The DOMMatrixInit dictionary.
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct DOMMatrixInit {
    inner: Any,
}

impl FromVal for DOMMatrixInit {
    fn from_val(v: &Any) -> Self {
        DOMMatrixInit { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for DOMMatrixInit {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for DOMMatrixInit {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for DOMMatrixInit {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for DOMMatrixInit {
    fn as_mut(&mut self) -> &mut Any {
      &mut self.inner
  }
}

impl From<DOMMatrixInit> for Any {
    fn from(s: DOMMatrixInit) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&DOMMatrixInit> for Any {
    fn from(s: &DOMMatrixInit) -> Any {
        s.inner.clone()
    }
}

impl DOMMatrixInit {
    /// Getter of the `m13` attribute.
    pub fn m13(&self) -> f64 {
        self.inner.get("m13").as_::<f64>()
    }

    /// Setter of the `m13` attribute.
    pub fn set_m13(&mut self, value: f64) {
        self.inner.set("m13", value);
    }
}
impl DOMMatrixInit {
    /// Getter of the `m14` attribute.
    pub fn m14(&self) -> f64 {
        self.inner.get("m14").as_::<f64>()
    }

    /// Setter of the `m14` attribute.
    pub fn set_m14(&mut self, value: f64) {
        self.inner.set("m14", value);
    }
}
impl DOMMatrixInit {
    /// Getter of the `m23` attribute.
    pub fn m23(&self) -> f64 {
        self.inner.get("m23").as_::<f64>()
    }

    /// Setter of the `m23` attribute.
    pub fn set_m23(&mut self, value: f64) {
        self.inner.set("m23", value);
    }
}
impl DOMMatrixInit {
    /// Getter of the `m24` attribute.
    pub fn m24(&self) -> f64 {
        self.inner.get("m24").as_::<f64>()
    }

    /// Setter of the `m24` attribute.
    pub fn set_m24(&mut self, value: f64) {
        self.inner.set("m24", value);
    }
}
impl DOMMatrixInit {
    /// Getter of the `m31` attribute.
    pub fn m31(&self) -> f64 {
        self.inner.get("m31").as_::<f64>()
    }

    /// Setter of the `m31` attribute.
    pub fn set_m31(&mut self, value: f64) {
        self.inner.set("m31", value);
    }
}
impl DOMMatrixInit {
    /// Getter of the `m32` attribute.
    pub fn m32(&self) -> f64 {
        self.inner.get("m32").as_::<f64>()
    }

    /// Setter of the `m32` attribute.
    pub fn set_m32(&mut self, value: f64) {
        self.inner.set("m32", value);
    }
}
impl DOMMatrixInit {
    /// Getter of the `m33` attribute.
    pub fn m33(&self) -> f64 {
        self.inner.get("m33").as_::<f64>()
    }

    /// Setter of the `m33` attribute.
    pub fn set_m33(&mut self, value: f64) {
        self.inner.set("m33", value);
    }
}
impl DOMMatrixInit {
    /// Getter of the `m34` attribute.
    pub fn m34(&self) -> f64 {
        self.inner.get("m34").as_::<f64>()
    }

    /// Setter of the `m34` attribute.
    pub fn set_m34(&mut self, value: f64) {
        self.inner.set("m34", value);
    }
}
impl DOMMatrixInit {
    /// Getter of the `m43` attribute.
    pub fn m43(&self) -> f64 {
        self.inner.get("m43").as_::<f64>()
    }

    /// Setter of the `m43` attribute.
    pub fn set_m43(&mut self, value: f64) {
        self.inner.set("m43", value);
    }
}
impl DOMMatrixInit {
    /// Getter of the `m44` attribute.
    pub fn m44(&self) -> f64 {
        self.inner.get("m44").as_::<f64>()
    }

    /// Setter of the `m44` attribute.
    pub fn set_m44(&mut self, value: f64) {
        self.inner.set("m44", value);
    }
}
impl DOMMatrixInit {
    /// Getter of the `is2D` attribute.
    pub fn is2_d(&self) -> bool {
        self.inner.get("is2D").as_::<bool>()
    }

    /// Setter of the `is2D` attribute.
    pub fn set_is2_d(&mut self, value: bool) {
        self.inner.set("is2D", value);
    }
}
