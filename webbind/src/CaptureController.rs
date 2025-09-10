use super::*;

/// The CaptureController class.
/// [`CaptureController`](https://developer.mozilla.org/en-US/docs/Web/API/CaptureController)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct CaptureController {
    inner: EventTarget,
}

impl FromVal for CaptureController {
    fn from_val(v: &Any) -> Self {
        CaptureController {
            inner: EventTarget::from_val(v),
        }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
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

impl AsRef<Any> for CaptureController {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for CaptureController {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}

impl From<CaptureController> for Any {
    fn from(s: CaptureController) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&CaptureController> for Any {
    fn from(s: &CaptureController) -> Any {
        s.inner.clone().into()
    }
}

jsbind::utils::impl_dyn_cast!(CaptureController);

impl CaptureController {
    /// The `new CaptureController(..)` constructor, creating a new CaptureController instance
    pub fn new() -> CaptureController {
        Self {
            inner: Any::global("CaptureController")
                .new(&[])
                .as_::<EventTarget>(),
        }
    }
}
impl CaptureController {
    /// The setFocusBehavior method.
    /// [`CaptureController.setFocusBehavior`](https://developer.mozilla.org/en-US/docs/Web/API/CaptureController/setFocusBehavior)
    pub fn set_focus_behavior(&self, focus_behavior: &CaptureStartFocusBehavior) -> Undefined {
        self.inner
            .call("setFocusBehavior", &[focus_behavior.into()])
            .as_::<Undefined>()
    }
}
impl CaptureController {
    /// Getter of the `oncapturedmousechange` attribute.
    /// [`CaptureController.oncapturedmousechange`](https://developer.mozilla.org/en-US/docs/Web/API/CaptureController/oncapturedmousechange)
    pub fn oncapturedmousechange(&self) -> Any {
        self.inner.get("oncapturedmousechange").as_::<Any>()
    }

    /// Setter of the `oncapturedmousechange` attribute.
    /// [`CaptureController.oncapturedmousechange`](https://developer.mozilla.org/en-US/docs/Web/API/CaptureController/oncapturedmousechange)
    pub fn set_oncapturedmousechange(&mut self, value: &Any) {
        self.inner.set("oncapturedmousechange", value);
    }
}
impl CaptureController {
    /// The getSupportedZoomLevels method.
    /// [`CaptureController.getSupportedZoomLevels`](https://developer.mozilla.org/en-US/docs/Web/API/CaptureController/getSupportedZoomLevels)
    pub fn get_supported_zoom_levels(&self) -> TypedArray<i32> {
        self.inner
            .call("getSupportedZoomLevels", &[])
            .as_::<TypedArray<i32>>()
    }
}
impl CaptureController {
    /// Getter of the `zoomLevel` attribute.
    /// [`CaptureController.zoomLevel`](https://developer.mozilla.org/en-US/docs/Web/API/CaptureController/zoomLevel)
    pub fn zoom_level(&self) -> i32 {
        self.inner.get("zoomLevel").as_::<i32>()
    }
}
impl CaptureController {
    /// The increaseZoomLevel method.
    /// [`CaptureController.increaseZoomLevel`](https://developer.mozilla.org/en-US/docs/Web/API/CaptureController/increaseZoomLevel)
    pub fn increase_zoom_level(&self) -> Promise<Undefined> {
        self.inner
            .call("increaseZoomLevel", &[])
            .as_::<Promise<Undefined>>()
    }
}
impl CaptureController {
    /// The decreaseZoomLevel method.
    /// [`CaptureController.decreaseZoomLevel`](https://developer.mozilla.org/en-US/docs/Web/API/CaptureController/decreaseZoomLevel)
    pub fn decrease_zoom_level(&self) -> Promise<Undefined> {
        self.inner
            .call("decreaseZoomLevel", &[])
            .as_::<Promise<Undefined>>()
    }
}
impl CaptureController {
    /// The resetZoomLevel method.
    /// [`CaptureController.resetZoomLevel`](https://developer.mozilla.org/en-US/docs/Web/API/CaptureController/resetZoomLevel)
    pub fn reset_zoom_level(&self) -> Promise<Undefined> {
        self.inner
            .call("resetZoomLevel", &[])
            .as_::<Promise<Undefined>>()
    }
}
impl CaptureController {
    /// Getter of the `onzoomlevelchange` attribute.
    /// [`CaptureController.onzoomlevelchange`](https://developer.mozilla.org/en-US/docs/Web/API/CaptureController/onzoomlevelchange)
    pub fn onzoomlevelchange(&self) -> Any {
        self.inner.get("onzoomlevelchange").as_::<Any>()
    }

    /// Setter of the `onzoomlevelchange` attribute.
    /// [`CaptureController.onzoomlevelchange`](https://developer.mozilla.org/en-US/docs/Web/API/CaptureController/onzoomlevelchange)
    pub fn set_onzoomlevelchange(&mut self, value: &Any) {
        self.inner.set("onzoomlevelchange", value);
    }
}
impl CaptureController {
    /// The forwardWheel method.
    /// [`CaptureController.forwardWheel`](https://developer.mozilla.org/en-US/docs/Web/API/CaptureController/forwardWheel)
    pub fn forward_wheel(&self, element: &HTMLElement) -> Promise<Undefined> {
        self.inner
            .call("forwardWheel", &[element.into()])
            .as_::<Promise<Undefined>>()
    }
}
