use super::*;

/// The PerformanceObserverEntryList class.
/// [`PerformanceObserverEntryList`](https://developer.mozilla.org/en-US/docs/Web/API/PerformanceObserverEntryList)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct PerformanceObserverEntryList {
    inner: Any,
}
impl FromVal for PerformanceObserverEntryList {
    fn from_val(v: &Any) -> Self {
        PerformanceObserverEntryList {
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
impl core::ops::Deref for PerformanceObserverEntryList {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for PerformanceObserverEntryList {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for PerformanceObserverEntryList {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for PerformanceObserverEntryList {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<PerformanceObserverEntryList> for Any {
    fn from(s: PerformanceObserverEntryList) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&PerformanceObserverEntryList> for Any {
    fn from(s: &PerformanceObserverEntryList) -> Any {
        s.inner.clone().into()
    }
}
jsbind::utils::impl_dyn_cast!(PerformanceObserverEntryList);

impl PerformanceObserverEntryList {
    /// The getEntries method.
    /// [`PerformanceObserverEntryList.getEntries`](https://developer.mozilla.org/en-US/docs/Web/API/PerformanceObserverEntryList/getEntries)
    pub fn get_entries(&self) -> Any {
        self.inner.call("getEntries", &[]).as_::<Any>()
    }
}
impl PerformanceObserverEntryList {
    /// The getEntriesByType method.
    /// [`PerformanceObserverEntryList.getEntriesByType`](https://developer.mozilla.org/en-US/docs/Web/API/PerformanceObserverEntryList/getEntriesByType)
    pub fn get_entries_by_type(&self, type_: &DOMString) -> Any {
        self.inner
            .call("getEntriesByType", &[type_.into()])
            .as_::<Any>()
    }
}
impl PerformanceObserverEntryList {
    /// The getEntriesByName method.
    /// [`PerformanceObserverEntryList.getEntriesByName`](https://developer.mozilla.org/en-US/docs/Web/API/PerformanceObserverEntryList/getEntriesByName)
    pub fn get_entries_by_name0(&self, name: &DOMString) -> Any {
        self.inner
            .call("getEntriesByName", &[name.into()])
            .as_::<Any>()
    }
    /// The getEntriesByName method.
    /// [`PerformanceObserverEntryList.getEntriesByName`](https://developer.mozilla.org/en-US/docs/Web/API/PerformanceObserverEntryList/getEntriesByName)
    pub fn get_entries_by_name1(&self, name: &DOMString, type_: &DOMString) -> Any {
        self.inner
            .call("getEntriesByName", &[name.into(), type_.into()])
            .as_::<Any>()
    }
}
