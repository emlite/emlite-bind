use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
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
impl AsRef<emlite::Val> for ScreenOrientation {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for ScreenOrientation {
    fn as_mut(&mut self) -> &mut emlite::Val {
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
impl From<&ScreenOrientation> for emlite::Val {
    fn from(s: &ScreenOrientation) -> emlite::Val {
        s.inner.clone().into()
    }
}
jsbind::utils::impl_dyn_cast!(ScreenOrientation);

impl ScreenOrientation {
    pub fn lock(&self, orientation: &OrientationLockType) -> Promise {
        self.inner
            .call("lock", &[orientation.into()])
            .as_::<Promise>()
    }
}
impl ScreenOrientation {
    pub fn unlock(&self) -> Undefined {
        self.inner.call("unlock", &[]).as_::<Undefined>()
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
    pub fn onchange(&self) -> Any {
        self.inner.get("onchange").as_::<Any>()
    }

    pub fn set_onchange(&mut self, value: &Any) {
        self.inner.set("onchange", value);
    }
}
