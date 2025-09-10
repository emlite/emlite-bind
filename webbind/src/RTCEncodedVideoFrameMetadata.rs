use super::*;

/// The RTCEncodedVideoFrameMetadata dictionary.
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct RTCEncodedVideoFrameMetadata {
    inner: Any,
}

impl FromVal for RTCEncodedVideoFrameMetadata {
    fn from_val(v: &Any) -> Self {
        RTCEncodedVideoFrameMetadata { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for RTCEncodedVideoFrameMetadata {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for RTCEncodedVideoFrameMetadata {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for RTCEncodedVideoFrameMetadata {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for RTCEncodedVideoFrameMetadata {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}

impl From<RTCEncodedVideoFrameMetadata> for Any {
    fn from(s: RTCEncodedVideoFrameMetadata) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&RTCEncodedVideoFrameMetadata> for Any {
    fn from(s: &RTCEncodedVideoFrameMetadata) -> Any {
        s.inner.clone()
    }
}

impl RTCEncodedVideoFrameMetadata {
    /// Getter of the `frameId` attribute.
    pub fn frame_id(&self) -> u64 {
        self.inner.get("frameId").as_::<u64>()
    }

    /// Setter of the `frameId` attribute.
    pub fn set_frame_id(&mut self, value: u64) {
        self.inner.set("frameId", value);
    }
}
impl RTCEncodedVideoFrameMetadata {
    /// Getter of the `dependencies` attribute.
    pub fn dependencies(&self) -> TypedArray<u64> {
        self.inner.get("dependencies").as_::<TypedArray<u64>>()
    }

    /// Setter of the `dependencies` attribute.
    pub fn set_dependencies(&mut self, value: TypedArray<u64>) {
        self.inner.set("dependencies", value);
    }
}
impl RTCEncodedVideoFrameMetadata {
    /// Getter of the `width` attribute.
    pub fn width(&self) -> u16 {
        self.inner.get("width").as_::<u16>()
    }

    /// Setter of the `width` attribute.
    pub fn set_width(&mut self, value: u16) {
        self.inner.set("width", value);
    }
}
impl RTCEncodedVideoFrameMetadata {
    /// Getter of the `height` attribute.
    pub fn height(&self) -> u16 {
        self.inner.get("height").as_::<u16>()
    }

    /// Setter of the `height` attribute.
    pub fn set_height(&mut self, value: u16) {
        self.inner.set("height", value);
    }
}
impl RTCEncodedVideoFrameMetadata {
    /// Getter of the `spatialIndex` attribute.
    pub fn spatial_index(&self) -> u32 {
        self.inner.get("spatialIndex").as_::<u32>()
    }

    /// Setter of the `spatialIndex` attribute.
    pub fn set_spatial_index(&mut self, value: u32) {
        self.inner.set("spatialIndex", value);
    }
}
impl RTCEncodedVideoFrameMetadata {
    /// Getter of the `temporalIndex` attribute.
    pub fn temporal_index(&self) -> u32 {
        self.inner.get("temporalIndex").as_::<u32>()
    }

    /// Setter of the `temporalIndex` attribute.
    pub fn set_temporal_index(&mut self, value: u32) {
        self.inner.set("temporalIndex", value);
    }
}
impl RTCEncodedVideoFrameMetadata {
    /// Getter of the `timestamp` attribute.
    pub fn timestamp(&self) -> i64 {
        self.inner.get("timestamp").as_::<i64>()
    }

    /// Setter of the `timestamp` attribute.
    pub fn set_timestamp(&mut self, value: i64) {
        self.inner.set("timestamp", value);
    }
}
