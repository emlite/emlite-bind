use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct SubscribeOptions {
    inner: Any,
}
impl FromVal for SubscribeOptions {
    fn from_val(v: &Any) -> Self {
        SubscribeOptions { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for SubscribeOptions {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for SubscribeOptions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for SubscribeOptions {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for SubscribeOptions {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<SubscribeOptions> for Any {
    fn from(s: SubscribeOptions) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&SubscribeOptions> for Any {
    fn from(s: &SubscribeOptions) -> Any {
        s.inner.clone()
    }
}

impl SubscribeOptions {
    pub fn signal(&self) -> AbortSignal {
        self.inner.get("signal").as_::<AbortSignal>()
    }

    pub fn set_signal(&mut self, value: &AbortSignal) {
        self.inner.set("signal", value);
    }
}
/// The Observable class.
/// [`Observable`](https://developer.mozilla.org/en-US/docs/Web/API/Observable)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct Observable {
    inner: Any,
}
impl FromVal for Observable {
    fn from_val(v: &Any) -> Self {
        Observable {
            inner: Any::from_val(v),
        }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for Observable {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for Observable {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for Observable {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for Observable {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<Observable> for Any {
    fn from(s: Observable) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&Observable> for Any {
    fn from(s: &Observable) -> Any {
        s.inner.clone().into()
    }
}
jsbind::utils::impl_dyn_cast!(Observable);

impl Observable {
    /// The `new Observable(..)` constructor, creating a new Observable instance
    pub fn new(callback: &Function) -> Observable {
        Self {
            inner: Any::global("Observable")
                .new(&[callback.into()])
                .as_::<Any>(),
        }
    }
}
impl Observable {
    /// The subscribe method.
    /// [`Observable.subscribe`](https://developer.mozilla.org/en-US/docs/Web/API/Observable/subscribe)
    pub fn subscribe0(&self) -> Undefined {
        self.inner.call("subscribe", &[]).as_::<Undefined>()
    }
    /// The subscribe method.
    /// [`Observable.subscribe`](https://developer.mozilla.org/en-US/docs/Web/API/Observable/subscribe)
    pub fn subscribe1(&self, observer: &Any) -> Undefined {
        self.inner
            .call("subscribe", &[observer.into()])
            .as_::<Undefined>()
    }
    /// The subscribe method.
    /// [`Observable.subscribe`](https://developer.mozilla.org/en-US/docs/Web/API/Observable/subscribe)
    pub fn subscribe2(&self, observer: &Any, options: &SubscribeOptions) -> Undefined {
        self.inner
            .call("subscribe", &[observer.into(), options.into()])
            .as_::<Undefined>()
    }
}
impl Observable {
    /// The from method.
    /// [`Observable.from`](https://developer.mozilla.org/en-US/docs/Web/API/Observable/from)
    pub fn from(value: &Any) -> Observable {
        Any::global("Observable")
            .call("from", &[value.into()])
            .as_::<Observable>()
    }
}
impl Observable {
    /// The takeUntil method.
    /// [`Observable.takeUntil`](https://developer.mozilla.org/en-US/docs/Web/API/Observable/takeUntil)
    pub fn take_until(&self, value: &Any) -> Observable {
        self.inner
            .call("takeUntil", &[value.into()])
            .as_::<Observable>()
    }
}
impl Observable {
    /// The map method.
    /// [`Observable.map`](https://developer.mozilla.org/en-US/docs/Web/API/Observable/map)
    pub fn map(&self, mapper: &Function) -> Observable {
        self.inner.call("map", &[mapper.into()]).as_::<Observable>()
    }
}
impl Observable {
    /// The filter method.
    /// [`Observable.filter`](https://developer.mozilla.org/en-US/docs/Web/API/Observable/filter)
    pub fn filter(&self, predicate: &Function) -> Observable {
        self.inner
            .call("filter", &[predicate.into()])
            .as_::<Observable>()
    }
}
impl Observable {
    /// The take method.
    /// [`Observable.take`](https://developer.mozilla.org/en-US/docs/Web/API/Observable/take)
    pub fn take(&self, amount: u64) -> Observable {
        self.inner
            .call("take", &[amount.into()])
            .as_::<Observable>()
    }
}
impl Observable {
    /// The drop method.
    /// [`Observable.drop`](https://developer.mozilla.org/en-US/docs/Web/API/Observable/drop)
    pub fn drop(&self, amount: u64) -> Observable {
        self.inner
            .call("drop", &[amount.into()])
            .as_::<Observable>()
    }
}
impl Observable {
    /// The flatMap method.
    /// [`Observable.flatMap`](https://developer.mozilla.org/en-US/docs/Web/API/Observable/flatMap)
    pub fn flat_map(&self, mapper: &Function) -> Observable {
        self.inner
            .call("flatMap", &[mapper.into()])
            .as_::<Observable>()
    }
}
impl Observable {
    /// The switchMap method.
    /// [`Observable.switchMap`](https://developer.mozilla.org/en-US/docs/Web/API/Observable/switchMap)
    pub fn switch_map(&self, mapper: &Function) -> Observable {
        self.inner
            .call("switchMap", &[mapper.into()])
            .as_::<Observable>()
    }
}
impl Observable {
    /// The inspect method.
    /// [`Observable.inspect`](https://developer.mozilla.org/en-US/docs/Web/API/Observable/inspect)
    pub fn inspect0(&self) -> Observable {
        self.inner.call("inspect", &[]).as_::<Observable>()
    }
    /// The inspect method.
    /// [`Observable.inspect`](https://developer.mozilla.org/en-US/docs/Web/API/Observable/inspect)
    pub fn inspect1(&self, inspector_union: &Any) -> Observable {
        self.inner
            .call("inspect", &[inspector_union.into()])
            .as_::<Observable>()
    }
}
impl Observable {
    /// The catch method.
    /// [`Observable.catch`](https://developer.mozilla.org/en-US/docs/Web/API/Observable/catch)
    pub fn catch(&self, callback: &Function) -> Observable {
        self.inner
            .call("catch", &[callback.into()])
            .as_::<Observable>()
    }
}
impl Observable {
    /// The finally method.
    /// [`Observable.finally`](https://developer.mozilla.org/en-US/docs/Web/API/Observable/finally)
    pub fn finally(&self, callback: &Any) -> Observable {
        self.inner
            .call("finally", &[callback.into()])
            .as_::<Observable>()
    }
}
impl Observable {
    /// The toArray method.
    /// [`Observable.toArray`](https://developer.mozilla.org/en-US/docs/Web/API/Observable/toArray)
    pub fn to_array0(&self) -> Promise<TypedArray<Any>> {
        self.inner
            .call("toArray", &[])
            .as_::<Promise<TypedArray<Any>>>()
    }
    /// The toArray method.
    /// [`Observable.toArray`](https://developer.mozilla.org/en-US/docs/Web/API/Observable/toArray)
    pub fn to_array1(&self, options: &SubscribeOptions) -> Promise<TypedArray<Any>> {
        self.inner
            .call("toArray", &[options.into()])
            .as_::<Promise<TypedArray<Any>>>()
    }
}
impl Observable {
    /// The forEach method.
    /// [`Observable.forEach`](https://developer.mozilla.org/en-US/docs/Web/API/Observable/forEach)
    pub fn for_each0(&self, callback: &Function) -> Promise<Undefined> {
        self.inner
            .call("forEach", &[callback.into()])
            .as_::<Promise<Undefined>>()
    }
    /// The forEach method.
    /// [`Observable.forEach`](https://developer.mozilla.org/en-US/docs/Web/API/Observable/forEach)
    pub fn for_each1(&self, callback: &Function, options: &SubscribeOptions) -> Promise<Undefined> {
        self.inner
            .call("forEach", &[callback.into(), options.into()])
            .as_::<Promise<Undefined>>()
    }
}
impl Observable {
    /// The every method.
    /// [`Observable.every`](https://developer.mozilla.org/en-US/docs/Web/API/Observable/every)
    pub fn every0(&self, predicate: &Function) -> Promise<bool> {
        self.inner
            .call("every", &[predicate.into()])
            .as_::<Promise<bool>>()
    }
    /// The every method.
    /// [`Observable.every`](https://developer.mozilla.org/en-US/docs/Web/API/Observable/every)
    pub fn every1(&self, predicate: &Function, options: &SubscribeOptions) -> Promise<bool> {
        self.inner
            .call("every", &[predicate.into(), options.into()])
            .as_::<Promise<bool>>()
    }
}
impl Observable {
    /// The first method.
    /// [`Observable.first`](https://developer.mozilla.org/en-US/docs/Web/API/Observable/first)
    pub fn first0(&self) -> Promise<Any> {
        self.inner.call("first", &[]).as_::<Promise<Any>>()
    }
    /// The first method.
    /// [`Observable.first`](https://developer.mozilla.org/en-US/docs/Web/API/Observable/first)
    pub fn first1(&self, options: &SubscribeOptions) -> Promise<Any> {
        self.inner
            .call("first", &[options.into()])
            .as_::<Promise<Any>>()
    }
}
impl Observable {
    /// The last method.
    /// [`Observable.last`](https://developer.mozilla.org/en-US/docs/Web/API/Observable/last)
    pub fn last0(&self) -> Promise<Any> {
        self.inner.call("last", &[]).as_::<Promise<Any>>()
    }
    /// The last method.
    /// [`Observable.last`](https://developer.mozilla.org/en-US/docs/Web/API/Observable/last)
    pub fn last1(&self, options: &SubscribeOptions) -> Promise<Any> {
        self.inner
            .call("last", &[options.into()])
            .as_::<Promise<Any>>()
    }
}
impl Observable {
    /// The find method.
    /// [`Observable.find`](https://developer.mozilla.org/en-US/docs/Web/API/Observable/find)
    pub fn find0(&self, predicate: &Function) -> Promise<Any> {
        self.inner
            .call("find", &[predicate.into()])
            .as_::<Promise<Any>>()
    }
    /// The find method.
    /// [`Observable.find`](https://developer.mozilla.org/en-US/docs/Web/API/Observable/find)
    pub fn find1(&self, predicate: &Function, options: &SubscribeOptions) -> Promise<Any> {
        self.inner
            .call("find", &[predicate.into(), options.into()])
            .as_::<Promise<Any>>()
    }
}
impl Observable {
    /// The some method.
    /// [`Observable.some`](https://developer.mozilla.org/en-US/docs/Web/API/Observable/some)
    pub fn some0(&self, predicate: &Function) -> Promise<bool> {
        self.inner
            .call("some", &[predicate.into()])
            .as_::<Promise<bool>>()
    }
    /// The some method.
    /// [`Observable.some`](https://developer.mozilla.org/en-US/docs/Web/API/Observable/some)
    pub fn some1(&self, predicate: &Function, options: &SubscribeOptions) -> Promise<bool> {
        self.inner
            .call("some", &[predicate.into(), options.into()])
            .as_::<Promise<bool>>()
    }
}
impl Observable {
    /// The reduce method.
    /// [`Observable.reduce`](https://developer.mozilla.org/en-US/docs/Web/API/Observable/reduce)
    pub fn reduce0(&self, reducer: &Function) -> Promise<Any> {
        self.inner
            .call("reduce", &[reducer.into()])
            .as_::<Promise<Any>>()
    }
    /// The reduce method.
    /// [`Observable.reduce`](https://developer.mozilla.org/en-US/docs/Web/API/Observable/reduce)
    pub fn reduce1(&self, reducer: &Function, initial_value: &Any) -> Promise<Any> {
        self.inner
            .call("reduce", &[reducer.into(), initial_value.into()])
            .as_::<Promise<Any>>()
    }
    /// The reduce method.
    /// [`Observable.reduce`](https://developer.mozilla.org/en-US/docs/Web/API/Observable/reduce)
    pub fn reduce2(
        &self,
        reducer: &Function,
        initial_value: &Any,
        options: &SubscribeOptions,
    ) -> Promise<Any> {
        self.inner
            .call(
                "reduce",
                &[reducer.into(), initial_value.into(), options.into()],
            )
            .as_::<Promise<Any>>()
    }
}
