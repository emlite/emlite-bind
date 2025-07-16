use super::*;

/// The WindowControlsOverlay class.
/// [`WindowControlsOverlay`](https://developer.mozilla.org/en-US/docs/Web/API/WindowControlsOverlay)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct WindowControlsOverlay {
    inner: EventTarget,
}
impl FromVal for WindowControlsOverlay {
    fn from_val(v: &Any) -> Self {
        WindowControlsOverlay {
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
impl core::ops::Deref for WindowControlsOverlay {
    type Target = EventTarget;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for WindowControlsOverlay {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for WindowControlsOverlay {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for WindowControlsOverlay {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<WindowControlsOverlay> for Any {
    fn from(s: WindowControlsOverlay) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&WindowControlsOverlay> for Any {
    fn from(s: &WindowControlsOverlay) -> Any {
        s.inner.clone().into()
    }
}
jsbind::utils::impl_dyn_cast!(WindowControlsOverlay);

impl WindowControlsOverlay {
    /// Getter of the `visible` attribute.
    /// [`WindowControlsOverlay.visible`](https://developer.mozilla.org/en-US/docs/Web/API/WindowControlsOverlay/visible)
    pub fn visible(&self) -> bool {
        self.inner.get("visible").as_::<bool>()
    }
}
impl WindowControlsOverlay {
    /// The getTitlebarAreaRect method.
    /// [`WindowControlsOverlay.getTitlebarAreaRect`](https://developer.mozilla.org/en-US/docs/Web/API/WindowControlsOverlay/getTitlebarAreaRect)
    pub fn get_titlebar_area_rect(&self) -> DOMRect {
        self.inner.call("getTitlebarAreaRect", &[]).as_::<DOMRect>()
    }
}
impl WindowControlsOverlay {
    /// Getter of the `ongeometrychange` attribute.
    /// [`WindowControlsOverlay.ongeometrychange`](https://developer.mozilla.org/en-US/docs/Web/API/WindowControlsOverlay/ongeometrychange)
    pub fn ongeometrychange(&self) -> Any {
        self.inner.get("ongeometrychange").as_::<Any>()
    }

    /// Setter of the `ongeometrychange` attribute.
    /// [`WindowControlsOverlay.ongeometrychange`](https://developer.mozilla.org/en-US/docs/Web/API/WindowControlsOverlay/ongeometrychange)
    pub fn set_ongeometrychange(&mut self, value: &Any) {
        self.inner.set("ongeometrychange", value);
    }
}
