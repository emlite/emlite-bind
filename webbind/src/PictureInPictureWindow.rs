use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
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
impl core::ops::Deref for PictureInPictureWindow {
    type Target = EventTarget;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for PictureInPictureWindow {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for PictureInPictureWindow {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for PictureInPictureWindow {
    fn as_mut(&mut self) -> &mut emlite::Val {
        &mut self.inner
    }
}
impl From<PictureInPictureWindow> for emlite::Val {
    fn from(s: PictureInPictureWindow) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}
impl From<&PictureInPictureWindow> for emlite::Val {
    fn from(s: &PictureInPictureWindow) -> emlite::Val {
        s.inner.clone().into()
    }
}
jsbind::utils::impl_dyn_cast!(PictureInPictureWindow);

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
    pub fn onresize(&self) -> Any {
        self.inner.get("onresize").as_::<Any>()
    }

    pub fn set_onresize(&mut self, value: &Any) {
        self.inner.set("onresize", value);
    }
}
