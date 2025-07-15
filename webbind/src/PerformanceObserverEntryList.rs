use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
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
impl core::ops::Deref for PerformanceObserverEntryList {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for PerformanceObserverEntryList {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for PerformanceObserverEntryList {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for PerformanceObserverEntryList {
    fn as_mut(&mut self) -> &mut emlite::Val {
        &mut self.inner
    }
}
impl From<PerformanceObserverEntryList> for emlite::Val {
    fn from(s: PerformanceObserverEntryList) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}
jsbind::utils::impl_dyn_cast!(PerformanceObserverEntryList);

impl PerformanceObserverEntryList {
    pub fn get_entries(&self) -> Any {
        self.inner.call("getEntries", &[]).as_::<Any>()
    }
}
impl PerformanceObserverEntryList {
    pub fn get_entries_by_type(&self, type_: DOMString) -> Any {
        self.inner
            .call("getEntriesByType", &[type_.into()])
            .as_::<Any>()
    }
}
impl PerformanceObserverEntryList {
    pub fn get_entries_by_name0(&self, name: DOMString) -> Any {
        self.inner
            .call("getEntriesByName", &[name.into()])
            .as_::<Any>()
    }

    pub fn get_entries_by_name1(&self, name: DOMString, type_: DOMString) -> Any {
        self.inner
            .call("getEntriesByName", &[name.into(), type_.into()])
            .as_::<Any>()
    }
}
