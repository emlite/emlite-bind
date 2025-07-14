use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct ScreenOrientation {
    inner: EventTarget,
}
impl FromVal for ScreenOrientation {
    fn from_val(v: &emlite::Val) -> Self {
        ScreenOrientation {
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
impl core::ops::Deref for ScreenOrientation {
    type Target = EventTarget;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for ScreenOrientation {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<ScreenOrientation> for emlite::Val {
    fn from(s: ScreenOrientation) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl ScreenOrientation {
    pub fn lock(&self, orientation: OrientationLockType) -> jsbind::Promise {
        self.inner
            .call("lock", &[orientation.into()])
            .as_::<jsbind::Promise>()
    }
}
impl ScreenOrientation {
    pub fn unlock(&self) -> jsbind::Undefined {
        self.inner.call("unlock", &[]).as_::<jsbind::Undefined>()
    }
}
impl ScreenOrientation {
    pub fn type_(&self) -> OrientationType {
        self.inner.get("type").as_::<OrientationType>()
    }
}
impl ScreenOrientation {
    pub fn angle(&self) -> u16 {
        self.inner.get("angle").as_::<u16>()
    }
}
impl ScreenOrientation {
    pub fn onchange(&self) -> jsbind::Any {
        self.inner.get("onchange").as_::<jsbind::Any>()
    }

    pub fn set_onchange(&mut self, value: jsbind::Any) {
        self.inner.set("onchange", value);
    }
}
