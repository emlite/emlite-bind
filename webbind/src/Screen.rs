use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct Screen {
    inner: emlite::Val,
}
impl FromVal for Screen {
    fn from_val(v: &emlite::Val) -> Self {
        Screen {
            inner: emlite::Val::from_val(v),
        }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for Screen {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for Screen {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for Screen {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for Screen {
    fn as_mut(&mut self) -> &mut emlite::Val {
        &mut self.inner
    }
}
impl From<Screen> for emlite::Val {
    fn from(s: Screen) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}
impl From<&Screen> for emlite::Val {
    fn from(s: &Screen) -> emlite::Val {
        s.inner.clone().into()
    }
}
jsbind::utils::impl_dyn_cast!(Screen);

impl Screen {
    pub fn avail_width(&self) -> i32 {
        self.inner.get("availWidth").as_::<i32>()
    }
}
impl Screen {
    pub fn avail_height(&self) -> i32 {
        self.inner.get("availHeight").as_::<i32>()
    }
}
impl Screen {
    pub fn width(&self) -> i32 {
        self.inner.get("width").as_::<i32>()
    }
}
impl Screen {
    pub fn height(&self) -> i32 {
        self.inner.get("height").as_::<i32>()
    }
}
impl Screen {
    pub fn color_depth(&self) -> u32 {
        self.inner.get("colorDepth").as_::<u32>()
    }
}
impl Screen {
    pub fn pixel_depth(&self) -> u32 {
        self.inner.get("pixelDepth").as_::<u32>()
    }
}
impl Screen {
    pub fn orientation(&self) -> ScreenOrientation {
        self.inner.get("orientation").as_::<ScreenOrientation>()
    }
}
impl Screen {
    pub fn is_extended(&self) -> bool {
        self.inner.get("isExtended").as_::<bool>()
    }
}
impl Screen {
    pub fn onchange(&self) -> Any {
        self.inner.get("onchange").as_::<Any>()
    }

    pub fn set_onchange(&mut self, value: Any) {
        self.inner.set("onchange", value);
    }
}
