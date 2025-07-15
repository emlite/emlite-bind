use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct MediaDeviceInfo {
    inner: emlite::Val,
}
impl FromVal for MediaDeviceInfo {
    fn from_val(v: &emlite::Val) -> Self {
        MediaDeviceInfo {
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
impl core::ops::Deref for MediaDeviceInfo {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for MediaDeviceInfo {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for MediaDeviceInfo {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for MediaDeviceInfo {
    fn as_mut(&mut self) -> &mut emlite::Val {
        &mut self.inner
    }
}
impl From<MediaDeviceInfo> for emlite::Val {
    fn from(s: MediaDeviceInfo) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}
impl From<&MediaDeviceInfo> for emlite::Val {
    fn from(s: &MediaDeviceInfo) -> emlite::Val {
        s.inner.clone().into()
    }
}
jsbind::utils::impl_dyn_cast!(MediaDeviceInfo);

impl MediaDeviceInfo {
    pub fn device_id(&self) -> String {
        self.inner.get("deviceId").as_::<String>()
    }
}
impl MediaDeviceInfo {
    pub fn kind(&self) -> MediaDeviceKind {
        self.inner.get("kind").as_::<MediaDeviceKind>()
    }
}
impl MediaDeviceInfo {
    pub fn label(&self) -> String {
        self.inner.get("label").as_::<String>()
    }
}
impl MediaDeviceInfo {
    pub fn group_id(&self) -> String {
        self.inner.get("groupId").as_::<String>()
    }
}
impl MediaDeviceInfo {
    pub fn to_json(&self) -> Object {
        self.inner.call("toJSON", &[]).as_::<Object>()
    }
}
