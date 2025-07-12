use super::*;

#[derive(Clone, Debug)]
pub struct CanvasCaptureMediaStreamTrack {
    inner: MediaStreamTrack,
}
impl FromVal for CanvasCaptureMediaStreamTrack {
    fn from_val(v: &emlite::Val) -> Self {
        CanvasCaptureMediaStreamTrack {
            inner: MediaStreamTrack::from_val(v),
        }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl std::ops::Deref for CanvasCaptureMediaStreamTrack {
    type Target = MediaStreamTrack;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for CanvasCaptureMediaStreamTrack {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<CanvasCaptureMediaStreamTrack> for emlite::Val {
    fn from(s: CanvasCaptureMediaStreamTrack) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl CanvasCaptureMediaStreamTrack {
    pub fn canvas(&self) -> HTMLCanvasElement {
        self.inner.get("canvas").as_::<HTMLCanvasElement>()
    }
}
impl CanvasCaptureMediaStreamTrack {
    pub fn request_frame(&self) -> jsbind::Undefined {
        self.inner
            .call("requestFrame", &[])
            .as_::<jsbind::Undefined>()
    }
}
