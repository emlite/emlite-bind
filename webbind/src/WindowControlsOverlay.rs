use super::*;

#[derive(Clone, Debug)]
pub struct WindowControlsOverlay {
    inner: EventTarget,
}
impl FromVal for WindowControlsOverlay {
    fn from_val(v: &emlite::Val) -> Self {
        WindowControlsOverlay {
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
impl std::ops::Deref for WindowControlsOverlay {
    type Target = EventTarget;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for WindowControlsOverlay {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<WindowControlsOverlay> for emlite::Val {
    fn from(s: WindowControlsOverlay) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl WindowControlsOverlay {
    pub fn visible(&self) -> bool {
        self.inner.get("visible").as_::<bool>()
    }
}
impl WindowControlsOverlay {
    pub fn get_titlebar_area_rect(&self) -> DOMRect {
        self.inner.call("getTitlebarAreaRect", &[]).as_::<DOMRect>()
    }
}
impl WindowControlsOverlay {
    pub fn ongeometrychange(&self) -> jsbind::Any {
        self.inner.get("ongeometrychange").as_::<jsbind::Any>()
    }

    pub fn set_ongeometrychange(&mut self, value: jsbind::Any) {
        self.inner.set("ongeometrychange", value);
    }
}
