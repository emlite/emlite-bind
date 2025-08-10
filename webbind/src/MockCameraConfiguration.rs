use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct MockCameraConfiguration {
    inner: Any,
}
impl FromVal for MockCameraConfiguration {
    fn from_val(v: &Any) -> Self {
        MockCameraConfiguration { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for MockCameraConfiguration {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for MockCameraConfiguration {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for MockCameraConfiguration {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for MockCameraConfiguration {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<MockCameraConfiguration> for Any {
    fn from(s: MockCameraConfiguration) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&MockCameraConfiguration> for Any {
    fn from(s: &MockCameraConfiguration) -> Any {
        s.inner.clone()
    }
}

impl MockCameraConfiguration {
    pub fn default_frame_rate(&self) -> f64 {
        self.inner.get("defaultFrameRate").as_::<f64>()
    }

    pub fn set_default_frame_rate(&mut self, value: f64) {
        self.inner.set("defaultFrameRate", value);
    }
}
impl MockCameraConfiguration {
    pub fn facing_mode(&self) -> JsString {
        self.inner.get("facingMode").as_::<JsString>()
    }

    pub fn set_facing_mode(&mut self, value: &JsString) {
        self.inner.set("facingMode", value);
    }
}
