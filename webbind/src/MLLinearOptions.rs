use super::*;

/// The MLLinearOptions dictionary.
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct MLLinearOptions {
    inner: Any,
}

impl FromVal for MLLinearOptions {
    fn from_val(v: &Any) -> Self {
        MLLinearOptions { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for MLLinearOptions {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for MLLinearOptions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for MLLinearOptions {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for MLLinearOptions {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}

impl From<MLLinearOptions> for Any {
    fn from(s: MLLinearOptions) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&MLLinearOptions> for Any {
    fn from(s: &MLLinearOptions) -> Any {
        s.inner.clone()
    }
}

impl MLLinearOptions {
    /// Getter of the `alpha` attribute.
    pub fn alpha(&self) -> f64 {
        self.inner.get("alpha").as_::<f64>()
    }

    /// Setter of the `alpha` attribute.
    pub fn set_alpha(&mut self, value: f64) {
        self.inner.set("alpha", value);
    }
}
impl MLLinearOptions {
    /// Getter of the `beta` attribute.
    pub fn beta(&self) -> f64 {
        self.inner.get("beta").as_::<f64>()
    }

    /// Setter of the `beta` attribute.
    pub fn set_beta(&mut self, value: f64) {
        self.inner.set("beta", value);
    }
}
