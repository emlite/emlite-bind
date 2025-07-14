use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct BackgroundFetchOptions {
    inner: emlite::Val,
}
impl FromVal for BackgroundFetchOptions {
    fn from_val(v: &emlite::Val) -> Self {
        BackgroundFetchOptions { inner: v.clone() }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for BackgroundFetchOptions {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for BackgroundFetchOptions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<BackgroundFetchOptions> for emlite::Val {
    fn from(s: BackgroundFetchOptions) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl BackgroundFetchOptions {
    pub fn download_total(&self) -> u64 {
        self.inner.get("downloadTotal").as_::<u64>()
    }

    pub fn set_download_total(&mut self, value: u64) {
        self.inner.set("downloadTotal", value);
    }
}
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct BackgroundFetchManager {
    inner: emlite::Val,
}
impl FromVal for BackgroundFetchManager {
    fn from_val(v: &emlite::Val) -> Self {
        BackgroundFetchManager {
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
impl core::ops::Deref for BackgroundFetchManager {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for BackgroundFetchManager {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<BackgroundFetchManager> for emlite::Val {
    fn from(s: BackgroundFetchManager) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl BackgroundFetchManager {
    pub fn fetch0(&self, id: jsbind::DOMString, requests: jsbind::Any) -> jsbind::Promise {
        self.inner
            .call("fetch", &[id.into(), requests.into()])
            .as_::<jsbind::Promise>()
    }

    pub fn fetch1(
        &self,
        id: jsbind::DOMString,
        requests: jsbind::Any,
        options: BackgroundFetchOptions,
    ) -> jsbind::Promise {
        self.inner
            .call("fetch", &[id.into(), requests.into(), options.into()])
            .as_::<jsbind::Promise>()
    }
}
impl BackgroundFetchManager {
    pub fn get(&self, id: jsbind::DOMString) -> jsbind::Promise {
        self.inner
            .call("get", &[id.into()])
            .as_::<jsbind::Promise>()
    }
}
impl BackgroundFetchManager {
    pub fn get_ids(&self) -> jsbind::Promise {
        self.inner.call("getIds", &[]).as_::<jsbind::Promise>()
    }
}
