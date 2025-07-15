use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct PerformanceObserverInit {
    inner: emlite::Val,
}
impl FromVal for PerformanceObserverInit {
    fn from_val(v: &emlite::Val) -> Self {
        PerformanceObserverInit { inner: v.clone() }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for PerformanceObserverInit {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for PerformanceObserverInit {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for PerformanceObserverInit {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for PerformanceObserverInit {
    fn as_mut(&mut self) -> &mut emlite::Val {
        &mut self.inner
    }
}
impl From<PerformanceObserverInit> for emlite::Val {
    fn from(s: PerformanceObserverInit) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}
impl From<&PerformanceObserverInit> for emlite::Val {
    fn from(s: &PerformanceObserverInit) -> emlite::Val {
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
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct PerformanceObserver {
    inner: emlite::Val,
}
impl FromVal for PerformanceObserver {
    fn from_val(v: &emlite::Val) -> Self {
        PerformanceObserver {
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
impl core::ops::Deref for PerformanceObserver {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for PerformanceObserver {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for PerformanceObserver {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for PerformanceObserver {
    fn as_mut(&mut self) -> &mut emlite::Val {
        &mut self.inner
    }
}
impl From<PerformanceObserver> for emlite::Val {
    fn from(s: PerformanceObserver) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}
impl From<&PerformanceObserver> for emlite::Val {
    fn from(s: &PerformanceObserver) -> emlite::Val {
        s.inner.clone().into()
    }
}
jsbind::utils::impl_dyn_cast!(PerformanceObserver);

impl PerformanceObserver {
    pub fn new(callback: &Function) -> PerformanceObserver {
        Self {
            inner: emlite::Val::global("PerformanceObserver")
                .new(&[callback.into()])
                .as_::<emlite::Val>(),
        }
    }
}
impl PerformanceObserver {
    pub fn observe0(&self) -> Undefined {
        self.inner.call("observe", &[]).as_::<Undefined>()
    }

    pub fn observe1(&self, options: &PerformanceObserverInit) -> Undefined {
        self.inner
            .call("observe", &[options.into()])
            .as_::<Undefined>()
    }
}
impl PerformanceObserver {
    pub fn disconnect(&self) -> Undefined {
        self.inner.call("disconnect", &[]).as_::<Undefined>()
    }
}
impl PerformanceObserver {
    pub fn take_records(&self) -> Any {
        self.inner.call("takeRecords", &[]).as_::<Any>()
    }
}
impl PerformanceObserver {
    pub fn supported_entry_types() -> FrozenArray<String> {
        emlite::Val::global("PerformanceObserver")
            .get("supportedEntryTypes")
            .as_::<FrozenArray<String>>()
    }
}
