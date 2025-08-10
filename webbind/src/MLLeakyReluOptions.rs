use super::*;

/// The MLLeakyReluOptions dictionary.
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct MLLeakyReluOptions {
    inner: Any,
}

impl FromVal for MLLeakyReluOptions {
    fn from_val(v: &Any) -> Self {
        MLLeakyReluOptions { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for MLLeakyReluOptions {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for MLLeakyReluOptions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for MLLeakyReluOptions {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for MLLeakyReluOptions {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}

impl From<MLLeakyReluOptions> for Any {
    fn from(s: MLLeakyReluOptions) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&MLLeakyReluOptions> for Any {
    fn from(s: &MLLeakyReluOptions) -> Any {
        s.inner.clone()
    }
}

impl MLLeakyReluOptions {
    /// Getter of the `alpha` attribute.
    pub fn alpha(&self) -> f64 {
        self.inner.get("alpha").as_::<f64>()
    }

    /// Setter of the `alpha` attribute.
    pub fn set_alpha(&mut self, value: f64) {
        self.inner.set("alpha", value);
    }
}
