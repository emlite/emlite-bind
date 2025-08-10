use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct AudioSinkOptions {
    inner: Any,
}
impl FromVal for AudioSinkOptions {
    fn from_val(v: &Any) -> Self {
        AudioSinkOptions { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for AudioSinkOptions {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for AudioSinkOptions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for AudioSinkOptions {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for AudioSinkOptions {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<AudioSinkOptions> for Any {
    fn from(s: AudioSinkOptions) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&AudioSinkOptions> for Any {
    fn from(s: &AudioSinkOptions) -> Any {
        s.inner.clone()
    }
}

impl AudioSinkOptions {
    pub fn type_(&self) -> AudioSinkType {
        self.inner.get("type").as_::<AudioSinkType>()
    }

    pub fn set_type_(&mut self, value: &AudioSinkType) {
        self.inner.set("type", value);
    }
}
