use super::*;

/// The AudioBuffer class.
/// [`AudioBuffer`](https://developer.mozilla.org/en-US/docs/Web/API/AudioBuffer)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct AudioBuffer {
    inner: Any,
}

impl FromVal for AudioBuffer {
    fn from_val(v: &Any) -> Self {
        AudioBuffer {
            inner: Any::from_val(v),
        }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for AudioBuffer {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for AudioBuffer {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for AudioBuffer {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for AudioBuffer {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}

impl From<AudioBuffer> for Any {
    fn from(s: AudioBuffer) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&AudioBuffer> for Any {
    fn from(s: &AudioBuffer) -> Any {
        s.inner.clone().into()
    }
}

jsbind::utils::impl_dyn_cast!(AudioBuffer);

impl AudioBuffer {
    /// Getter of the `sampleRate` attribute.
    /// [`AudioBuffer.sampleRate`](https://developer.mozilla.org/en-US/docs/Web/API/AudioBuffer/sampleRate)
    pub fn sample_rate(&self) -> f32 {
        self.inner.get("sampleRate").as_::<f32>()
    }
}
impl AudioBuffer {
    /// Getter of the `length` attribute.
    /// [`AudioBuffer.length`](https://developer.mozilla.org/en-US/docs/Web/API/AudioBuffer/length)
    pub fn length(&self) -> u32 {
        self.inner.get("length").as_::<u32>()
    }
}
impl AudioBuffer {
    /// Getter of the `duration` attribute.
    /// [`AudioBuffer.duration`](https://developer.mozilla.org/en-US/docs/Web/API/AudioBuffer/duration)
    pub fn duration(&self) -> f64 {
        self.inner.get("duration").as_::<f64>()
    }
}
impl AudioBuffer {
    /// Getter of the `numberOfChannels` attribute.
    /// [`AudioBuffer.numberOfChannels`](https://developer.mozilla.org/en-US/docs/Web/API/AudioBuffer/numberOfChannels)
    pub fn number_of_channels(&self) -> u32 {
        self.inner.get("numberOfChannels").as_::<u32>()
    }
}

impl AudioBuffer {
    /// The `new AudioBuffer(..)` constructor, creating a new AudioBuffer instance
    pub fn new(options: &AudioBufferOptions) -> AudioBuffer {
        Self {
            inner: Any::global("AudioBuffer")
                .new(&[options.into()])
                .as_::<Any>(),
        }
    }
}

impl AudioBuffer {
    /// The getChannelData method.
    /// [`AudioBuffer.getChannelData`](https://developer.mozilla.org/en-US/docs/Web/API/AudioBuffer/getChannelData)
    pub fn get_channel_data(&self, channel: u32) -> Float32Array {
        self.inner
            .call("getChannelData", &[channel.into()])
            .as_::<Float32Array>()
    }
}
impl AudioBuffer {
    /// The copyFromChannel method.
    /// [`AudioBuffer.copyFromChannel`](https://developer.mozilla.org/en-US/docs/Web/API/AudioBuffer/copyFromChannel)
    pub fn copy_from_channel(&self, destination: &Float32Array, channel_number: u32) -> Undefined {
        self.inner
            .call(
                "copyFromChannel",
                &[destination.into(), channel_number.into()],
            )
            .as_::<Undefined>()
    }
}
impl AudioBuffer {
    /// The copyFromChannel method.
    /// [`AudioBuffer.copyFromChannel`](https://developer.mozilla.org/en-US/docs/Web/API/AudioBuffer/copyFromChannel)
    pub fn copy_from_channel_with_buffer_offset(
        &self,
        destination: &Float32Array,
        channel_number: u32,
        buffer_offset: u32,
    ) -> Undefined {
        self.inner
            .call(
                "copyFromChannel",
                &[
                    destination.into(),
                    channel_number.into(),
                    buffer_offset.into(),
                ],
            )
            .as_::<Undefined>()
    }
}
impl AudioBuffer {
    /// The copyToChannel method.
    /// [`AudioBuffer.copyToChannel`](https://developer.mozilla.org/en-US/docs/Web/API/AudioBuffer/copyToChannel)
    pub fn copy_to_channel(&self, source: &Float32Array, channel_number: u32) -> Undefined {
        self.inner
            .call("copyToChannel", &[source.into(), channel_number.into()])
            .as_::<Undefined>()
    }
}
impl AudioBuffer {
    /// The copyToChannel method.
    /// [`AudioBuffer.copyToChannel`](https://developer.mozilla.org/en-US/docs/Web/API/AudioBuffer/copyToChannel)
    pub fn copy_to_channel_with_buffer_offset(
        &self,
        source: &Float32Array,
        channel_number: u32,
        buffer_offset: u32,
    ) -> Undefined {
        self.inner
            .call(
                "copyToChannel",
                &[source.into(), channel_number.into(), buffer_offset.into()],
            )
            .as_::<Undefined>()
    }
}
