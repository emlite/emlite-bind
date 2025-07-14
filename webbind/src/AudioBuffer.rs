use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct AudioBuffer {
    inner: emlite::Val,
}
impl FromVal for AudioBuffer {
    fn from_val(v: &emlite::Val) -> Self {
        AudioBuffer {
            inner: emlite::Val::from_val(v),
        }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for AudioBuffer {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for AudioBuffer {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for AudioBuffer {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for AudioBuffer {
    fn as_mut(&mut self) -> &mut emlite::Val {
        &mut self.inner
    }
}
impl From<AudioBuffer> for emlite::Val {
    fn from(s: AudioBuffer) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}
jsbind::utils::impl_dyn_cast!(AudioBuffer);

impl AudioBuffer {
    pub fn new(options: jsbind::Any) -> AudioBuffer {
        Self {
            inner: emlite::Val::global("AudioBuffer")
                .new(&[options.into()])
                .as_::<emlite::Val>(),
        }
    }
}
impl AudioBuffer {
    pub fn sample_rate(&self) -> f32 {
        self.inner.get("sampleRate").as_::<f32>()
    }
}
impl AudioBuffer {
    pub fn length(&self) -> u32 {
        self.inner.get("length").as_::<u32>()
    }
}
impl AudioBuffer {
    pub fn duration(&self) -> f64 {
        self.inner.get("duration").as_::<f64>()
    }
}
impl AudioBuffer {
    pub fn number_of_channels(&self) -> u32 {
        self.inner.get("numberOfChannels").as_::<u32>()
    }
}
impl AudioBuffer {
    pub fn get_channel_data(&self, channel: u32) -> jsbind::Float32Array {
        self.inner
            .call("getChannelData", &[channel.into()])
            .as_::<jsbind::Float32Array>()
    }
}
impl AudioBuffer {
    pub fn copy_from_channel0(
        &self,
        destination: jsbind::Float32Array,
        channel_number: u32,
    ) -> jsbind::Undefined {
        self.inner
            .call(
                "copyFromChannel",
                &[destination.into(), channel_number.into()],
            )
            .as_::<jsbind::Undefined>()
    }

    pub fn copy_from_channel1(
        &self,
        destination: jsbind::Float32Array,
        channel_number: u32,
        buffer_offset: u32,
    ) -> jsbind::Undefined {
        self.inner
            .call(
                "copyFromChannel",
                &[
                    destination.into(),
                    channel_number.into(),
                    buffer_offset.into(),
                ],
            )
            .as_::<jsbind::Undefined>()
    }
}
impl AudioBuffer {
    pub fn copy_to_channel0(
        &self,
        source: jsbind::Float32Array,
        channel_number: u32,
    ) -> jsbind::Undefined {
        self.inner
            .call("copyToChannel", &[source.into(), channel_number.into()])
            .as_::<jsbind::Undefined>()
    }

    pub fn copy_to_channel1(
        &self,
        source: jsbind::Float32Array,
        channel_number: u32,
        buffer_offset: u32,
    ) -> jsbind::Undefined {
        self.inner
            .call(
                "copyToChannel",
                &[source.into(), channel_number.into(), buffer_offset.into()],
            )
            .as_::<jsbind::Undefined>()
    }
}
