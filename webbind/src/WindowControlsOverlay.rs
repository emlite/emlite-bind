use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
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
impl AsRef<emlite::Val> for WindowControlsOverlay {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for WindowControlsOverlay {
    fn as_mut(&mut self) -> &mut emlite::Val {
        &mut self.inner
    }
}
impl From<WindowControlsOverlay> for emlite::Val {
    fn from(s: WindowControlsOverlay) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}
jsbind::utils::impl_dyn_cast!(WindowControlsOverlay);

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
