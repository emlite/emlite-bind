use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct SyncManager {
    inner: emlite::Val,
}
impl FromVal for SyncManager {
    fn from_val(v: &emlite::Val) -> Self {
        SyncManager {
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
impl core::ops::Deref for SyncManager {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for SyncManager {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for SyncManager {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for SyncManager {
    fn as_mut(&mut self) -> &mut emlite::Val {
        &mut self.inner
    }
}
impl From<SyncManager> for emlite::Val {
    fn from(s: SyncManager) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}
impl From<&SyncManager> for emlite::Val {
    fn from(s: &SyncManager) -> emlite::Val {
        s.inner.clone().into()
    }
}
jsbind::utils::impl_dyn_cast!(SyncManager);

impl SyncManager {
    pub fn register(&self, tag: DOMString) -> Promise {
        self.inner.call("register", &[tag.into()]).as_::<Promise>()
    }
}
impl SyncManager {
    pub fn get_tags(&self) -> Promise {
        self.inner.call("getTags", &[]).as_::<Promise>()
    }
}
