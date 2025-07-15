use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct MediaKeySystemAccess {
    inner: emlite::Val,
}
impl FromVal for MediaKeySystemAccess {
    fn from_val(v: &emlite::Val) -> Self {
        MediaKeySystemAccess {
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
impl core::ops::Deref for MediaKeySystemAccess {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for MediaKeySystemAccess {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for MediaKeySystemAccess {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for MediaKeySystemAccess {
    fn as_mut(&mut self) -> &mut emlite::Val {
        &mut self.inner
    }
}
impl From<MediaKeySystemAccess> for emlite::Val {
    fn from(s: MediaKeySystemAccess) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}
jsbind::utils::impl_dyn_cast!(MediaKeySystemAccess);

impl MediaKeySystemAccess {
    pub fn key_system(&self) -> DOMString {
        self.inner.get("keySystem").as_::<DOMString>()
    }
}
impl MediaKeySystemAccess {
    pub fn get_configuration(&self) -> MediaKeySystemConfiguration {
        self.inner
            .call("getConfiguration", &[])
            .as_::<MediaKeySystemConfiguration>()
    }
}
impl MediaKeySystemAccess {
    pub fn create_media_keys(&self) -> Promise {
        self.inner.call("createMediaKeys", &[]).as_::<Promise>()
    }
}
