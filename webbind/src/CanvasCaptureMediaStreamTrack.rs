use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
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
impl core::ops::Deref for CanvasCaptureMediaStreamTrack {
    type Target = MediaStreamTrack;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for CanvasCaptureMediaStreamTrack {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for CanvasCaptureMediaStreamTrack {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for CanvasCaptureMediaStreamTrack {
    fn as_mut(&mut self) -> &mut emlite::Val {
        &mut self.inner
    }
}
impl From<CanvasCaptureMediaStreamTrack> for emlite::Val {
    fn from(s: CanvasCaptureMediaStreamTrack) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}
impl From<&CanvasCaptureMediaStreamTrack> for emlite::Val {
    fn from(s: &CanvasCaptureMediaStreamTrack) -> emlite::Val {
        s.inner.clone().into()
    }
}
jsbind::utils::impl_dyn_cast!(CanvasCaptureMediaStreamTrack);

impl CanvasCaptureMediaStreamTrack {
    pub fn canvas(&self) -> HTMLCanvasElement {
        self.inner.get("canvas").as_::<HTMLCanvasElement>()
    }
}
impl CanvasCaptureMediaStreamTrack {
    pub fn request_frame(&self) -> Undefined {
        self.inner.call("requestFrame", &[]).as_::<Undefined>()
    }
}
