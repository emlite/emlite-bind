use super::*;

#[derive(Clone, Debug)]
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
impl std::ops::Deref for SyncManager {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for SyncManager {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<SyncManager> for emlite::Val {
    fn from(s: SyncManager) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl SyncManager {
    pub fn register(&self, tag: jsbind::DOMString) -> jsbind::Promise {
        self.inner
            .call("register", &[tag.into()])
            .as_::<jsbind::Promise>()
    }
}
impl SyncManager {
    pub fn get_tags(&self) -> jsbind::Promise {
        self.inner.call("getTags", &[]).as_::<jsbind::Promise>()
    }
}
