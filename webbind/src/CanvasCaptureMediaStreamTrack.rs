use super::*;

/// The CanvasCaptureMediaStreamTrack class.
/// [`CanvasCaptureMediaStreamTrack`](https://developer.mozilla.org/en-US/docs/Web/API/CanvasCaptureMediaStreamTrack)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct CanvasCaptureMediaStreamTrack {
    inner: MediaStreamTrack,
}
impl FromVal for CanvasCaptureMediaStreamTrack {
    fn from_val(v: &Any) -> Self {
        CanvasCaptureMediaStreamTrack {
            inner: MediaStreamTrack::from_val(v),
        }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
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
impl AsRef<Any> for CanvasCaptureMediaStreamTrack {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for CanvasCaptureMediaStreamTrack {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<CanvasCaptureMediaStreamTrack> for Any {
    fn from(s: CanvasCaptureMediaStreamTrack) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&CanvasCaptureMediaStreamTrack> for Any {
    fn from(s: &CanvasCaptureMediaStreamTrack) -> Any {
        s.inner.clone().into()
    }
}
jsbind::utils::impl_dyn_cast!(CanvasCaptureMediaStreamTrack);

impl CanvasCaptureMediaStreamTrack {
    /// Getter of the `canvas` attribute.
    /// [`CanvasCaptureMediaStreamTrack.canvas`](https://developer.mozilla.org/en-US/docs/Web/API/CanvasCaptureMediaStreamTrack/canvas)
    pub fn canvas(&self) -> HTMLCanvasElement {
        self.inner.get("canvas").as_::<HTMLCanvasElement>()
    }
}
impl CanvasCaptureMediaStreamTrack {
    /// The requestFrame method.
    /// [`CanvasCaptureMediaStreamTrack.requestFrame`](https://developer.mozilla.org/en-US/docs/Web/API/CanvasCaptureMediaStreamTrack/requestFrame)
    pub fn request_frame(&self) -> Undefined {
        self.inner.call("requestFrame", &[]).as_::<Undefined>()
    }
}
