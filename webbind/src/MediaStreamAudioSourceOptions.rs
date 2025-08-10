use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct MediaStreamAudioSourceOptions {
    inner: Any,
}
impl FromVal for MediaStreamAudioSourceOptions {
    fn from_val(v: &Any) -> Self {
        MediaStreamAudioSourceOptions { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for MediaStreamAudioSourceOptions {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for MediaStreamAudioSourceOptions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for MediaStreamAudioSourceOptions {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for MediaStreamAudioSourceOptions {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<MediaStreamAudioSourceOptions> for Any {
    fn from(s: MediaStreamAudioSourceOptions) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&MediaStreamAudioSourceOptions> for Any {
    fn from(s: &MediaStreamAudioSourceOptions) -> Any {
        s.inner.clone()
    }
}

impl MediaStreamAudioSourceOptions {
    pub fn media_stream(&self) -> MediaStream {
        self.inner.get("mediaStream").as_::<MediaStream>()
    }

    pub fn set_media_stream(&mut self, value: &MediaStream) {
        self.inner.set("mediaStream", value);
    }
}
