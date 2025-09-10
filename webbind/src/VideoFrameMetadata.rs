use super::*;

/// The VideoFrameMetadata dictionary.
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct VideoFrameMetadata {
    inner: Any,
}

impl FromVal for VideoFrameMetadata {
    fn from_val(v: &Any) -> Self {
        VideoFrameMetadata { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for VideoFrameMetadata {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for VideoFrameMetadata {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for VideoFrameMetadata {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for VideoFrameMetadata {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}

impl From<VideoFrameMetadata> for Any {
    fn from(s: VideoFrameMetadata) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&VideoFrameMetadata> for Any {
    fn from(s: &VideoFrameMetadata) -> Any {
        s.inner.clone()
    }
}
