use super::*;

/// The BrowserCaptureMediaStreamTrack class.
/// [`BrowserCaptureMediaStreamTrack`](https://developer.mozilla.org/en-US/docs/Web/API/BrowserCaptureMediaStreamTrack)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct BrowserCaptureMediaStreamTrack {
    inner: MediaStreamTrack,
}

impl FromVal for BrowserCaptureMediaStreamTrack {
    fn from_val(v: &Any) -> Self {
        BrowserCaptureMediaStreamTrack {
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

impl core::ops::Deref for BrowserCaptureMediaStreamTrack {
    type Target = MediaStreamTrack;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for BrowserCaptureMediaStreamTrack {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for BrowserCaptureMediaStreamTrack {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for BrowserCaptureMediaStreamTrack {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}

impl From<BrowserCaptureMediaStreamTrack> for Any {
    fn from(s: BrowserCaptureMediaStreamTrack) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&BrowserCaptureMediaStreamTrack> for Any {
    fn from(s: &BrowserCaptureMediaStreamTrack) -> Any {
        s.inner.clone().into()
    }
}

jsbind::utils::impl_dyn_cast!(BrowserCaptureMediaStreamTrack);

impl BrowserCaptureMediaStreamTrack {
    /// The cropTo method.
    /// [`BrowserCaptureMediaStreamTrack.cropTo`](https://developer.mozilla.org/en-US/docs/Web/API/BrowserCaptureMediaStreamTrack/cropTo)
    pub fn crop_to(&self, crop_target: &CropTarget) -> Promise<Undefined> {
        self.inner
            .call("cropTo", &[crop_target.into()])
            .as_::<Promise<Undefined>>()
    }
}
impl BrowserCaptureMediaStreamTrack {
    /// The clone method.
    /// [`BrowserCaptureMediaStreamTrack.clone`](https://developer.mozilla.org/en-US/docs/Web/API/BrowserCaptureMediaStreamTrack/clone)
    pub fn clone_(&self) -> BrowserCaptureMediaStreamTrack {
        self.inner
            .call("clone", &[])
            .as_::<BrowserCaptureMediaStreamTrack>()
    }
}
impl BrowserCaptureMediaStreamTrack {
    /// The restrictTo method.
    /// [`BrowserCaptureMediaStreamTrack.restrictTo`](https://developer.mozilla.org/en-US/docs/Web/API/BrowserCaptureMediaStreamTrack/restrictTo)
    pub fn restrict_to(&self, restriction_target: &RestrictionTarget) -> Promise<Undefined> {
        self.inner
            .call("restrictTo", &[restriction_target.into()])
            .as_::<Promise<Undefined>>()
    }
}
