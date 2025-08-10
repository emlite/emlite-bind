use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct WindowControlsOverlayGeometryChangeEventInit {
    inner: Any,
}
impl FromVal for WindowControlsOverlayGeometryChangeEventInit {
    fn from_val(v: &Any) -> Self {
        WindowControlsOverlayGeometryChangeEventInit { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for WindowControlsOverlayGeometryChangeEventInit {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for WindowControlsOverlayGeometryChangeEventInit {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for WindowControlsOverlayGeometryChangeEventInit {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for WindowControlsOverlayGeometryChangeEventInit {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<WindowControlsOverlayGeometryChangeEventInit> for Any {
    fn from(s: WindowControlsOverlayGeometryChangeEventInit) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&WindowControlsOverlayGeometryChangeEventInit> for Any {
    fn from(s: &WindowControlsOverlayGeometryChangeEventInit) -> Any {
        s.inner.clone()
    }
}

impl WindowControlsOverlayGeometryChangeEventInit {
    pub fn titlebar_area_rect(&self) -> DOMRect {
        self.inner.get("titlebarAreaRect").as_::<DOMRect>()
    }

    pub fn set_titlebar_area_rect(&mut self, value: &DOMRect) {
        self.inner.set("titlebarAreaRect", value);
    }
}
impl WindowControlsOverlayGeometryChangeEventInit {
    pub fn visible(&self) -> bool {
        self.inner.get("visible").as_::<bool>()
    }

    pub fn set_visible(&mut self, value: bool) {
        self.inner.set("visible", value);
    }
}
