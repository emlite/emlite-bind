use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
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
impl AsRef<emlite::Val> for XRPermissionStatus {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for XRPermissionStatus {
    fn as_mut(&mut self) -> &mut emlite::Val {
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
impl From<&XRPermissionStatus> for emlite::Val {
    fn from(s: &XRPermissionStatus) -> emlite::Val {
        s.inner.clone().into()
    }
}
jsbind::utils::impl_dyn_cast!(XRPermissionStatus);

impl XRPermissionStatus {
    pub fn granted(&self) -> FrozenArray<String> {
        self.inner.get("granted").as_::<FrozenArray<String>>()
    }

    pub fn set_granted(&mut self, value: &FrozenArray<String>) {
        self.inner.set("granted", value);
    }
}
