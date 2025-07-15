use super::*;




#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct IdleOptions {
    inner: emlite::Val,
}
impl FromVal for IdleOptions {
    fn from_val(v: &emlite::Val) -> Self {
        IdleOptions { inner: v.clone() }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for IdleOptions {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for IdleOptions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for IdleOptions {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for IdleOptions {
    fn as_mut(&mut self) -> &mut emlite::Val {
      &mut self.inner
  }
}
impl From<IdleOptions> for emlite::Val {
    fn from(s: IdleOptions) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl IdleOptions {
    pub fn threshold(&self) -> u64 {
        self.inner.get("threshold").as_::<u64>()
    }

    pub fn set_threshold(&mut self, value: u64) {
        self.inner.set("threshold", value);
    }

}
impl IdleOptions {
    pub fn signal(&self) -> AbortSignal {
        self.inner.get("signal").as_::<AbortSignal>()
    }

    pub fn set_signal(&mut self, value: AbortSignal) {
        self.inner.set("signal", value);
    }

}
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct IdleDetector {
    inner: EventTarget,
}
impl FromVal for IdleDetector {
    fn from_val(v: &emlite::Val) -> Self {
        IdleDetector { inner: EventTarget::from_val(v) }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for IdleDetector {
    type Target = EventTarget;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for IdleDetector {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for IdleDetector {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for IdleDetector {
    fn as_mut(&mut self) -> &mut emlite::Val {
      &mut self.inner
  }
}
impl From<IdleDetector> for emlite::Val {
    fn from(s: IdleDetector) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}
jsbind::utils::impl_dyn_cast!(IdleDetector);



impl IdleDetector {
    pub fn new() -> IdleDetector {
        Self {
            inner: emlite::Val::global("IdleDetector").new(&[]).as_::<EventTarget>(),
        }
    }

}
impl IdleDetector {
    pub fn user_state(&self) -> UserIdleState {
        self.inner.get("userState").as_::<UserIdleState>()
    }

}
impl IdleDetector {
    pub fn screen_state(&self) -> ScreenIdleState {
        self.inner.get("screenState").as_::<ScreenIdleState>()
    }

}
impl IdleDetector {
    pub fn onchange(&self) -> Any {
        self.inner.get("onchange").as_::<Any>()
    }

    pub fn set_onchange(&mut self, value: Any) {
        self.inner.set("onchange", value);
    }

}
impl IdleDetector {
    pub fn request_permission() -> Promise {
        emlite::Val::global("idledetector").call("requestPermission", &[]).as_::<Promise>()
    }

}
impl IdleDetector {
    pub fn start0(&self, ) -> Promise {
        self.inner.call("start", &[]).as_::<Promise>()
    }

    pub fn start1(&self, options: IdleOptions) -> Promise {
        self.inner.call("start", &[options.into(), ]).as_::<Promise>()
    }

}
