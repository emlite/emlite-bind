use super::*;

/// The MLGemmOptions dictionary.
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct MLGemmOptions {
    inner: Any,
}

impl FromVal for MLGemmOptions {
    fn from_val(v: &Any) -> Self {
        MLGemmOptions { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for MLGemmOptions {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for MLGemmOptions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for MLGemmOptions {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for MLGemmOptions {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}

impl From<MLGemmOptions> for Any {
    fn from(s: MLGemmOptions) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&MLGemmOptions> for Any {
    fn from(s: &MLGemmOptions) -> Any {
        s.inner.clone()
    }
}

impl MLGemmOptions {
    /// Getter of the `c` attribute.
    pub fn c(&self) -> MLOperand {
        self.inner.get("c").as_::<MLOperand>()
    }

    /// Setter of the `c` attribute.
    pub fn set_c(&mut self, value: &MLOperand) {
        self.inner.set("c", value);
    }
}
impl MLGemmOptions {
    /// Getter of the `alpha` attribute.
    pub fn alpha(&self) -> f64 {
        self.inner.get("alpha").as_::<f64>()
    }

    /// Setter of the `alpha` attribute.
    pub fn set_alpha(&mut self, value: f64) {
        self.inner.set("alpha", value);
    }
}
impl MLGemmOptions {
    /// Getter of the `beta` attribute.
    pub fn beta(&self) -> f64 {
        self.inner.get("beta").as_::<f64>()
    }

    /// Setter of the `beta` attribute.
    pub fn set_beta(&mut self, value: f64) {
        self.inner.set("beta", value);
    }
}
impl MLGemmOptions {
    /// Getter of the `aTranspose` attribute.
    pub fn a_transpose(&self) -> bool {
        self.inner.get("aTranspose").as_::<bool>()
    }

    /// Setter of the `aTranspose` attribute.
    pub fn set_a_transpose(&mut self, value: bool) {
        self.inner.set("aTranspose", value);
    }
}
impl MLGemmOptions {
    /// Getter of the `bTranspose` attribute.
    pub fn b_transpose(&self) -> bool {
        self.inner.get("bTranspose").as_::<bool>()
    }

    /// Setter of the `bTranspose` attribute.
    pub fn set_b_transpose(&mut self, value: bool) {
        self.inner.set("bTranspose", value);
    }
}
