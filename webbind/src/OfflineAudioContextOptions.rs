use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct OfflineAudioContextOptions {
    inner: Any,
}
impl FromVal for OfflineAudioContextOptions {
    fn from_val(v: &Any) -> Self {
        OfflineAudioContextOptions { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for OfflineAudioContextOptions {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for OfflineAudioContextOptions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for OfflineAudioContextOptions {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for OfflineAudioContextOptions {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<OfflineAudioContextOptions> for Any {
    fn from(s: OfflineAudioContextOptions) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&OfflineAudioContextOptions> for Any {
    fn from(s: &OfflineAudioContextOptions) -> Any {
        s.inner.clone()
    }
}

impl OfflineAudioContextOptions {
    pub fn number_of_channels(&self) -> u32 {
        self.inner.get("numberOfChannels").as_::<u32>()
    }

    pub fn set_number_of_channels(&mut self, value: u32) {
        self.inner.set("numberOfChannels", value);
    }
}
impl OfflineAudioContextOptions {
    pub fn length(&self) -> u32 {
        self.inner.get("length").as_::<u32>()
    }

    pub fn set_length(&mut self, value: u32) {
        self.inner.set("length", value);
    }
}
impl OfflineAudioContextOptions {
    pub fn sample_rate(&self) -> f32 {
        self.inner.get("sampleRate").as_::<f32>()
    }

    pub fn set_sample_rate(&mut self, value: f32) {
        self.inner.set("sampleRate", value);
    }
}
impl OfflineAudioContextOptions {
    pub fn render_size_hint(&self) -> Any {
        self.inner.get("renderSizeHint").as_::<Any>()
    }

    pub fn set_render_size_hint(&mut self, value: &Any) {
        self.inner.set("renderSizeHint", value);
    }
}
