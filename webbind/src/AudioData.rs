use super::*;

#[derive(Clone, Debug)]
pub struct AudioDataCopyToOptions {
    inner: emlite::Val,
}
impl FromVal for AudioDataCopyToOptions {
    fn from_val(v: &emlite::Val) -> Self {
        AudioDataCopyToOptions { inner: v.clone() }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl std::ops::Deref for AudioDataCopyToOptions {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for AudioDataCopyToOptions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<AudioDataCopyToOptions> for emlite::Val {
    fn from(s: AudioDataCopyToOptions) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl AudioDataCopyToOptions {
    pub fn plane_index(&self) -> u32 {
        self.inner.get("planeIndex").as_::<u32>()
    }

    pub fn set_plane_index(&mut self, value: u32) {
        self.inner.set("planeIndex", value);
    }
}
impl AudioDataCopyToOptions {
    pub fn frame_offset(&self) -> u32 {
        self.inner.get("frameOffset").as_::<u32>()
    }

    pub fn set_frame_offset(&mut self, value: u32) {
        self.inner.set("frameOffset", value);
    }
}
impl AudioDataCopyToOptions {
    pub fn frame_count(&self) -> u32 {
        self.inner.get("frameCount").as_::<u32>()
    }

    pub fn set_frame_count(&mut self, value: u32) {
        self.inner.set("frameCount", value);
    }
}
impl AudioDataCopyToOptions {
    pub fn format(&self) -> AudioSampleFormat {
        self.inner.get("format").as_::<AudioSampleFormat>()
    }

    pub fn set_format(&mut self, value: AudioSampleFormat) {
        self.inner.set("format", value);
    }
}
#[derive(Clone, Debug)]
pub struct AudioData {
    inner: emlite::Val,
}
impl FromVal for AudioData {
    fn from_val(v: &emlite::Val) -> Self {
        AudioData {
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
impl std::ops::Deref for AudioData {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for AudioData {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<AudioData> for emlite::Val {
    fn from(s: AudioData) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl AudioData {
    pub fn new(init: jsbind::Any) -> AudioData {
        Self {
            inner: emlite::Val::global("AudioData")
                .new(&[init.into()])
                .as_::<emlite::Val>(),
        }
    }
}
impl AudioData {
    pub fn format(&self) -> AudioSampleFormat {
        self.inner.get("format").as_::<AudioSampleFormat>()
    }
}
impl AudioData {
    pub fn sample_rate(&self) -> f32 {
        self.inner.get("sampleRate").as_::<f32>()
    }
}
impl AudioData {
    pub fn number_of_frames(&self) -> u32 {
        self.inner.get("numberOfFrames").as_::<u32>()
    }
}
impl AudioData {
    pub fn number_of_channels(&self) -> u32 {
        self.inner.get("numberOfChannels").as_::<u32>()
    }
}
impl AudioData {
    pub fn duration(&self) -> u64 {
        self.inner.get("duration").as_::<u64>()
    }
}
impl AudioData {
    pub fn timestamp(&self) -> i64 {
        self.inner.get("timestamp").as_::<i64>()
    }
}
impl AudioData {
    pub fn allocation_size(&self, options: AudioDataCopyToOptions) -> u32 {
        self.inner
            .call("allocationSize", &[options.into()])
            .as_::<u32>()
    }
}
impl AudioData {
    pub fn copy_to(
        &self,
        destination: jsbind::Any,
        options: AudioDataCopyToOptions,
    ) -> jsbind::Undefined {
        self.inner
            .call("copyTo", &[destination.into(), options.into()])
            .as_::<jsbind::Undefined>()
    }
}
impl AudioData {
    pub fn clone_(&self) -> AudioData {
        self.inner.call("clone", &[]).as_::<AudioData>()
    }
}
impl AudioData {
    pub fn close(&self) -> jsbind::Undefined {
        self.inner.call("close", &[]).as_::<jsbind::Undefined>()
    }
}
