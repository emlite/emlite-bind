use super::*;

#[derive(Clone, Debug)]
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
impl std::ops::Deref for SubscribeOptions {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for SubscribeOptions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<SubscribeOptions> for emlite::Val {
    fn from(s: SubscribeOptions) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
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
#[derive(Clone, Debug)]
pub struct Observable {
    inner: emlite::Val,
}
impl FromVal for Observable {
    fn from_val(v: &emlite::Val) -> Self {
        Observable {
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
impl std::ops::Deref for Observable {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for Observable {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<Observable> for emlite::Val {
    fn from(s: Observable) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl Observable {
    pub fn new(callback: jsbind::Function) -> Observable {
        Self {
            inner: emlite::Val::global("Observable")
                .new(&[callback.into()])
                .as_::<emlite::Val>(),
        }
    }
}
impl Observable {
    pub fn subscribe0(&self) -> jsbind::Undefined {
        self.inner.call("subscribe", &[]).as_::<jsbind::Undefined>()
    }

    pub fn subscribe1(&self, observer: jsbind::Any) -> jsbind::Undefined {
        self.inner
            .call("subscribe", &[observer.into()])
            .as_::<jsbind::Undefined>()
    }

    pub fn subscribe2(
        &self,
        observer: jsbind::Any,
        options: SubscribeOptions,
    ) -> jsbind::Undefined {
        self.inner
            .call("subscribe", &[observer.into(), options.into()])
            .as_::<jsbind::Undefined>()
    }
}
impl Observable {
    pub fn from(value: jsbind::Any) -> Observable {
        emlite::Val::global("observable")
            .call("from", &[value.into()])
            .as_::<Observable>()
    }
}
impl Observable {
    pub fn take_until(&self, value: jsbind::Any) -> Observable {
        self.inner
            .call("takeUntil", &[value.into()])
            .as_::<Observable>()
    }
}
impl Observable {
    pub fn map(&self, mapper: jsbind::Function) -> Observable {
        self.inner.call("map", &[mapper.into()]).as_::<Observable>()
    }
}
impl Observable {
    pub fn filter(&self, predicate: jsbind::Function) -> Observable {
        self.inner
            .call("filter", &[predicate.into()])
            .as_::<Observable>()
    }
}
impl Observable {
    pub fn take(&self, amount: u64) -> Observable {
        self.inner
            .call("take", &[amount.into()])
            .as_::<Observable>()
    }
}
impl Observable {
    pub fn drop(&self, amount: u64) -> Observable {
        self.inner
            .call("drop", &[amount.into()])
            .as_::<Observable>()
    }
}
impl Observable {
    pub fn flat_map(&self, mapper: jsbind::Function) -> Observable {
        self.inner
            .call("flatMap", &[mapper.into()])
            .as_::<Observable>()
    }
}
impl Observable {
    pub fn switch_map(&self, mapper: jsbind::Function) -> Observable {
        self.inner
            .call("switchMap", &[mapper.into()])
            .as_::<Observable>()
    }
}
impl Observable {
    pub fn inspect0(&self) -> Observable {
        self.inner.call("inspect", &[]).as_::<Observable>()
    }

    pub fn inspect1(&self, inspector_union: jsbind::Any) -> Observable {
        self.inner
            .call("inspect", &[inspector_union.into()])
            .as_::<Observable>()
    }
}
impl Observable {
    pub fn catch(&self, callback: jsbind::Function) -> Observable {
        self.inner
            .call("catch", &[callback.into()])
            .as_::<Observable>()
    }
}
impl Observable {
    pub fn finally(&self, callback: jsbind::Any) -> Observable {
        self.inner
            .call("finally", &[callback.into()])
            .as_::<Observable>()
    }
}
impl Observable {
    pub fn to_array0(&self) -> jsbind::Promise {
        self.inner.call("toArray", &[]).as_::<jsbind::Promise>()
    }

    pub fn to_array1(&self, options: SubscribeOptions) -> jsbind::Promise {
        self.inner
            .call("toArray", &[options.into()])
            .as_::<jsbind::Promise>()
    }
}
impl Observable {
    pub fn for_each0(&self, callback: jsbind::Function) -> jsbind::Promise {
        self.inner
            .call("forEach", &[callback.into()])
            .as_::<jsbind::Promise>()
    }

    pub fn for_each1(
        &self,
        callback: jsbind::Function,
        options: SubscribeOptions,
    ) -> jsbind::Promise {
        self.inner
            .call("forEach", &[callback.into(), options.into()])
            .as_::<jsbind::Promise>()
    }
}
impl Observable {
    pub fn every0(&self, predicate: jsbind::Function) -> jsbind::Promise {
        self.inner
            .call("every", &[predicate.into()])
            .as_::<jsbind::Promise>()
    }

    pub fn every1(
        &self,
        predicate: jsbind::Function,
        options: SubscribeOptions,
    ) -> jsbind::Promise {
        self.inner
            .call("every", &[predicate.into(), options.into()])
            .as_::<jsbind::Promise>()
    }
}
impl Observable {
    pub fn first0(&self) -> jsbind::Promise {
        self.inner.call("first", &[]).as_::<jsbind::Promise>()
    }

    pub fn first1(&self, options: SubscribeOptions) -> jsbind::Promise {
        self.inner
            .call("first", &[options.into()])
            .as_::<jsbind::Promise>()
    }
}
impl Observable {
    pub fn last0(&self) -> jsbind::Promise {
        self.inner.call("last", &[]).as_::<jsbind::Promise>()
    }

    pub fn last1(&self, options: SubscribeOptions) -> jsbind::Promise {
        self.inner
            .call("last", &[options.into()])
            .as_::<jsbind::Promise>()
    }
}
impl Observable {
    pub fn find0(&self, predicate: jsbind::Function) -> jsbind::Promise {
        self.inner
            .call("find", &[predicate.into()])
            .as_::<jsbind::Promise>()
    }

    pub fn find1(&self, predicate: jsbind::Function, options: SubscribeOptions) -> jsbind::Promise {
        self.inner
            .call("find", &[predicate.into(), options.into()])
            .as_::<jsbind::Promise>()
    }
}
impl Observable {
    pub fn some0(&self, predicate: jsbind::Function) -> jsbind::Promise {
        self.inner
            .call("some", &[predicate.into()])
            .as_::<jsbind::Promise>()
    }

    pub fn some1(&self, predicate: jsbind::Function, options: SubscribeOptions) -> jsbind::Promise {
        self.inner
            .call("some", &[predicate.into(), options.into()])
            .as_::<jsbind::Promise>()
    }
}
impl Observable {
    pub fn reduce0(&self, reducer: jsbind::Function) -> jsbind::Promise {
        self.inner
            .call("reduce", &[reducer.into()])
            .as_::<jsbind::Promise>()
    }

    pub fn reduce1(
        &self,
        reducer: jsbind::Function,
        initial_value: jsbind::Any,
    ) -> jsbind::Promise {
        self.inner
            .call("reduce", &[reducer.into(), initial_value.into()])
            .as_::<jsbind::Promise>()
    }

    pub fn reduce2(
        &self,
        reducer: jsbind::Function,
        initial_value: jsbind::Any,
        options: SubscribeOptions,
    ) -> jsbind::Promise {
        self.inner
            .call(
                "reduce",
                &[reducer.into(), initial_value.into(), options.into()],
            )
            .as_::<jsbind::Promise>()
    }
}
