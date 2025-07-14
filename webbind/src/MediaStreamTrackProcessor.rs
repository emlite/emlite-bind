use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct MediaStreamTrackProcessor {
    inner: emlite::Val,
}
impl FromVal for MediaStreamTrackProcessor {
    fn from_val(v: &emlite::Val) -> Self {
        MediaStreamTrackProcessor {
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
impl core::ops::Deref for MediaStreamTrackProcessor {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for MediaStreamTrackProcessor {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<MediaStreamTrackProcessor> for emlite::Val {
    fn from(s: MediaStreamTrackProcessor) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl MediaStreamTrackProcessor {
    pub fn new(init: jsbind::Any) -> MediaStreamTrackProcessor {
        Self {
            inner: emlite::Val::global("MediaStreamTrackProcessor")
                .new(&[init.into()])
                .as_::<emlite::Val>(),
        }
    }
}
impl MediaStreamTrackProcessor {
    pub fn readable(&self) -> ReadableStream {
        self.inner.get("readable").as_::<ReadableStream>()
    }
}
