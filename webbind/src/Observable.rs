use super::*;




#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct SubscribeOptions {
    inner: emlite::Val,
}
impl FromVal for SubscribeOptions {
    fn from_val(v: &emlite::Val) -> Self {
        SubscribeOptions { inner: v.clone() }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for SubscribeOptions {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for SubscribeOptions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for SubscribeOptions {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for SubscribeOptions {
    fn as_mut(&mut self) -> &mut emlite::Val {
      &mut self.inner
  }
}
impl From<SubscribeOptions> for emlite::Val {
    fn from(s: SubscribeOptions) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl SubscribeOptions {
    pub fn signal(&self) -> AbortSignal {
        self.inner.get("signal").as_::<AbortSignal>()
    }

    pub fn set_signal(&mut self, value: AbortSignal) {
        self.inner.set("signal", value);
    }

}
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct Observable {
    inner: emlite::Val,
}
impl FromVal for Observable {
    fn from_val(v: &emlite::Val) -> Self {
        Observable { inner: emlite::Val::from_val(v) }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for Observable {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for Observable {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for Observable {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for Observable {
    fn as_mut(&mut self) -> &mut emlite::Val {
      &mut self.inner
  }
}
impl From<Observable> for emlite::Val {
    fn from(s: Observable) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}
jsbind::utils::impl_dyn_cast!(Observable);



impl Observable {
    pub fn new(callback: Function) -> Observable {
        Self {
            inner: emlite::Val::global("Observable").new(&[callback.into()]).as_::<emlite::Val>(),
        }
    }

}
impl Observable {
    pub fn subscribe0(&self, ) -> Undefined {
        self.inner.call("subscribe", &[]).as_::<Undefined>()
    }

    pub fn subscribe1(&self, observer: Any) -> Undefined {
        self.inner.call("subscribe", &[observer.into(), ]).as_::<Undefined>()
    }

    pub fn subscribe2(&self, observer: Any, options: SubscribeOptions) -> Undefined {
        self.inner.call("subscribe", &[observer.into(), options.into(), ]).as_::<Undefined>()
    }

}
impl Observable {
    pub fn from(value: Any) -> Observable {
        emlite::Val::global("observable").call("from", &[value.into(), ]).as_::<Observable>()
    }

}
impl Observable {
    pub fn take_until(&self, value: Any) -> Observable {
        self.inner.call("takeUntil", &[value.into(), ]).as_::<Observable>()
    }

}
impl Observable {
    pub fn map(&self, mapper: Function) -> Observable {
        self.inner.call("map", &[mapper.into(), ]).as_::<Observable>()
    }

}
impl Observable {
    pub fn filter(&self, predicate: Function) -> Observable {
        self.inner.call("filter", &[predicate.into(), ]).as_::<Observable>()
    }

}
impl Observable {
    pub fn take(&self, amount: u64) -> Observable {
        self.inner.call("take", &[amount.into(), ]).as_::<Observable>()
    }

}
impl Observable {
    pub fn drop(&self, amount: u64) -> Observable {
        self.inner.call("drop", &[amount.into(), ]).as_::<Observable>()
    }

}
impl Observable {
    pub fn flat_map(&self, mapper: Function) -> Observable {
        self.inner.call("flatMap", &[mapper.into(), ]).as_::<Observable>()
    }

}
impl Observable {
    pub fn switch_map(&self, mapper: Function) -> Observable {
        self.inner.call("switchMap", &[mapper.into(), ]).as_::<Observable>()
    }

}
impl Observable {
    pub fn inspect0(&self, ) -> Observable {
        self.inner.call("inspect", &[]).as_::<Observable>()
    }

    pub fn inspect1(&self, inspector_union: Any) -> Observable {
        self.inner.call("inspect", &[inspector_union.into(), ]).as_::<Observable>()
    }

}
impl Observable {
    pub fn catch(&self, callback: Function) -> Observable {
        self.inner.call("catch", &[callback.into(), ]).as_::<Observable>()
    }

}
impl Observable {
    pub fn finally(&self, callback: Any) -> Observable {
        self.inner.call("finally", &[callback.into(), ]).as_::<Observable>()
    }

}
impl Observable {
    pub fn to_array0(&self, ) -> Promise {
        self.inner.call("toArray", &[]).as_::<Promise>()
    }

    pub fn to_array1(&self, options: SubscribeOptions) -> Promise {
        self.inner.call("toArray", &[options.into(), ]).as_::<Promise>()
    }

}
impl Observable {
    pub fn for_each0(&self, callback: Function) -> Promise {
        self.inner.call("forEach", &[callback.into(), ]).as_::<Promise>()
    }

    pub fn for_each1(&self, callback: Function, options: SubscribeOptions) -> Promise {
        self.inner.call("forEach", &[callback.into(), options.into(), ]).as_::<Promise>()
    }

}
impl Observable {
    pub fn every0(&self, predicate: Function) -> Promise {
        self.inner.call("every", &[predicate.into(), ]).as_::<Promise>()
    }

    pub fn every1(&self, predicate: Function, options: SubscribeOptions) -> Promise {
        self.inner.call("every", &[predicate.into(), options.into(), ]).as_::<Promise>()
    }

}
impl Observable {
    pub fn first0(&self, ) -> Promise {
        self.inner.call("first", &[]).as_::<Promise>()
    }

    pub fn first1(&self, options: SubscribeOptions) -> Promise {
        self.inner.call("first", &[options.into(), ]).as_::<Promise>()
    }

}
impl Observable {
    pub fn last0(&self, ) -> Promise {
        self.inner.call("last", &[]).as_::<Promise>()
    }

    pub fn last1(&self, options: SubscribeOptions) -> Promise {
        self.inner.call("last", &[options.into(), ]).as_::<Promise>()
    }

}
impl Observable {
    pub fn find0(&self, predicate: Function) -> Promise {
        self.inner.call("find", &[predicate.into(), ]).as_::<Promise>()
    }

    pub fn find1(&self, predicate: Function, options: SubscribeOptions) -> Promise {
        self.inner.call("find", &[predicate.into(), options.into(), ]).as_::<Promise>()
    }

}
impl Observable {
    pub fn some0(&self, predicate: Function) -> Promise {
        self.inner.call("some", &[predicate.into(), ]).as_::<Promise>()
    }

    pub fn some1(&self, predicate: Function, options: SubscribeOptions) -> Promise {
        self.inner.call("some", &[predicate.into(), options.into(), ]).as_::<Promise>()
    }

}
impl Observable {
    pub fn reduce0(&self, reducer: Function) -> Promise {
        self.inner.call("reduce", &[reducer.into(), ]).as_::<Promise>()
    }

    pub fn reduce1(&self, reducer: Function, initial_value: Any) -> Promise {
        self.inner.call("reduce", &[reducer.into(), initial_value.into(), ]).as_::<Promise>()
    }

    pub fn reduce2(&self, reducer: Function, initial_value: Any, options: SubscribeOptions) -> Promise {
        self.inner.call("reduce", &[reducer.into(), initial_value.into(), options.into(), ]).as_::<Promise>()
    }

}
