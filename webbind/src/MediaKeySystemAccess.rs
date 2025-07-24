use super::*;

/// The MediaKeySystemAccess class.
/// [`MediaKeySystemAccess`](https://developer.mozilla.org/en-US/docs/Web/API/MediaKeySystemAccess)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct MediaKeySystemAccess {
    inner: Any,
}
impl FromVal for MediaKeySystemAccess {
    fn from_val(v: &Any) -> Self {
        MediaKeySystemAccess {
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
impl core::ops::Deref for MediaKeySystemAccess {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for MediaKeySystemAccess {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for MediaKeySystemAccess {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for MediaKeySystemAccess {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<MediaKeySystemAccess> for Any {
    fn from(s: MediaKeySystemAccess) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&MediaKeySystemAccess> for Any {
    fn from(s: &MediaKeySystemAccess) -> Any {
        s.inner.clone().into()
    }
}
jsbind::utils::impl_dyn_cast!(MediaKeySystemAccess);

impl MediaKeySystemAccess {
    /// Getter of the `keySystem` attribute.
    /// [`MediaKeySystemAccess.keySystem`](https://developer.mozilla.org/en-US/docs/Web/API/MediaKeySystemAccess/keySystem)
    pub fn key_system(&self) -> DOMString {
        self.inner.get("keySystem").as_::<DOMString>()
    }
}
impl MediaKeySystemAccess {
    /// The getConfiguration method.
    /// [`MediaKeySystemAccess.getConfiguration`](https://developer.mozilla.org/en-US/docs/Web/API/MediaKeySystemAccess/getConfiguration)
    pub fn get_configuration(&self) -> MediaKeySystemConfiguration {
        self.inner
            .call("getConfiguration", &[])
            .as_::<MediaKeySystemConfiguration>()
    }
}
impl MediaKeySystemAccess {
    /// The createMediaKeys method.
    /// [`MediaKeySystemAccess.createMediaKeys`](https://developer.mozilla.org/en-US/docs/Web/API/MediaKeySystemAccess/createMediaKeys)
    pub fn create_media_keys(&self) -> Promise<MediaKeys> {
        self.inner
            .call("createMediaKeys", &[])
            .as_::<Promise<MediaKeys>>()
    }
}
