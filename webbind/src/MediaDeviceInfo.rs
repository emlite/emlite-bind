use super::*;

#[derive(Clone, Debug)]
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
impl std::ops::Deref for MediaDeviceInfo {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for MediaDeviceInfo {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<MediaDeviceInfo> for emlite::Val {
    fn from(s: MediaDeviceInfo) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl MediaDeviceInfo {
    pub fn device_id(&self) -> jsbind::DOMString {
        self.inner.get("deviceId").as_::<jsbind::DOMString>()
    }
}
impl MediaDeviceInfo {
    pub fn kind(&self) -> MediaDeviceKind {
        self.inner.get("kind").as_::<MediaDeviceKind>()
    }
}
impl MediaDeviceInfo {
    pub fn label(&self) -> jsbind::DOMString {
        self.inner.get("label").as_::<jsbind::DOMString>()
    }
}
impl MediaDeviceInfo {
    pub fn group_id(&self) -> jsbind::DOMString {
        self.inner.get("groupId").as_::<jsbind::DOMString>()
    }
}
impl MediaDeviceInfo {
    pub fn to_json(&self) -> jsbind::Object {
        self.inner.call("toJSON", &[]).as_::<jsbind::Object>()
    }
}
