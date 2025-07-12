use super::*;

#[derive(Clone, Debug)]
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
impl std::ops::Deref for MediaKeySystemAccess {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for MediaKeySystemAccess {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<MediaKeySystemAccess> for emlite::Val {
    fn from(s: MediaKeySystemAccess) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl MediaKeySystemAccess {
    pub fn key_system(&self) -> jsbind::DOMString {
        self.inner.get("keySystem").as_::<jsbind::DOMString>()
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
    pub fn create_media_keys(&self) -> jsbind::Promise {
        self.inner
            .call("createMediaKeys", &[])
            .as_::<jsbind::Promise>()
    }
}
