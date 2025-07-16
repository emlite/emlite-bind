use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct PressureObserverOptions {
    inner: Any,
}
impl FromVal for PressureObserverOptions {
    fn from_val(v: &Any) -> Self {
        PressureObserverOptions { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for PressureObserverOptions {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for PressureObserverOptions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for PressureObserverOptions {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for PressureObserverOptions {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<PressureObserverOptions> for Any {
    fn from(s: PressureObserverOptions) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&PressureObserverOptions> for Any {
    fn from(s: &PressureObserverOptions) -> Any {
        s.inner.clone()
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
/// The PressureObserver class.
/// [`PressureObserver`](https://developer.mozilla.org/en-US/docs/Web/API/PressureObserver)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct PressureObserver {
    inner: Any,
}
impl FromVal for PressureObserver {
    fn from_val(v: &Any) -> Self {
        PressureObserver {
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
impl core::ops::Deref for PressureObserver {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for PressureObserver {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for PressureObserver {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for PressureObserver {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<PressureObserver> for Any {
    fn from(s: PressureObserver) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&PressureObserver> for Any {
    fn from(s: &PressureObserver) -> Any {
        s.inner.clone().into()
    }
}
jsbind::utils::impl_dyn_cast!(PressureObserver);

impl PressureObserver {
    /// The `new PressureObserver(..)` constructor, creating a new PressureObserver instance
    pub fn new(callback: &Function) -> PressureObserver {
        Self {
            inner: Any::global("PressureObserver")
                .new(&[callback.into()])
                .as_::<Any>(),
        }
    }
}
impl PressureObserver {
    /// The observe method.
    /// [`PressureObserver.observe`](https://developer.mozilla.org/en-US/docs/Web/API/PressureObserver/observe)
    pub fn observe0(&self, source: &PressureSource) -> Promise {
        self.inner
            .call("observe", &[source.into()])
            .as_::<Promise>()
    }
    /// The observe method.
    /// [`PressureObserver.observe`](https://developer.mozilla.org/en-US/docs/Web/API/PressureObserver/observe)
    pub fn observe1(&self, source: &PressureSource, options: &PressureObserverOptions) -> Promise {
        self.inner
            .call("observe", &[source.into(), options.into()])
            .as_::<Promise>()
    }
}
impl PressureObserver {
    /// The unobserve method.
    /// [`PressureObserver.unobserve`](https://developer.mozilla.org/en-US/docs/Web/API/PressureObserver/unobserve)
    pub fn unobserve(&self, source: &PressureSource) -> Undefined {
        self.inner
            .call("unobserve", &[source.into()])
            .as_::<Undefined>()
    }
}
impl PressureObserver {
    /// The disconnect method.
    /// [`PressureObserver.disconnect`](https://developer.mozilla.org/en-US/docs/Web/API/PressureObserver/disconnect)
    pub fn disconnect(&self) -> Undefined {
        self.inner.call("disconnect", &[]).as_::<Undefined>()
    }
}
impl PressureObserver {
    /// The takeRecords method.
    /// [`PressureObserver.takeRecords`](https://developer.mozilla.org/en-US/docs/Web/API/PressureObserver/takeRecords)
    pub fn take_records(&self) -> Sequence<PressureRecord> {
        self.inner
            .call("takeRecords", &[])
            .as_::<Sequence<PressureRecord>>()
    }
}
impl PressureObserver {
    /// Getter of the `knownSources` static attribute.
    /// [`PressureObserver.knownSources`](https://developer.mozilla.org/en-US/docs/Web/API/PressureObserver/knownSources)
    pub fn known_sources() -> FrozenArray<PressureSource> {
        Any::global("PressureObserver")
            .get("knownSources")
            .as_::<FrozenArray<PressureSource>>()
    }
}
