use super::*;

#[derive(Clone, Debug)]
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
impl std::ops::Deref for PerformanceObserverInit {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for PerformanceObserverInit {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<PerformanceObserverInit> for emlite::Val {
    fn from(s: PerformanceObserverInit) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl PerformanceObserverInit {
    pub fn entry_types(&self) -> jsbind::Sequence<jsbind::DOMString> {
        self.inner
            .get("entryTypes")
            .as_::<jsbind::Sequence<jsbind::DOMString>>()
    }

    pub fn set_entry_types(&mut self, value: jsbind::Sequence<jsbind::DOMString>) {
        self.inner.set("entryTypes", value);
    }
}
impl PerformanceObserverInit {
    pub fn type_(&self) -> jsbind::DOMString {
        self.inner.get("type").as_::<jsbind::DOMString>()
    }

    pub fn set_type_(&mut self, value: jsbind::DOMString) {
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
#[derive(Clone, Debug)]
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
impl std::ops::Deref for PerformanceObserver {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for PerformanceObserver {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<PerformanceObserver> for emlite::Val {
    fn from(s: PerformanceObserver) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl PerformanceObserver {
    pub fn new(callback: jsbind::Function) -> PerformanceObserver {
        Self {
            inner: emlite::Val::global("PerformanceObserver")
                .new(&[callback.into()])
                .as_::<emlite::Val>(),
        }
    }
}
impl PerformanceObserver {
    pub fn observe0(&self) -> jsbind::Undefined {
        self.inner.call("observe", &[]).as_::<jsbind::Undefined>()
    }

    pub fn observe1(&self, options: PerformanceObserverInit) -> jsbind::Undefined {
        self.inner
            .call("observe", &[options.into()])
            .as_::<jsbind::Undefined>()
    }
}
impl PerformanceObserver {
    pub fn disconnect(&self) -> jsbind::Undefined {
        self.inner
            .call("disconnect", &[])
            .as_::<jsbind::Undefined>()
    }
}
impl PerformanceObserver {
    pub fn take_records(&self) -> jsbind::Any {
        self.inner.call("takeRecords", &[]).as_::<jsbind::Any>()
    }
}
impl PerformanceObserver {
    pub fn supported_entry_types() -> jsbind::FrozenArray<jsbind::DOMString> {
        emlite::Val::global("performanceobserver")
            .get("supportedEntryTypes")
            .as_::<jsbind::FrozenArray<jsbind::DOMString>>()
    }
}
