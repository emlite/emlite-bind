use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct WindowControlsOverlayGeometryChangeEvent {
    inner: Event,
}
impl FromVal for WindowControlsOverlayGeometryChangeEvent {
    fn from_val(v: &emlite::Val) -> Self {
        WindowControlsOverlayGeometryChangeEvent {
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
impl AsRef<emlite::Val> for WindowControlsOverlayGeometryChangeEvent {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for WindowControlsOverlayGeometryChangeEvent {
    fn as_mut(&mut self) -> &mut emlite::Val {
        &mut self.inner
    }
}
impl From<WindowControlsOverlayGeometryChangeEvent> for emlite::Val {
    fn from(s: WindowControlsOverlayGeometryChangeEvent) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}
impl From<&WindowControlsOverlayGeometryChangeEvent> for emlite::Val {
    fn from(s: &WindowControlsOverlayGeometryChangeEvent) -> emlite::Val {
        s.inner.clone().into()
    }
}
jsbind::utils::impl_dyn_cast!(WindowControlsOverlayGeometryChangeEvent);

impl WindowControlsOverlayGeometryChangeEvent {
    pub fn new(type_: DOMString, event_init_dict: Any) -> WindowControlsOverlayGeometryChangeEvent {
        Self {
            inner: emlite::Val::global("WindowControlsOverlayGeometryChangeEvent")
                .new(&[type_.into(), event_init_dict.into()])
                .as_::<Event>(),
        }
    }
}
impl WindowControlsOverlayGeometryChangeEvent {
    pub fn titlebar_area_rect(&self) -> DOMRect {
        self.inner.get("titlebarAreaRect").as_::<DOMRect>()
    }
}
impl WindowControlsOverlayGeometryChangeEvent {
    pub fn visible(&self) -> bool {
        self.inner.get("visible").as_::<bool>()
    }
}
