use super::*;

#[derive(Clone, Debug)]
pub struct PerformanceObserverEntryList {
    inner: emlite::Val,
}
impl FromVal for PerformanceObserverEntryList {
    fn from_val(v: &emlite::Val) -> Self {
        PerformanceObserverEntryList {
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
impl std::ops::Deref for PerformanceObserverEntryList {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for PerformanceObserverEntryList {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<PerformanceObserverEntryList> for emlite::Val {
    fn from(s: PerformanceObserverEntryList) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl PerformanceObserverEntryList {
    pub fn get_entries(&self) -> jsbind::Any {
        self.inner.call("getEntries", &[]).as_::<jsbind::Any>()
    }
}
impl PerformanceObserverEntryList {
    pub fn get_entries_by_type(&self, type_: jsbind::DOMString) -> jsbind::Any {
        self.inner
            .call("getEntriesByType", &[type_.into()])
            .as_::<jsbind::Any>()
    }
}
impl PerformanceObserverEntryList {
    pub fn get_entries_by_name0(&self, name: jsbind::DOMString) -> jsbind::Any {
        self.inner
            .call("getEntriesByName", &[name.into()])
            .as_::<jsbind::Any>()
    }

    pub fn get_entries_by_name1(
        &self,
        name: jsbind::DOMString,
        type_: jsbind::DOMString,
    ) -> jsbind::Any {
        self.inner
            .call("getEntriesByName", &[name.into(), type_.into()])
            .as_::<jsbind::Any>()
    }
}
