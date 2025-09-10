use super::*;

/// The MLPadOptions dictionary.
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct MLPadOptions {
    inner: Any,
}

impl FromVal for MLPadOptions {
    fn from_val(v: &Any) -> Self {
        MLPadOptions { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for MLPadOptions {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for MLPadOptions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for MLPadOptions {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for MLPadOptions {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}

impl From<MLPadOptions> for Any {
    fn from(s: MLPadOptions) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&MLPadOptions> for Any {
    fn from(s: &MLPadOptions) -> Any {
        s.inner.clone()
    }
}

impl MLPadOptions {
    /// Getter of the `mode` attribute.
    pub fn mode(&self) -> MLPaddingMode {
        self.inner.get("mode").as_::<MLPaddingMode>()
    }

    /// Setter of the `mode` attribute.
    pub fn set_mode(&mut self, value: &MLPaddingMode) {
        self.inner.set("mode", value);
    }
}
impl MLPadOptions {
    /// Getter of the `value` attribute.
    pub fn value(&self) -> Any {
        self.inner.get("value").as_::<Any>()
    }

    /// Setter of the `value` attribute.
    pub fn set_value(&mut self, value: &Any) {
        self.inner.set("value", value);
    }
}
