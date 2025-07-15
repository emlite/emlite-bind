use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct VirtualKeyboard {
    inner: EventTarget,
}
impl FromVal for VirtualKeyboard {
    fn from_val(v: &emlite::Val) -> Self {
        VirtualKeyboard {
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
impl core::ops::Deref for VirtualKeyboard {
    type Target = EventTarget;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for VirtualKeyboard {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for VirtualKeyboard {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for VirtualKeyboard {
    fn as_mut(&mut self) -> &mut emlite::Val {
        &mut self.inner
    }
}
impl From<VirtualKeyboard> for emlite::Val {
    fn from(s: VirtualKeyboard) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}
impl From<&VirtualKeyboard> for emlite::Val {
    fn from(s: &VirtualKeyboard) -> emlite::Val {
        s.inner.clone().into()
    }
}
jsbind::utils::impl_dyn_cast!(VirtualKeyboard);

impl VirtualKeyboard {
    pub fn show(&self) -> Undefined {
        self.inner.call("show", &[]).as_::<Undefined>()
    }
}
impl VirtualKeyboard {
    pub fn hide(&self) -> Undefined {
        self.inner.call("hide", &[]).as_::<Undefined>()
    }
}
impl VirtualKeyboard {
    pub fn bounding_rect(&self) -> DOMRect {
        self.inner.get("boundingRect").as_::<DOMRect>()
    }
}
impl VirtualKeyboard {
    pub fn overlays_content(&self) -> bool {
        self.inner.get("overlaysContent").as_::<bool>()
    }

    pub fn set_overlays_content(&mut self, value: bool) {
        self.inner.set("overlaysContent", value);
    }
}
impl VirtualKeyboard {
    pub fn ongeometrychange(&self) -> Any {
        self.inner.get("ongeometrychange").as_::<Any>()
    }

    pub fn set_ongeometrychange(&mut self, value: &Any) {
        self.inner.set("ongeometrychange", value);
    }
}
