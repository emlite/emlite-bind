use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct PermissionStatus {
    inner: EventTarget,
}
impl FromVal for PermissionStatus {
    fn from_val(v: &emlite::Val) -> Self {
        PermissionStatus {
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
impl core::ops::Deref for PermissionStatus {
    type Target = EventTarget;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for PermissionStatus {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<PermissionStatus> for emlite::Val {
    fn from(s: PermissionStatus) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl PermissionStatus {
    pub fn state(&self) -> PermissionState {
        self.inner.get("state").as_::<PermissionState>()
    }
}
impl PermissionStatus {
    pub fn name(&self) -> jsbind::DOMString {
        self.inner.get("name").as_::<jsbind::DOMString>()
    }
}
impl PermissionStatus {
    pub fn onchange(&self) -> jsbind::Any {
        self.inner.get("onchange").as_::<jsbind::Any>()
    }

    pub fn set_onchange(&mut self, value: jsbind::Any) {
        self.inner.set("onchange", value);
    }
}
