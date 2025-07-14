use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct BackgroundSyncOptions {
    inner: emlite::Val,
}
impl FromVal for BackgroundSyncOptions {
    fn from_val(v: &emlite::Val) -> Self {
        BackgroundSyncOptions { inner: v.clone() }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for BackgroundSyncOptions {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for BackgroundSyncOptions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<BackgroundSyncOptions> for emlite::Val {
    fn from(s: BackgroundSyncOptions) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl BackgroundSyncOptions {
    pub fn min_interval(&self) -> u64 {
        self.inner.get("minInterval").as_::<u64>()
    }

    pub fn set_min_interval(&mut self, value: u64) {
        self.inner.set("minInterval", value);
    }
}
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct PeriodicSyncManager {
    inner: emlite::Val,
}
impl FromVal for PeriodicSyncManager {
    fn from_val(v: &emlite::Val) -> Self {
        PeriodicSyncManager {
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
impl core::ops::Deref for PeriodicSyncManager {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for PeriodicSyncManager {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<PeriodicSyncManager> for emlite::Val {
    fn from(s: PeriodicSyncManager) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl PeriodicSyncManager {
    pub fn register0(&self, tag: jsbind::DOMString) -> jsbind::Promise {
        self.inner
            .call("register", &[tag.into()])
            .as_::<jsbind::Promise>()
    }

    pub fn register1(
        &self,
        tag: jsbind::DOMString,
        options: BackgroundSyncOptions,
    ) -> jsbind::Promise {
        self.inner
            .call("register", &[tag.into(), options.into()])
            .as_::<jsbind::Promise>()
    }
}
impl PeriodicSyncManager {
    pub fn get_tags(&self) -> jsbind::Promise {
        self.inner.call("getTags", &[]).as_::<jsbind::Promise>()
    }
}
impl PeriodicSyncManager {
    pub fn unregister(&self, tag: jsbind::DOMString) -> jsbind::Promise {
        self.inner
            .call("unregister", &[tag.into()])
            .as_::<jsbind::Promise>()
    }
}
