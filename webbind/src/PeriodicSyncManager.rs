use super::*;




#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
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
impl AsRef<emlite::Val> for BackgroundSyncOptions {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for BackgroundSyncOptions {
    fn as_mut(&mut self) -> &mut emlite::Val {
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
#[repr(transparent)]
pub struct PeriodicSyncManager {
    inner: emlite::Val,
}
impl FromVal for PeriodicSyncManager {
    fn from_val(v: &emlite::Val) -> Self {
        PeriodicSyncManager { inner: emlite::Val::from_val(v) }
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
impl AsRef<emlite::Val> for PeriodicSyncManager {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for PeriodicSyncManager {
    fn as_mut(&mut self) -> &mut emlite::Val {
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
jsbind::utils::impl_dyn_cast!(PeriodicSyncManager);


impl PeriodicSyncManager {
    pub fn register0(&self, tag: DOMString) -> Promise {
        self.inner.call("register", &[tag.into(), ]).as_::<Promise>()
    }

    pub fn register1(&self, tag: DOMString, options: BackgroundSyncOptions) -> Promise {
        self.inner.call("register", &[tag.into(), options.into(), ]).as_::<Promise>()
    }

}
impl PeriodicSyncManager {
    pub fn get_tags(&self, ) -> Promise {
        self.inner.call("getTags", &[]).as_::<Promise>()
    }

}
impl PeriodicSyncManager {
    pub fn unregister(&self, tag: DOMString) -> Promise {
        self.inner.call("unregister", &[tag.into(), ]).as_::<Promise>()
    }

}
