use super::*;




/// The MockCaptureDeviceConfiguration dictionary.
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct MockCaptureDeviceConfiguration {
    inner: Any,
}

impl FromVal for MockCaptureDeviceConfiguration {
    fn from_val(v: &Any) -> Self {
        MockCaptureDeviceConfiguration { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for MockCaptureDeviceConfiguration {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for MockCaptureDeviceConfiguration {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for MockCaptureDeviceConfiguration {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for MockCaptureDeviceConfiguration {
    fn as_mut(&mut self) -> &mut Any {
      &mut self.inner
  }
}

impl From<MockCaptureDeviceConfiguration> for Any {
    fn from(s: MockCaptureDeviceConfiguration) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&MockCaptureDeviceConfiguration> for Any {
    fn from(s: &MockCaptureDeviceConfiguration) -> Any {
        s.inner.clone()
    }
}

impl MockCaptureDeviceConfiguration {
    /// Getter of the `label` attribute.
    pub fn label(&self) -> JsString {
        self.inner.get("label").as_::<JsString>()
    }

    /// Setter of the `label` attribute.
    pub fn set_label(&mut self, value: &JsString) {
        self.inner.set("label", value);
    }
}
impl MockCaptureDeviceConfiguration {
    /// Getter of the `deviceId` attribute.
    pub fn device_id(&self) -> JsString {
        self.inner.get("deviceId").as_::<JsString>()
    }

    /// Setter of the `deviceId` attribute.
    pub fn set_device_id(&mut self, value: &JsString) {
        self.inner.set("deviceId", value);
    }
}
impl MockCaptureDeviceConfiguration {
    /// Getter of the `groupId` attribute.
    pub fn group_id(&self) -> JsString {
        self.inner.get("groupId").as_::<JsString>()
    }

    /// Setter of the `groupId` attribute.
    pub fn set_group_id(&mut self, value: &JsString) {
        self.inner.set("groupId", value);
    }
}
