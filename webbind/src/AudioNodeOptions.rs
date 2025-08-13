use super::*;




/// The AudioNodeOptions dictionary.
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct AudioNodeOptions {
    inner: Any,
}

impl FromVal for AudioNodeOptions {
    fn from_val(v: &Any) -> Self {
        AudioNodeOptions { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for AudioNodeOptions {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for AudioNodeOptions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for AudioNodeOptions {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for AudioNodeOptions {
    fn as_mut(&mut self) -> &mut Any {
      &mut self.inner
  }
}

impl From<AudioNodeOptions> for Any {
    fn from(s: AudioNodeOptions) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&AudioNodeOptions> for Any {
    fn from(s: &AudioNodeOptions) -> Any {
        s.inner.clone()
    }
}

impl AudioNodeOptions {
    /// Getter of the `channelCount` attribute.
    pub fn channel_count(&self) -> u32 {
        self.inner.get("channelCount").as_::<u32>()
    }

    /// Setter of the `channelCount` attribute.
    pub fn set_channel_count(&mut self, value: u32) {
        self.inner.set("channelCount", value);
    }
}
impl AudioNodeOptions {
    /// Getter of the `channelCountMode` attribute.
    pub fn channel_count_mode(&self) -> ChannelCountMode {
        self.inner.get("channelCountMode").as_::<ChannelCountMode>()
    }

    /// Setter of the `channelCountMode` attribute.
    pub fn set_channel_count_mode(&mut self, value: &ChannelCountMode) {
        self.inner.set("channelCountMode", value);
    }
}
impl AudioNodeOptions {
    /// Getter of the `channelInterpretation` attribute.
    pub fn channel_interpretation(&self) -> ChannelInterpretation {
        self.inner.get("channelInterpretation").as_::<ChannelInterpretation>()
    }

    /// Setter of the `channelInterpretation` attribute.
    pub fn set_channel_interpretation(&mut self, value: &ChannelInterpretation) {
        self.inner.set("channelInterpretation", value);
    }
}
