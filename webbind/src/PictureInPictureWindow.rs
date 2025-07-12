use super::*;

#[derive(Clone, Debug)]
pub struct PictureInPictureWindow {
    inner: EventTarget,
}
impl FromVal for PictureInPictureWindow {
    fn from_val(v: &emlite::Val) -> Self {
        PictureInPictureWindow {
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
impl std::ops::Deref for PictureInPictureWindow {
    type Target = EventTarget;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for PictureInPictureWindow {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<PictureInPictureWindow> for emlite::Val {
    fn from(s: PictureInPictureWindow) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl PictureInPictureWindow {
    pub fn width(&self) -> i32 {
        self.inner.get("width").as_::<i32>()
    }
}
impl PictureInPictureWindow {
    pub fn height(&self) -> i32 {
        self.inner.get("height").as_::<i32>()
    }
}
impl PictureInPictureWindow {
    pub fn onresize(&self) -> jsbind::Any {
        self.inner.get("onresize").as_::<jsbind::Any>()
    }

    pub fn set_onresize(&mut self, value: jsbind::Any) {
        self.inner.set("onresize", value);
    }
}
