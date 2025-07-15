use super::*;




#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct WakeLockSentinel {
    inner: EventTarget,
}
impl FromVal for WakeLockSentinel {
    fn from_val(v: &emlite::Val) -> Self {
        WakeLockSentinel { inner: EventTarget::from_val(v) }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for WakeLockSentinel {
    type Target = EventTarget;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for WakeLockSentinel {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for WakeLockSentinel {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for WakeLockSentinel {
    fn as_mut(&mut self) -> &mut emlite::Val {
      &mut self.inner
  }
}
impl From<WakeLockSentinel> for emlite::Val {
    fn from(s: WakeLockSentinel) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}
jsbind::utils::impl_dyn_cast!(WakeLockSentinel);


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
    pub fn release(&self, ) -> Promise {
        self.inner.call("release", &[]).as_::<Promise>()
    }

}
impl WakeLockSentinel {
    pub fn onrelease(&self) -> Any {
        self.inner.get("onrelease").as_::<Any>()
    }

    pub fn set_onrelease(&mut self, value: Any) {
        self.inner.set("onrelease", value);
    }

}
