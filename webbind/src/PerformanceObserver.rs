use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct PerformanceObserverInit {
    inner: Any,
}
impl FromVal for PerformanceObserverInit {
    fn from_val(v: &Any) -> Self {
        PerformanceObserverInit { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for PerformanceObserverInit {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for PerformanceObserverInit {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for PerformanceObserverInit {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for PerformanceObserverInit {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<PerformanceObserverInit> for Any {
    fn from(s: PerformanceObserverInit) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&PerformanceObserverInit> for Any {
    fn from(s: &PerformanceObserverInit) -> Any {
        s.inner.clone()
    }
}

impl PerformanceObserverInit {
    pub fn entry_types(&self) -> Sequence<String> {
        self.inner.get("entryTypes").as_::<Sequence<String>>()
    }

    pub fn set_entry_types(&mut self, value: &Sequence<String>) {
        self.inner.set("entryTypes", value);
    }
}
impl PerformanceObserverInit {
    pub fn type_(&self) -> String {
        self.inner.get("type").as_::<String>()
    }

    pub fn set_type_(&mut self, value: &str) {
        self.inner.set("type", value);
    }
}
impl PerformanceObserverInit {
    pub fn buffered(&self) -> bool {
        self.inner.get("buffered").as_::<bool>()
    }

    pub fn set_buffered(&mut self, value: bool) {
        self.inner.set("buffered", value);
    }
}
/// The PerformanceObserver class.
/// [`PerformanceObserver`](https://developer.mozilla.org/en-US/docs/Web/API/PerformanceObserver)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct PerformanceObserver {
    inner: Any,
}
impl FromVal for PerformanceObserver {
    fn from_val(v: &Any) -> Self {
        PerformanceObserver {
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
impl core::ops::Deref for PerformanceObserver {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for PerformanceObserver {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for PerformanceObserver {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for PerformanceObserver {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<PerformanceObserver> for Any {
    fn from(s: PerformanceObserver) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&PerformanceObserver> for Any {
    fn from(s: &PerformanceObserver) -> Any {
        s.inner.clone().into()
    }
}
jsbind::utils::impl_dyn_cast!(PerformanceObserver);

impl PerformanceObserver {
    /// The `new PerformanceObserver(..)` constructor, creating a new PerformanceObserver instance
    pub fn new(callback: &Function) -> PerformanceObserver {
        Self {
            inner: Any::global("PerformanceObserver")
                .new(&[callback.into()])
                .as_::<Any>(),
        }
    }
}
impl PerformanceObserver {
    /// The observe method.
    /// [`PerformanceObserver.observe`](https://developer.mozilla.org/en-US/docs/Web/API/PerformanceObserver/observe)
    pub fn observe0(&self) -> Undefined {
        self.inner.call("observe", &[]).as_::<Undefined>()
    }
    /// The observe method.
    /// [`PerformanceObserver.observe`](https://developer.mozilla.org/en-US/docs/Web/API/PerformanceObserver/observe)
    pub fn observe1(&self, options: &PerformanceObserverInit) -> Undefined {
        self.inner
            .call("observe", &[options.into()])
            .as_::<Undefined>()
    }
}
impl PerformanceObserver {
    /// The disconnect method.
    /// [`PerformanceObserver.disconnect`](https://developer.mozilla.org/en-US/docs/Web/API/PerformanceObserver/disconnect)
    pub fn disconnect(&self) -> Undefined {
        self.inner.call("disconnect", &[]).as_::<Undefined>()
    }
}
impl PerformanceObserver {
    /// The takeRecords method.
    /// [`PerformanceObserver.takeRecords`](https://developer.mozilla.org/en-US/docs/Web/API/PerformanceObserver/takeRecords)
    pub fn take_records(&self) -> Any {
        self.inner.call("takeRecords", &[]).as_::<Any>()
    }
}
impl PerformanceObserver {
    /// Getter of the `supportedEntryTypes` static attribute.
    /// [`PerformanceObserver.supportedEntryTypes`](https://developer.mozilla.org/en-US/docs/Web/API/PerformanceObserver/supportedEntryTypes)
    pub fn supported_entry_types() -> FrozenArray<String> {
        Any::global("PerformanceObserver")
            .get("supportedEntryTypes")
            .as_::<FrozenArray<String>>()
    }
}
