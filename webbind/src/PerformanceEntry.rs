use super::*;

/// The PerformanceEntry class.
/// [`PerformanceEntry`](https://developer.mozilla.org/en-US/docs/Web/API/PerformanceEntry)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct PerformanceEntry {
    inner: Any,
}
impl FromVal for PerformanceEntry {
    fn from_val(v: &Any) -> Self {
        PerformanceEntry {
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
impl core::ops::Deref for PerformanceEntry {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for PerformanceEntry {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for PerformanceEntry {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for PerformanceEntry {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<PerformanceEntry> for Any {
    fn from(s: PerformanceEntry) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&PerformanceEntry> for Any {
    fn from(s: &PerformanceEntry) -> Any {
        s.inner.clone().into()
    }
}
jsbind::utils::impl_dyn_cast!(PerformanceEntry);

impl PerformanceEntry {
    /// Getter of the `id` attribute.
    /// [`PerformanceEntry.id`](https://developer.mozilla.org/en-US/docs/Web/API/PerformanceEntry/id)
    pub fn id(&self) -> u64 {
        self.inner.get("id").as_::<u64>()
    }
}
impl PerformanceEntry {
    /// Getter of the `name` attribute.
    /// [`PerformanceEntry.name`](https://developer.mozilla.org/en-US/docs/Web/API/PerformanceEntry/name)
    pub fn name(&self) -> JsString {
        self.inner.get("name").as_::<JsString>()
    }
}
impl PerformanceEntry {
    /// Getter of the `entryType` attribute.
    /// [`PerformanceEntry.entryType`](https://developer.mozilla.org/en-US/docs/Web/API/PerformanceEntry/entryType)
    pub fn entry_type(&self) -> JsString {
        self.inner.get("entryType").as_::<JsString>()
    }
}
impl PerformanceEntry {
    /// Getter of the `startTime` attribute.
    /// [`PerformanceEntry.startTime`](https://developer.mozilla.org/en-US/docs/Web/API/PerformanceEntry/startTime)
    pub fn start_time(&self) -> Any {
        self.inner.get("startTime").as_::<Any>()
    }
}
impl PerformanceEntry {
    /// Getter of the `duration` attribute.
    /// [`PerformanceEntry.duration`](https://developer.mozilla.org/en-US/docs/Web/API/PerformanceEntry/duration)
    pub fn duration(&self) -> Any {
        self.inner.get("duration").as_::<Any>()
    }
}
impl PerformanceEntry {
    /// Getter of the `navigationId` attribute.
    /// [`PerformanceEntry.navigationId`](https://developer.mozilla.org/en-US/docs/Web/API/PerformanceEntry/navigationId)
    pub fn navigation_id(&self) -> u64 {
        self.inner.get("navigationId").as_::<u64>()
    }
}
impl PerformanceEntry {
    /// The toJSON method.
    /// [`PerformanceEntry.toJSON`](https://developer.mozilla.org/en-US/docs/Web/API/PerformanceEntry/toJSON)
    pub fn to_json(&self) -> Object {
        self.inner.call("toJSON", &[]).as_::<Object>()
    }
}
