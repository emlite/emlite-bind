use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct BrowserCaptureMediaStreamTrack {
    inner: MediaStreamTrack,
}
impl FromVal for BrowserCaptureMediaStreamTrack {
    fn from_val(v: &emlite::Val) -> Self {
        BrowserCaptureMediaStreamTrack {
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
impl AsRef<emlite::Val> for BrowserCaptureMediaStreamTrack {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for BrowserCaptureMediaStreamTrack {
    fn as_mut(&mut self) -> &mut emlite::Val {
        &mut self.inner
    }
}
impl From<BrowserCaptureMediaStreamTrack> for emlite::Val {
    fn from(s: BrowserCaptureMediaStreamTrack) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}
impl From<&BrowserCaptureMediaStreamTrack> for emlite::Val {
    fn from(s: &BrowserCaptureMediaStreamTrack) -> emlite::Val {
        s.inner.clone().into()
    }
}
jsbind::utils::impl_dyn_cast!(BrowserCaptureMediaStreamTrack);

impl BrowserCaptureMediaStreamTrack {
    pub fn crop_to(&self, crop_target: CropTarget) -> Promise {
        self.inner
            .call("cropTo", &[crop_target.into()])
            .as_::<Promise>()
    }
}
impl BrowserCaptureMediaStreamTrack {
    pub fn clone_(&self) -> BrowserCaptureMediaStreamTrack {
        self.inner
            .call("clone", &[])
            .as_::<BrowserCaptureMediaStreamTrack>()
    }
}
impl BrowserCaptureMediaStreamTrack {
    pub fn restrict_to(&self, restriction_target: RestrictionTarget) -> Promise {
        self.inner
            .call("restrictTo", &[restriction_target.into()])
            .as_::<Promise>()
    }
}
