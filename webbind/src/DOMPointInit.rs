use super::*;

/// The DOMPointInit dictionary.
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct DOMPointInit {
    inner: Any,
}

impl FromVal for DOMPointInit {
    fn from_val(v: &Any) -> Self {
        DOMPointInit { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for DOMPointInit {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for DOMPointInit {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for DOMPointInit {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for DOMPointInit {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}

impl From<DOMPointInit> for Any {
    fn from(s: DOMPointInit) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&DOMPointInit> for Any {
    fn from(s: &DOMPointInit) -> Any {
        s.inner.clone()
    }
}

impl DOMPointInit {
    /// Getter of the `x` attribute.
    pub fn x(&self) -> f64 {
        self.inner.get("x").as_::<f64>()
    }

    /// Setter of the `x` attribute.
    pub fn set_x(&mut self, value: f64) {
        self.inner.set("x", value);
    }
}
impl DOMPointInit {
    /// Getter of the `y` attribute.
    pub fn y(&self) -> f64 {
        self.inner.get("y").as_::<f64>()
    }

    /// Setter of the `y` attribute.
    pub fn set_y(&mut self, value: f64) {
        self.inner.set("y", value);
    }
}
impl DOMPointInit {
    /// Getter of the `z` attribute.
    pub fn z(&self) -> f64 {
        self.inner.get("z").as_::<f64>()
    }

    /// Setter of the `z` attribute.
    pub fn set_z(&mut self, value: f64) {
        self.inner.set("z", value);
    }
}
impl DOMPointInit {
    /// Getter of the `w` attribute.
    pub fn w(&self) -> f64 {
        self.inner.get("w").as_::<f64>()
    }

    /// Setter of the `w` attribute.
    pub fn set_w(&mut self, value: f64) {
        self.inner.set("w", value);
    }
}
