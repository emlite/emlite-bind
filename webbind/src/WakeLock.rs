use super::*;

#[derive(Clone, Debug)]
pub struct WakeLock {
    inner: emlite::Val,
}
impl FromVal for WakeLock {
    fn from_val(v: &emlite::Val) -> Self {
        WakeLock {
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
impl std::ops::Deref for WakeLock {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for WakeLock {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<WakeLock> for emlite::Val {
    fn from(s: WakeLock) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl WakeLock {
    pub fn request0(&self) -> jsbind::Promise {
        self.inner.call("request", &[]).as_::<jsbind::Promise>()
    }

    pub fn request1(&self, type_: WakeLockType) -> jsbind::Promise {
        self.inner
            .call("request", &[type_.into()])
            .as_::<jsbind::Promise>()
    }
}
