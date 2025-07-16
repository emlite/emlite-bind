use super::*;

/// The PerformanceMark class.
/// [`PerformanceMark`](https://developer.mozilla.org/en-US/docs/Web/API/PerformanceMark)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct PerformanceMark {
    inner: PerformanceEntry,
}
impl FromVal for PerformanceMark {
    fn from_val(v: &Any) -> Self {
        PerformanceMark {
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
impl core::ops::Deref for PerformanceMark {
    type Target = PerformanceEntry;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for PerformanceMark {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for PerformanceMark {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for PerformanceMark {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<PerformanceMark> for Any {
    fn from(s: PerformanceMark) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&PerformanceMark> for Any {
    fn from(s: &PerformanceMark) -> Any {
        s.inner.clone().into()
    }
}
jsbind::utils::impl_dyn_cast!(PerformanceMark);

impl PerformanceMark {
    /// The `new PerformanceMark(..)` constructor, creating a new PerformanceMark instance
    pub fn new0(mark_name: &str) -> PerformanceMark {
        Self {
            inner: Any::global("PerformanceMark")
                .new(&[mark_name.into()])
                .as_::<PerformanceEntry>(),
        }
    }

    /// The `new PerformanceMark(..)` constructor, creating a new PerformanceMark instance
    pub fn new1(mark_name: &str, mark_options: &PerformanceMarkOptions) -> PerformanceMark {
        Self {
            inner: Any::global("PerformanceMark")
                .new(&[mark_name.into(), mark_options.into()])
                .as_::<PerformanceEntry>(),
        }
    }
}
impl PerformanceMark {
    /// Getter of the `detail` attribute.
    /// [`PerformanceMark.detail`](https://developer.mozilla.org/en-US/docs/Web/API/PerformanceMark/detail)
    pub fn detail(&self) -> Any {
        self.inner.get("detail").as_::<Any>()
    }
}
