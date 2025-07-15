use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct CaptureController {
    inner: EventTarget,
}
impl FromVal for CaptureController {
    fn from_val(v: &emlite::Val) -> Self {
        CaptureController {
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
impl core::ops::Deref for CaptureController {
    type Target = EventTarget;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for CaptureController {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for CaptureController {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for CaptureController {
    fn as_mut(&mut self) -> &mut emlite::Val {
        &mut self.inner
    }
}
impl From<CaptureController> for emlite::Val {
    fn from(s: CaptureController) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}
impl From<&CaptureController> for emlite::Val {
    fn from(s: &CaptureController) -> emlite::Val {
        s.inner.clone().into()
    }
}
jsbind::utils::impl_dyn_cast!(CaptureController);

impl CaptureController {
    pub fn new() -> CaptureController {
        Self {
            inner: emlite::Val::global("CaptureController")
                .new(&[])
                .as_::<EventTarget>(),
        }
    }
}
impl CaptureController {
    pub fn set_focus_behavior(&self, focus_behavior: &CaptureStartFocusBehavior) -> Undefined {
        self.inner
            .call("setFocusBehavior", &[focus_behavior.into()])
            .as_::<Undefined>()
    }
}
impl CaptureController {
    pub fn oncapturedmousechange(&self) -> Any {
        self.inner.get("oncapturedmousechange").as_::<Any>()
    }

    pub fn set_oncapturedmousechange(&mut self, value: &Any) {
        self.inner.set("oncapturedmousechange", value);
    }
}
impl CaptureController {
    pub fn get_supported_zoom_levels(&self) -> Sequence<i32> {
        self.inner
            .call("getSupportedZoomLevels", &[])
            .as_::<Sequence<i32>>()
    }
}
impl CaptureController {
    pub fn zoom_level(&self) -> i32 {
        self.inner.get("zoomLevel").as_::<i32>()
    }
}
impl CaptureController {
    pub fn increase_zoom_level(&self) -> Promise {
        self.inner.call("increaseZoomLevel", &[]).as_::<Promise>()
    }
}
impl CaptureController {
    pub fn decrease_zoom_level(&self) -> Promise {
        self.inner.call("decreaseZoomLevel", &[]).as_::<Promise>()
    }
}
impl CaptureController {
    pub fn reset_zoom_level(&self) -> Promise {
        self.inner.call("resetZoomLevel", &[]).as_::<Promise>()
    }
}
impl CaptureController {
    pub fn onzoomlevelchange(&self) -> Any {
        self.inner.get("onzoomlevelchange").as_::<Any>()
    }

    pub fn set_onzoomlevelchange(&mut self, value: &Any) {
        self.inner.set("onzoomlevelchange", value);
    }
}
impl CaptureController {
    pub fn forward_wheel(&self, element: &HTMLElement) -> Promise {
        self.inner
            .call("forwardWheel", &[element.into()])
            .as_::<Promise>()
    }
}
