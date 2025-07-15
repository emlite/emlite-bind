use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct RTCEncodedVideoFrameMetadata {
    inner: emlite::Val,
}
impl FromVal for RTCEncodedVideoFrameMetadata {
    fn from_val(v: &emlite::Val) -> Self {
        RTCEncodedVideoFrameMetadata { inner: v.clone() }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for RTCEncodedVideoFrameMetadata {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for RTCEncodedVideoFrameMetadata {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for RTCEncodedVideoFrameMetadata {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for RTCEncodedVideoFrameMetadata {
    fn as_mut(&mut self) -> &mut emlite::Val {
        &mut self.inner
    }
}
impl From<RTCEncodedVideoFrameMetadata> for emlite::Val {
    fn from(s: RTCEncodedVideoFrameMetadata) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}
impl From<&RTCEncodedVideoFrameMetadata> for emlite::Val {
    fn from(s: &RTCEncodedVideoFrameMetadata) -> emlite::Val {
        s.inner.clone()
    }
}

impl RTCEncodedVideoFrameMetadata {
    pub fn frame_id(&self) -> u64 {
        self.inner.get("frameId").as_::<u64>()
    }

    pub fn set_frame_id(&mut self, value: u64) {
        self.inner.set("frameId", value);
    }
}
impl RTCEncodedVideoFrameMetadata {
    pub fn dependencies(&self) -> Sequence<u64> {
        self.inner.get("dependencies").as_::<Sequence<u64>>()
    }

    pub fn set_dependencies(&mut self, value: Sequence<u64>) {
        self.inner.set("dependencies", value);
    }
}
impl RTCEncodedVideoFrameMetadata {
    pub fn width(&self) -> u16 {
        self.inner.get("width").as_::<u16>()
    }

    pub fn set_width(&mut self, value: u16) {
        self.inner.set("width", value);
    }
}
impl RTCEncodedVideoFrameMetadata {
    pub fn height(&self) -> u16 {
        self.inner.get("height").as_::<u16>()
    }

    pub fn set_height(&mut self, value: u16) {
        self.inner.set("height", value);
    }
}
impl RTCEncodedVideoFrameMetadata {
    pub fn spatial_index(&self) -> u32 {
        self.inner.get("spatialIndex").as_::<u32>()
    }

    pub fn set_spatial_index(&mut self, value: u32) {
        self.inner.set("spatialIndex", value);
    }
}
impl RTCEncodedVideoFrameMetadata {
    pub fn temporal_index(&self) -> u32 {
        self.inner.get("temporalIndex").as_::<u32>()
    }

    pub fn set_temporal_index(&mut self, value: u32) {
        self.inner.set("temporalIndex", value);
    }
}
impl RTCEncodedVideoFrameMetadata {
    pub fn timestamp(&self) -> i64 {
        self.inner.get("timestamp").as_::<i64>()
    }

    pub fn set_timestamp(&mut self, value: i64) {
        self.inner.set("timestamp", value);
    }
}
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct RTCEncodedVideoFrame {
    inner: emlite::Val,
}
impl FromVal for RTCEncodedVideoFrame {
    fn from_val(v: &emlite::Val) -> Self {
        RTCEncodedVideoFrame {
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
impl core::ops::Deref for RTCEncodedVideoFrame {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for RTCEncodedVideoFrame {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for RTCEncodedVideoFrame {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for RTCEncodedVideoFrame {
    fn as_mut(&mut self) -> &mut emlite::Val {
        &mut self.inner
    }
}
impl From<RTCEncodedVideoFrame> for emlite::Val {
    fn from(s: RTCEncodedVideoFrame) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}
impl From<&RTCEncodedVideoFrame> for emlite::Val {
    fn from(s: &RTCEncodedVideoFrame) -> emlite::Val {
        s.inner.clone().into()
    }
}
jsbind::utils::impl_dyn_cast!(RTCEncodedVideoFrame);

impl RTCEncodedVideoFrame {
    pub fn new0(original_frame: RTCEncodedVideoFrame) -> RTCEncodedVideoFrame {
        Self {
            inner: emlite::Val::global("RTCEncodedVideoFrame")
                .new(&[original_frame.into()])
                .as_::<emlite::Val>(),
        }
    }

    pub fn new1(original_frame: RTCEncodedVideoFrame, options: Any) -> RTCEncodedVideoFrame {
        Self {
            inner: emlite::Val::global("RTCEncodedVideoFrame")
                .new(&[original_frame.into(), options.into()])
                .as_::<emlite::Val>(),
        }
    }
}
impl RTCEncodedVideoFrame {
    pub fn type_(&self) -> RTCEncodedVideoFrameType {
        self.inner.get("type").as_::<RTCEncodedVideoFrameType>()
    }
}
impl RTCEncodedVideoFrame {
    pub fn data(&self) -> ArrayBuffer {
        self.inner.get("data").as_::<ArrayBuffer>()
    }

    pub fn set_data(&mut self, value: ArrayBuffer) {
        self.inner.set("data", value);
    }
}
impl RTCEncodedVideoFrame {
    pub fn get_metadata(&self) -> RTCEncodedVideoFrameMetadata {
        self.inner
            .call("getMetadata", &[])
            .as_::<RTCEncodedVideoFrameMetadata>()
    }
}
