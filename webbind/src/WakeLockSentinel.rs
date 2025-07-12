use super::*;

#[derive(Clone, Debug)]
pub struct WakeLockSentinel {
    inner: EventTarget,
}
impl FromVal for WakeLockSentinel {
    fn from_val(v: &emlite::Val) -> Self {
        WakeLockSentinel {
            inner: EventTarget::from_val(v),
        }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl std::ops::Deref for WakeLockSentinel {
    type Target = EventTarget;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for WakeLockSentinel {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<WakeLockSentinel> for emlite::Val {
    fn from(s: WakeLockSentinel) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl WakeLockSentinel {
    pub fn released(&self) -> bool {
        self.inner.get("released").as_::<bool>()
    }
}
impl WakeLockSentinel {
    pub fn type_(&self) -> WakeLockType {
        self.inner.get("type").as_::<WakeLockType>()
    }
}
impl WakeLockSentinel {
    pub fn release(&self) -> jsbind::Promise {
        self.inner.call("release", &[]).as_::<jsbind::Promise>()
    }
}
impl WakeLockSentinel {
    pub fn onrelease(&self) -> jsbind::Any {
        self.inner.get("onrelease").as_::<jsbind::Any>()
    }

    pub fn set_onrelease(&mut self, value: jsbind::Any) {
        self.inner.set("onrelease", value);
    }
}
