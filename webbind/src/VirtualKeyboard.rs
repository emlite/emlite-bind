use super::*;

/// The VirtualKeyboard class.
/// [`VirtualKeyboard`](https://developer.mozilla.org/en-US/docs/Web/API/VirtualKeyboard)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct VirtualKeyboard {
    inner: EventTarget,
}
impl FromVal for VirtualKeyboard {
    fn from_val(v: &Any) -> Self {
        VirtualKeyboard {
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
impl core::ops::Deref for VirtualKeyboard {
    type Target = EventTarget;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for VirtualKeyboard {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for VirtualKeyboard {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for VirtualKeyboard {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<VirtualKeyboard> for Any {
    fn from(s: VirtualKeyboard) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&VirtualKeyboard> for Any {
    fn from(s: &VirtualKeyboard) -> Any {
        s.inner.clone().into()
    }
}
jsbind::utils::impl_dyn_cast!(VirtualKeyboard);

impl VirtualKeyboard {
    /// The show method.
    /// [`VirtualKeyboard.show`](https://developer.mozilla.org/en-US/docs/Web/API/VirtualKeyboard/show)
    pub fn show(&self) -> Undefined {
        self.inner.call("show", &[]).as_::<Undefined>()
    }
}
impl VirtualKeyboard {
    /// The hide method.
    /// [`VirtualKeyboard.hide`](https://developer.mozilla.org/en-US/docs/Web/API/VirtualKeyboard/hide)
    pub fn hide(&self) -> Undefined {
        self.inner.call("hide", &[]).as_::<Undefined>()
    }
}
impl VirtualKeyboard {
    /// Getter of the `boundingRect` attribute.
    /// [`VirtualKeyboard.boundingRect`](https://developer.mozilla.org/en-US/docs/Web/API/VirtualKeyboard/boundingRect)
    pub fn bounding_rect(&self) -> DOMRect {
        self.inner.get("boundingRect").as_::<DOMRect>()
    }
}
impl VirtualKeyboard {
    /// Getter of the `overlaysContent` attribute.
    /// [`VirtualKeyboard.overlaysContent`](https://developer.mozilla.org/en-US/docs/Web/API/VirtualKeyboard/overlaysContent)
    pub fn overlays_content(&self) -> bool {
        self.inner.get("overlaysContent").as_::<bool>()
    }

    /// Setter of the `overlaysContent` attribute.
    /// [`VirtualKeyboard.overlaysContent`](https://developer.mozilla.org/en-US/docs/Web/API/VirtualKeyboard/overlaysContent)
    pub fn set_overlays_content(&mut self, value: bool) {
        self.inner.set("overlaysContent", value);
    }
}
impl VirtualKeyboard {
    /// Getter of the `ongeometrychange` attribute.
    /// [`VirtualKeyboard.ongeometrychange`](https://developer.mozilla.org/en-US/docs/Web/API/VirtualKeyboard/ongeometrychange)
    pub fn ongeometrychange(&self) -> Any {
        self.inner.get("ongeometrychange").as_::<Any>()
    }

    /// Setter of the `ongeometrychange` attribute.
    /// [`VirtualKeyboard.ongeometrychange`](https://developer.mozilla.org/en-US/docs/Web/API/VirtualKeyboard/ongeometrychange)
    pub fn set_ongeometrychange(&mut self, value: &Any) {
        self.inner.set("ongeometrychange", value);
    }
}
