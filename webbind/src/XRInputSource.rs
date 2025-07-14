use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct XRInputSource {
    inner: emlite::Val,
}
impl FromVal for XRInputSource {
    fn from_val(v: &emlite::Val) -> Self {
        XRInputSource {
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
impl core::ops::Deref for XRInputSource {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for XRInputSource {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<XRInputSource> for emlite::Val {
    fn from(s: XRInputSource) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl XRInputSource {
    pub fn handedness(&self) -> XRHandedness {
        self.inner.get("handedness").as_::<XRHandedness>()
    }
}
impl XRInputSource {
    pub fn target_ray_mode(&self) -> XRTargetRayMode {
        self.inner.get("targetRayMode").as_::<XRTargetRayMode>()
    }
}
impl XRInputSource {
    pub fn target_ray_space(&self) -> XRSpace {
        self.inner.get("targetRaySpace").as_::<XRSpace>()
    }
}
impl XRInputSource {
    pub fn grip_space(&self) -> XRSpace {
        self.inner.get("gripSpace").as_::<XRSpace>()
    }
}
impl XRInputSource {
    pub fn profiles(&self) -> jsbind::FrozenArray<jsbind::DOMString> {
        self.inner
            .get("profiles")
            .as_::<jsbind::FrozenArray<jsbind::DOMString>>()
    }
}
impl XRInputSource {
    pub fn skip_rendering(&self) -> bool {
        self.inner.get("skipRendering").as_::<bool>()
    }
}
impl XRInputSource {
    pub fn gamepad(&self) -> Gamepad {
        self.inner.get("gamepad").as_::<Gamepad>()
    }
}
impl XRInputSource {
    pub fn hand(&self) -> XRHand {
        self.inner.get("hand").as_::<XRHand>()
    }
}
