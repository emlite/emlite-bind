use super::*;

/// The VisibilityStateEntry class.
/// [`VisibilityStateEntry`](https://developer.mozilla.org/en-US/docs/Web/API/VisibilityStateEntry)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct VisibilityStateEntry {
    inner: PerformanceEntry,
}
impl FromVal for VisibilityStateEntry {
    fn from_val(v: &Any) -> Self {
        VisibilityStateEntry {
            inner: PerformanceEntry::from_val(v),
        }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for VisibilityStateEntry {
    type Target = PerformanceEntry;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for VisibilityStateEntry {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for VisibilityStateEntry {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for VisibilityStateEntry {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<VisibilityStateEntry> for Any {
    fn from(s: VisibilityStateEntry) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&VisibilityStateEntry> for Any {
    fn from(s: &VisibilityStateEntry) -> Any {
        s.inner.clone().into()
    }
}
jsbind::utils::impl_dyn_cast!(VisibilityStateEntry);

impl VisibilityStateEntry {
    /// Getter of the `name` attribute.
    /// [`VisibilityStateEntry.name`](https://developer.mozilla.org/en-US/docs/Web/API/VisibilityStateEntry/name)
    pub fn name(&self) -> String {
        self.inner.get("name").as_::<String>()
    }
}
impl VisibilityStateEntry {
    /// Getter of the `entryType` attribute.
    /// [`VisibilityStateEntry.entryType`](https://developer.mozilla.org/en-US/docs/Web/API/VisibilityStateEntry/entryType)
    pub fn entry_type(&self) -> String {
        self.inner.get("entryType").as_::<String>()
    }
}
impl VisibilityStateEntry {
    /// Getter of the `startTime` attribute.
    /// [`VisibilityStateEntry.startTime`](https://developer.mozilla.org/en-US/docs/Web/API/VisibilityStateEntry/startTime)
    pub fn start_time(&self) -> Any {
        self.inner.get("startTime").as_::<Any>()
    }
}
impl VisibilityStateEntry {
    /// Getter of the `duration` attribute.
    /// [`VisibilityStateEntry.duration`](https://developer.mozilla.org/en-US/docs/Web/API/VisibilityStateEntry/duration)
    pub fn duration(&self) -> u32 {
        self.inner.get("duration").as_::<u32>()
    }
}
