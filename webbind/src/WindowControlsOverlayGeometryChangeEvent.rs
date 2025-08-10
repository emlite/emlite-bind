use super::*;

/// The WindowControlsOverlayGeometryChangeEvent class.
/// [`WindowControlsOverlayGeometryChangeEvent`](https://developer.mozilla.org/en-US/docs/Web/API/WindowControlsOverlayGeometryChangeEvent)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct WindowControlsOverlayGeometryChangeEvent {
    inner: Event,
}
impl FromVal for WindowControlsOverlayGeometryChangeEvent {
    fn from_val(v: &Any) -> Self {
        WindowControlsOverlayGeometryChangeEvent {
            inner: Event::from_val(v),
        }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for WindowControlsOverlayGeometryChangeEvent {
    type Target = Event;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for WindowControlsOverlayGeometryChangeEvent {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for WindowControlsOverlayGeometryChangeEvent {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for WindowControlsOverlayGeometryChangeEvent {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<WindowControlsOverlayGeometryChangeEvent> for Any {
    fn from(s: WindowControlsOverlayGeometryChangeEvent) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&WindowControlsOverlayGeometryChangeEvent> for Any {
    fn from(s: &WindowControlsOverlayGeometryChangeEvent) -> Any {
        s.inner.clone().into()
    }
}
jsbind::utils::impl_dyn_cast!(WindowControlsOverlayGeometryChangeEvent);

impl WindowControlsOverlayGeometryChangeEvent {
    /// The `new WindowControlsOverlayGeometryChangeEvent(..)` constructor, creating a new WindowControlsOverlayGeometryChangeEvent instance
    pub fn new(
        type_: &JsString,
        event_init_dict: &WindowControlsOverlayGeometryChangeEventInit,
    ) -> WindowControlsOverlayGeometryChangeEvent {
        Self {
            inner: Any::global("WindowControlsOverlayGeometryChangeEvent")
                .new(&[type_.into(), event_init_dict.into()])
                .as_::<Event>(),
        }
    }
}
impl WindowControlsOverlayGeometryChangeEvent {
    /// Getter of the `titlebarAreaRect` attribute.
    /// [`WindowControlsOverlayGeometryChangeEvent.titlebarAreaRect`](https://developer.mozilla.org/en-US/docs/Web/API/WindowControlsOverlayGeometryChangeEvent/titlebarAreaRect)
    pub fn titlebar_area_rect(&self) -> DOMRect {
        self.inner.get("titlebarAreaRect").as_::<DOMRect>()
    }
}
impl WindowControlsOverlayGeometryChangeEvent {
    /// Getter of the `visible` attribute.
    /// [`WindowControlsOverlayGeometryChangeEvent.visible`](https://developer.mozilla.org/en-US/docs/Web/API/WindowControlsOverlayGeometryChangeEvent/visible)
    pub fn visible(&self) -> bool {
        self.inner.get("visible").as_::<bool>()
    }
}
