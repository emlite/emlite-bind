use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct XRPermissionStatus {
    inner: PermissionStatus,
}
impl FromVal for XRPermissionStatus {
    fn from_val(v: &emlite::Val) -> Self {
        XRPermissionStatus {
            inner: PermissionStatus::from_val(v),
        }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for XRPermissionStatus {
    type Target = PermissionStatus;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for XRPermissionStatus {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<XRPermissionStatus> for emlite::Val {
    fn from(s: XRPermissionStatus) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl XRPermissionStatus {
    pub fn granted(&self) -> jsbind::FrozenArray<jsbind::DOMString> {
        self.inner
            .get("granted")
            .as_::<jsbind::FrozenArray<jsbind::DOMString>>()
    }

    pub fn set_granted(&mut self, value: jsbind::FrozenArray<jsbind::DOMString>) {
        self.inner.set("granted", value);
    }
}
