use super::*;

/// The AudioOutputOptions dictionary.
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct AudioOutputOptions {
    inner: Any,
}

impl FromVal for AudioOutputOptions {
    fn from_val(v: &Any) -> Self {
        AudioOutputOptions { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for AudioOutputOptions {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for AudioOutputOptions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for AudioOutputOptions {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for AudioOutputOptions {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}

impl From<AudioOutputOptions> for Any {
    fn from(s: AudioOutputOptions) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&AudioOutputOptions> for Any {
    fn from(s: &AudioOutputOptions) -> Any {
        s.inner.clone()
    }
}

impl AudioOutputOptions {
    /// Getter of the `deviceId` attribute.
    pub fn device_id(&self) -> JsString {
        self.inner.get("deviceId").as_::<JsString>()
    }

    /// Setter of the `deviceId` attribute.
    pub fn set_device_id(&mut self, value: &JsString) {
        self.inner.set("deviceId", value);
    }
}
