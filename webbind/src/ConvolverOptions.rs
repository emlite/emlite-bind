use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct ConvolverOptions {
    inner: Any,
}
impl FromVal for ConvolverOptions {
    fn from_val(v: &Any) -> Self {
        ConvolverOptions { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for ConvolverOptions {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for ConvolverOptions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for ConvolverOptions {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for ConvolverOptions {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<ConvolverOptions> for Any {
    fn from(s: ConvolverOptions) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&ConvolverOptions> for Any {
    fn from(s: &ConvolverOptions) -> Any {
        s.inner.clone()
    }
}

impl ConvolverOptions {
    pub fn buffer(&self) -> AudioBuffer {
        self.inner.get("buffer").as_::<AudioBuffer>()
    }

    pub fn set_buffer(&mut self, value: &AudioBuffer) {
        self.inner.set("buffer", value);
    }
}
impl ConvolverOptions {
    pub fn disable_normalization(&self) -> bool {
        self.inner.get("disableNormalization").as_::<bool>()
    }

    pub fn set_disable_normalization(&mut self, value: bool) {
        self.inner.set("disableNormalization", value);
    }
}
