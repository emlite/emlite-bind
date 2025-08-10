use super::*;

/// The MockMicrophoneConfiguration dictionary.
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct MockMicrophoneConfiguration {
    inner: Any,
}

impl FromVal for MockMicrophoneConfiguration {
    fn from_val(v: &Any) -> Self {
        MockMicrophoneConfiguration { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for MockMicrophoneConfiguration {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for MockMicrophoneConfiguration {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for MockMicrophoneConfiguration {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for MockMicrophoneConfiguration {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}

impl From<MockMicrophoneConfiguration> for Any {
    fn from(s: MockMicrophoneConfiguration) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&MockMicrophoneConfiguration> for Any {
    fn from(s: &MockMicrophoneConfiguration) -> Any {
        s.inner.clone()
    }
}

impl MockMicrophoneConfiguration {
    /// Getter of the `defaultSampleRate` attribute.
    pub fn default_sample_rate(&self) -> u32 {
        self.inner.get("defaultSampleRate").as_::<u32>()
    }

    /// Setter of the `defaultSampleRate` attribute.
    pub fn set_default_sample_rate(&mut self, value: u32) {
        self.inner.set("defaultSampleRate", value);
    }
}
