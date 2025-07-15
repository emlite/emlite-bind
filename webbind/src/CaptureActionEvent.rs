use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct CaptureActionEvent {
    inner: Event,
}
impl FromVal for CaptureActionEvent {
    fn from_val(v: &emlite::Val) -> Self {
        CaptureActionEvent {
            inner: Event::from_val(v),
        }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for CaptureActionEvent {
    type Target = Event;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for CaptureActionEvent {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for CaptureActionEvent {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for CaptureActionEvent {
    fn as_mut(&mut self) -> &mut emlite::Val {
        &mut self.inner
    }
}
impl From<CaptureActionEvent> for emlite::Val {
    fn from(s: CaptureActionEvent) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}
jsbind::utils::impl_dyn_cast!(CaptureActionEvent);

impl CaptureActionEvent {
    pub fn new0() -> CaptureActionEvent {
        Self {
            inner: emlite::Val::global("CaptureActionEvent")
                .new(&[])
                .as_::<Event>(),
        }
    }

    pub fn new1(init: Any) -> CaptureActionEvent {
        Self {
            inner: emlite::Val::global("CaptureActionEvent")
                .new(&[init.into()])
                .as_::<Event>(),
        }
    }
}
impl CaptureActionEvent {
    pub fn action(&self) -> CaptureAction {
        self.inner.get("action").as_::<CaptureAction>()
    }
}
