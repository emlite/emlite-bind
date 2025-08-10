use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct MockCapturePromptResultConfiguration {
    inner: Any,
}
impl FromVal for MockCapturePromptResultConfiguration {
    fn from_val(v: &Any) -> Self {
        MockCapturePromptResultConfiguration { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for MockCapturePromptResultConfiguration {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for MockCapturePromptResultConfiguration {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for MockCapturePromptResultConfiguration {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for MockCapturePromptResultConfiguration {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<MockCapturePromptResultConfiguration> for Any {
    fn from(s: MockCapturePromptResultConfiguration) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&MockCapturePromptResultConfiguration> for Any {
    fn from(s: &MockCapturePromptResultConfiguration) -> Any {
        s.inner.clone()
    }
}

impl MockCapturePromptResultConfiguration {
    pub fn get_user_media(&self) -> MockCapturePromptResult {
        self.inner
            .get("getUserMedia")
            .as_::<MockCapturePromptResult>()
    }

    pub fn set_get_user_media(&mut self, value: &MockCapturePromptResult) {
        self.inner.set("getUserMedia", value);
    }
}
impl MockCapturePromptResultConfiguration {
    pub fn get_display_media(&self) -> MockCapturePromptResult {
        self.inner
            .get("getDisplayMedia")
            .as_::<MockCapturePromptResult>()
    }

    pub fn set_get_display_media(&mut self, value: &MockCapturePromptResult) {
        self.inner.set("getDisplayMedia", value);
    }
}
