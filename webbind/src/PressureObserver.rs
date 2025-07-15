use super::*;




#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct PressureObserverOptions {
    inner: emlite::Val,
}
impl FromVal for PressureObserverOptions {
    fn from_val(v: &emlite::Val) -> Self {
        PressureObserverOptions { inner: v.clone() }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for PressureObserverOptions {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for PressureObserverOptions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for PressureObserverOptions {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for PressureObserverOptions {
    fn as_mut(&mut self) -> &mut emlite::Val {
      &mut self.inner
  }
}
impl From<PressureObserverOptions> for emlite::Val {
    fn from(s: PressureObserverOptions) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl PressureObserverOptions {
    pub fn sample_interval(&self) -> u32 {
        self.inner.get("sampleInterval").as_::<u32>()
    }

    pub fn set_sample_interval(&mut self, value: u32) {
        self.inner.set("sampleInterval", value);
    }

}
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct PressureObserver {
    inner: emlite::Val,
}
impl FromVal for PressureObserver {
    fn from_val(v: &emlite::Val) -> Self {
        PressureObserver { inner: emlite::Val::from_val(v) }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for PressureObserver {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for PressureObserver {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for PressureObserver {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for PressureObserver {
    fn as_mut(&mut self) -> &mut emlite::Val {
      &mut self.inner
  }
}
impl From<PressureObserver> for emlite::Val {
    fn from(s: PressureObserver) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}
jsbind::utils::impl_dyn_cast!(PressureObserver);



impl PressureObserver {
    pub fn new(callback: Function) -> PressureObserver {
        Self {
            inner: emlite::Val::global("PressureObserver").new(&[callback.into()]).as_::<emlite::Val>(),
        }
    }

}
impl PressureObserver {
    pub fn observe0(&self, source: PressureSource) -> Promise {
        self.inner.call("observe", &[source.into(), ]).as_::<Promise>()
    }

    pub fn observe1(&self, source: PressureSource, options: PressureObserverOptions) -> Promise {
        self.inner.call("observe", &[source.into(), options.into(), ]).as_::<Promise>()
    }

}
impl PressureObserver {
    pub fn unobserve(&self, source: PressureSource) -> Undefined {
        self.inner.call("unobserve", &[source.into(), ]).as_::<Undefined>()
    }

}
impl PressureObserver {
    pub fn disconnect(&self, ) -> Undefined {
        self.inner.call("disconnect", &[]).as_::<Undefined>()
    }

}
impl PressureObserver {
    pub fn take_records(&self, ) -> Sequence<PressureRecord> {
        self.inner.call("takeRecords", &[]).as_::<Sequence<PressureRecord>>()
    }

}
impl PressureObserver {
    pub fn known_sources() -> FrozenArray<PressureSource> {
        emlite::Val::global("pressureobserver").get("knownSources").as_::<FrozenArray<PressureSource>>()
    }

}
