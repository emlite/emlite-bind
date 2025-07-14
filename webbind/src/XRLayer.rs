use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct XRLayer {
    inner: EventTarget,
}
impl FromVal for XRLayer {
    fn from_val(v: &emlite::Val) -> Self {
        XRLayer {
            inner: EventTarget::from_val(v),
        }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for XRLayer {
    type Target = EventTarget;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for XRLayer {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<XRLayer> for emlite::Val {
    fn from(s: XRLayer) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}
