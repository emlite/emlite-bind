use super::*;

/// The AudioData class.
/// [`AudioData`](https://developer.mozilla.org/en-US/docs/Web/API/AudioData)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct AudioData {
    inner: Any,
}

impl FromVal for AudioData {
    fn from_val(v: &Any) -> Self {
        AudioData {
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

impl core::ops::Deref for AudioData {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for AudioData {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for AudioData {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for AudioData {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}

impl From<AudioData> for Any {
    fn from(s: AudioData) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&AudioData> for Any {
    fn from(s: &AudioData) -> Any {
        s.inner.clone().into()
    }
}

jsbind::utils::impl_dyn_cast!(AudioData);

impl AudioData {
    /// Getter of the `format` attribute.
    /// [`AudioData.format`](https://developer.mozilla.org/en-US/docs/Web/API/AudioData/format)
    pub fn format(&self) -> AudioSampleFormat {
        self.inner.get("format").as_::<AudioSampleFormat>()
    }
}
impl AudioData {
    /// Getter of the `sampleRate` attribute.
    /// [`AudioData.sampleRate`](https://developer.mozilla.org/en-US/docs/Web/API/AudioData/sampleRate)
    pub fn sample_rate(&self) -> f32 {
        self.inner.get("sampleRate").as_::<f32>()
    }
}
impl AudioData {
    /// Getter of the `numberOfFrames` attribute.
    /// [`AudioData.numberOfFrames`](https://developer.mozilla.org/en-US/docs/Web/API/AudioData/numberOfFrames)
    pub fn number_of_frames(&self) -> u32 {
        self.inner.get("numberOfFrames").as_::<u32>()
    }
}
impl AudioData {
    /// Getter of the `numberOfChannels` attribute.
    /// [`AudioData.numberOfChannels`](https://developer.mozilla.org/en-US/docs/Web/API/AudioData/numberOfChannels)
    pub fn number_of_channels(&self) -> u32 {
        self.inner.get("numberOfChannels").as_::<u32>()
    }
}
impl AudioData {
    /// Getter of the `duration` attribute.
    /// [`AudioData.duration`](https://developer.mozilla.org/en-US/docs/Web/API/AudioData/duration)
    pub fn duration(&self) -> u64 {
        self.inner.get("duration").as_::<u64>()
    }
}
impl AudioData {
    /// Getter of the `timestamp` attribute.
    /// [`AudioData.timestamp`](https://developer.mozilla.org/en-US/docs/Web/API/AudioData/timestamp)
    pub fn timestamp(&self) -> i64 {
        self.inner.get("timestamp").as_::<i64>()
    }
}

impl AudioData {
    /// The `new AudioData(..)` constructor, creating a new AudioData instance
    pub fn new(init: &AudioDataInit) -> AudioData {
        Self {
            inner: Any::global("AudioData").new(&[init.into()]).as_::<Any>(),
        }
    }
}

impl AudioData {
    /// The allocationSize method.
    /// [`AudioData.allocationSize`](https://developer.mozilla.org/en-US/docs/Web/API/AudioData/allocationSize)
    pub fn allocation_size(&self, options: &AudioDataCopyToOptions) -> u32 {
        self.inner
            .call("allocationSize", &[options.into()])
            .as_::<u32>()
    }
}
impl AudioData {
    /// The copyTo method.
    /// [`AudioData.copyTo`](https://developer.mozilla.org/en-US/docs/Web/API/AudioData/copyTo)
    pub fn copy_to(&self, destination: &Any, options: &AudioDataCopyToOptions) -> Undefined {
        self.inner
            .call("copyTo", &[destination.into(), options.into()])
            .as_::<Undefined>()
    }
}
impl AudioData {
    /// The clone method.
    /// [`AudioData.clone`](https://developer.mozilla.org/en-US/docs/Web/API/AudioData/clone)
    pub fn clone_(&self) -> AudioData {
        self.inner.call("clone", &[]).as_::<AudioData>()
    }
}
impl AudioData {
    /// The close method.
    /// [`AudioData.close`](https://developer.mozilla.org/en-US/docs/Web/API/AudioData/close)
    pub fn close(&self) -> Undefined {
        self.inner.call("close", &[]).as_::<Undefined>()
    }
}
