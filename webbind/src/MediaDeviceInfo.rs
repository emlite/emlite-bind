use super::*;

/// The MediaDeviceInfo class.
/// [`MediaDeviceInfo`](https://developer.mozilla.org/en-US/docs/Web/API/MediaDeviceInfo)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct MediaDeviceInfo {
    inner: Any,
}
impl FromVal for MediaDeviceInfo {
    fn from_val(v: &Any) -> Self {
        MediaDeviceInfo {
            inner: Any::from_val(v),
        }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for MediaDeviceInfo {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for MediaDeviceInfo {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for MediaDeviceInfo {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for MediaDeviceInfo {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<MediaDeviceInfo> for Any {
    fn from(s: MediaDeviceInfo) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&MediaDeviceInfo> for Any {
    fn from(s: &MediaDeviceInfo) -> Any {
        s.inner.clone().into()
    }
}
jsbind::utils::impl_dyn_cast!(MediaDeviceInfo);

impl MediaDeviceInfo {
    /// Getter of the `deviceId` attribute.
    /// [`MediaDeviceInfo.deviceId`](https://developer.mozilla.org/en-US/docs/Web/API/MediaDeviceInfo/deviceId)
    pub fn device_id(&self) -> DOMString {
        self.inner.get("deviceId").as_::<DOMString>()
    }
}
impl MediaDeviceInfo {
    /// Getter of the `kind` attribute.
    /// [`MediaDeviceInfo.kind`](https://developer.mozilla.org/en-US/docs/Web/API/MediaDeviceInfo/kind)
    pub fn kind(&self) -> MediaDeviceKind {
        self.inner.get("kind").as_::<MediaDeviceKind>()
    }
}
impl MediaDeviceInfo {
    /// Getter of the `label` attribute.
    /// [`MediaDeviceInfo.label`](https://developer.mozilla.org/en-US/docs/Web/API/MediaDeviceInfo/label)
    pub fn label(&self) -> DOMString {
        self.inner.get("label").as_::<DOMString>()
    }
}
impl MediaDeviceInfo {
    /// Getter of the `groupId` attribute.
    /// [`MediaDeviceInfo.groupId`](https://developer.mozilla.org/en-US/docs/Web/API/MediaDeviceInfo/groupId)
    pub fn group_id(&self) -> DOMString {
        self.inner.get("groupId").as_::<DOMString>()
    }
}
impl MediaDeviceInfo {
    /// The toJSON method.
    /// [`MediaDeviceInfo.toJSON`](https://developer.mozilla.org/en-US/docs/Web/API/MediaDeviceInfo/toJSON)
    pub fn to_json(&self) -> Object {
        self.inner.call("toJSON", &[]).as_::<Object>()
    }
}
