use super::*;

/// The DOMMatrix2DInit dictionary.
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct DOMMatrix2DInit {
    inner: Any,
}

impl FromVal for DOMMatrix2DInit {
    fn from_val(v: &Any) -> Self {
        DOMMatrix2DInit { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for DOMMatrix2DInit {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for DOMMatrix2DInit {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for DOMMatrix2DInit {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for DOMMatrix2DInit {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}

impl From<DOMMatrix2DInit> for Any {
    fn from(s: DOMMatrix2DInit) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&DOMMatrix2DInit> for Any {
    fn from(s: &DOMMatrix2DInit) -> Any {
        s.inner.clone()
    }
}

impl DOMMatrix2DInit {
    /// Getter of the `a` attribute.
    pub fn a(&self) -> f64 {
        self.inner.get("a").as_::<f64>()
    }

    /// Setter of the `a` attribute.
    pub fn set_a(&mut self, value: f64) {
        self.inner.set("a", value);
    }
}
impl DOMMatrix2DInit {
    /// Getter of the `b` attribute.
    pub fn b(&self) -> f64 {
        self.inner.get("b").as_::<f64>()
    }

    /// Setter of the `b` attribute.
    pub fn set_b(&mut self, value: f64) {
        self.inner.set("b", value);
    }
}
impl DOMMatrix2DInit {
    /// Getter of the `c` attribute.
    pub fn c(&self) -> f64 {
        self.inner.get("c").as_::<f64>()
    }

    /// Setter of the `c` attribute.
    pub fn set_c(&mut self, value: f64) {
        self.inner.set("c", value);
    }
}
impl DOMMatrix2DInit {
    /// Getter of the `d` attribute.
    pub fn d(&self) -> f64 {
        self.inner.get("d").as_::<f64>()
    }

    /// Setter of the `d` attribute.
    pub fn set_d(&mut self, value: f64) {
        self.inner.set("d", value);
    }
}
impl DOMMatrix2DInit {
    /// Getter of the `e` attribute.
    pub fn e(&self) -> f64 {
        self.inner.get("e").as_::<f64>()
    }

    /// Setter of the `e` attribute.
    pub fn set_e(&mut self, value: f64) {
        self.inner.set("e", value);
    }
}
impl DOMMatrix2DInit {
    /// Getter of the `f` attribute.
    pub fn f(&self) -> f64 {
        self.inner.get("f").as_::<f64>()
    }

    /// Setter of the `f` attribute.
    pub fn set_f(&mut self, value: f64) {
        self.inner.set("f", value);
    }
}
impl DOMMatrix2DInit {
    /// Getter of the `m11` attribute.
    pub fn m11(&self) -> f64 {
        self.inner.get("m11").as_::<f64>()
    }

    /// Setter of the `m11` attribute.
    pub fn set_m11(&mut self, value: f64) {
        self.inner.set("m11", value);
    }
}
impl DOMMatrix2DInit {
    /// Getter of the `m12` attribute.
    pub fn m12(&self) -> f64 {
        self.inner.get("m12").as_::<f64>()
    }

    /// Setter of the `m12` attribute.
    pub fn set_m12(&mut self, value: f64) {
        self.inner.set("m12", value);
    }
}
impl DOMMatrix2DInit {
    /// Getter of the `m21` attribute.
    pub fn m21(&self) -> f64 {
        self.inner.get("m21").as_::<f64>()
    }

    /// Setter of the `m21` attribute.
    pub fn set_m21(&mut self, value: f64) {
        self.inner.set("m21", value);
    }
}
impl DOMMatrix2DInit {
    /// Getter of the `m22` attribute.
    pub fn m22(&self) -> f64 {
        self.inner.get("m22").as_::<f64>()
    }

    /// Setter of the `m22` attribute.
    pub fn set_m22(&mut self, value: f64) {
        self.inner.set("m22", value);
    }
}
impl DOMMatrix2DInit {
    /// Getter of the `m41` attribute.
    pub fn m41(&self) -> f64 {
        self.inner.get("m41").as_::<f64>()
    }

    /// Setter of the `m41` attribute.
    pub fn set_m41(&mut self, value: f64) {
        self.inner.set("m41", value);
    }
}
impl DOMMatrix2DInit {
    /// Getter of the `m42` attribute.
    pub fn m42(&self) -> f64 {
        self.inner.get("m42").as_::<f64>()
    }

    /// Setter of the `m42` attribute.
    pub fn set_m42(&mut self, value: f64) {
        self.inner.set("m42", value);
    }
}
