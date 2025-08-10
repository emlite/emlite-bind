use super::*;

/// The PerformanceMeasure class.
/// [`PerformanceMeasure`](https://developer.mozilla.org/en-US/docs/Web/API/PerformanceMeasure)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct PerformanceMeasure {
    inner: PerformanceEntry,
}

impl FromVal for PerformanceMeasure {
    fn from_val(v: &Any) -> Self {
        PerformanceMeasure {
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

impl core::ops::Deref for PerformanceMeasure {
    type Target = PerformanceEntry;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for PerformanceMeasure {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for PerformanceMeasure {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for PerformanceMeasure {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}

impl From<PerformanceMeasure> for Any {
    fn from(s: PerformanceMeasure) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&PerformanceMeasure> for Any {
    fn from(s: &PerformanceMeasure) -> Any {
        s.inner.clone().into()
    }
}

jsbind::utils::impl_dyn_cast!(PerformanceMeasure);

impl PerformanceMeasure {
    /// Getter of the `detail` attribute.
    /// [`PerformanceMeasure.detail`](https://developer.mozilla.org/en-US/docs/Web/API/PerformanceMeasure/detail)
    pub fn detail(&self) -> Any {
        self.inner.get("detail").as_::<Any>()
    }
}
