use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct RTCEncodedAudioFrameMetadata {
    inner: emlite::Val,
}
impl FromVal for RTCEncodedAudioFrameMetadata {
    fn from_val(v: &emlite::Val) -> Self {
        RTCEncodedAudioFrameMetadata { inner: v.clone() }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for RTCEncodedAudioFrameMetadata {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for RTCEncodedAudioFrameMetadata {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for RTCEncodedAudioFrameMetadata {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for RTCEncodedAudioFrameMetadata {
    fn as_mut(&mut self) -> &mut emlite::Val {
        &mut self.inner
    }
}
impl From<RTCEncodedAudioFrameMetadata> for emlite::Val {
    fn from(s: RTCEncodedAudioFrameMetadata) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}
impl From<&RTCEncodedAudioFrameMetadata> for emlite::Val {
    fn from(s: &RTCEncodedAudioFrameMetadata) -> emlite::Val {
        s.inner.clone()
    }
}

impl RTCEncodedAudioFrameMetadata {
    pub fn sequence_number(&self) -> i16 {
        self.inner.get("sequenceNumber").as_::<i16>()
    }

    pub fn set_sequence_number(&mut self, value: i16) {
        self.inner.set("sequenceNumber", value);
    }
}
impl RTCEncodedAudioFrameMetadata {
    pub fn audio_level(&self) -> f64 {
        self.inner.get("audioLevel").as_::<f64>()
    }

    pub fn set_audio_level(&mut self, value: f64) {
        self.inner.set("audioLevel", value);
    }
}
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct RTCEncodedAudioFrame {
    inner: emlite::Val,
}
impl FromVal for RTCEncodedAudioFrame {
    fn from_val(v: &emlite::Val) -> Self {
        RTCEncodedAudioFrame {
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
impl core::ops::Deref for RTCEncodedAudioFrame {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for RTCEncodedAudioFrame {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for RTCEncodedAudioFrame {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for RTCEncodedAudioFrame {
    fn as_mut(&mut self) -> &mut emlite::Val {
        &mut self.inner
    }
}
impl From<RTCEncodedAudioFrame> for emlite::Val {
    fn from(s: RTCEncodedAudioFrame) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}
impl From<&RTCEncodedAudioFrame> for emlite::Val {
    fn from(s: &RTCEncodedAudioFrame) -> emlite::Val {
        s.inner.clone().into()
    }
}
jsbind::utils::impl_dyn_cast!(RTCEncodedAudioFrame);

impl RTCEncodedAudioFrame {
    pub fn new0(original_frame: RTCEncodedAudioFrame) -> RTCEncodedAudioFrame {
        Self {
            inner: emlite::Val::global("RTCEncodedAudioFrame")
                .new(&[original_frame.into()])
                .as_::<emlite::Val>(),
        }
    }

    pub fn new1(original_frame: RTCEncodedAudioFrame, options: Any) -> RTCEncodedAudioFrame {
        Self {
            inner: emlite::Val::global("RTCEncodedAudioFrame")
                .new(&[original_frame.into(), options.into()])
                .as_::<emlite::Val>(),
        }
    }
}
impl RTCEncodedAudioFrame {
    pub fn data(&self) -> ArrayBuffer {
        self.inner.get("data").as_::<ArrayBuffer>()
    }

    pub fn set_data(&mut self, value: ArrayBuffer) {
        self.inner.set("data", value);
    }
}
impl RTCEncodedAudioFrame {
    pub fn get_metadata(&self) -> RTCEncodedAudioFrameMetadata {
        self.inner
            .call("getMetadata", &[])
            .as_::<RTCEncodedAudioFrameMetadata>()
    }
}
