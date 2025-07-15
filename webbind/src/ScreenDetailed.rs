use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct ScreenDetailed {
    inner: Screen,
}
impl FromVal for ScreenDetailed {
    fn from_val(v: &emlite::Val) -> Self {
        ScreenDetailed {
            inner: Screen::from_val(v),
        }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for ScreenDetailed {
    type Target = Screen;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for ScreenDetailed {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for ScreenDetailed {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for ScreenDetailed {
    fn as_mut(&mut self) -> &mut emlite::Val {
        &mut self.inner
    }
}
impl From<ScreenDetailed> for emlite::Val {
    fn from(s: ScreenDetailed) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}
impl From<&ScreenDetailed> for emlite::Val {
    fn from(s: &ScreenDetailed) -> emlite::Val {
        s.inner.clone().into()
    }
}
jsbind::utils::impl_dyn_cast!(ScreenDetailed);

impl ScreenDetailed {
    pub fn avail_left(&self) -> i32 {
        self.inner.get("availLeft").as_::<i32>()
    }
}
impl ScreenDetailed {
    pub fn avail_top(&self) -> i32 {
        self.inner.get("availTop").as_::<i32>()
    }
}
impl ScreenDetailed {
    pub fn left(&self) -> i32 {
        self.inner.get("left").as_::<i32>()
    }
}
impl ScreenDetailed {
    pub fn top(&self) -> i32 {
        self.inner.get("top").as_::<i32>()
    }
}
impl ScreenDetailed {
    pub fn is_primary(&self) -> bool {
        self.inner.get("isPrimary").as_::<bool>()
    }
}
impl ScreenDetailed {
    pub fn is_internal(&self) -> bool {
        self.inner.get("isInternal").as_::<bool>()
    }
}
impl ScreenDetailed {
    pub fn device_pixel_ratio(&self) -> f32 {
        self.inner.get("devicePixelRatio").as_::<f32>()
    }
}
impl ScreenDetailed {
    pub fn label(&self) -> String {
        self.inner.get("label").as_::<String>()
    }
}
