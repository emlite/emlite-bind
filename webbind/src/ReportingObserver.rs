use super::*;

/// The ReportingObserver class.
/// [`ReportingObserver`](https://developer.mozilla.org/en-US/docs/Web/API/ReportingObserver)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct ReportingObserver {
    inner: Any,
}
impl FromVal for ReportingObserver {
    fn from_val(v: &Any) -> Self {
        ReportingObserver {
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
impl core::ops::Deref for ReportingObserver {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for ReportingObserver {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for ReportingObserver {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for ReportingObserver {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<ReportingObserver> for Any {
    fn from(s: ReportingObserver) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&ReportingObserver> for Any {
    fn from(s: &ReportingObserver) -> Any {
        s.inner.clone().into()
    }
}
jsbind::utils::impl_dyn_cast!(ReportingObserver);

impl ReportingObserver {
    /// The `new ReportingObserver(..)` constructor, creating a new ReportingObserver instance
    pub fn new0(callback: &Function) -> ReportingObserver {
        Self {
            inner: Any::global("ReportingObserver")
                .new(&[callback.into()])
                .as_::<Any>(),
        }
    }

    /// The `new ReportingObserver(..)` constructor, creating a new ReportingObserver instance
    pub fn new1(callback: &Function, options: &Any) -> ReportingObserver {
        Self {
            inner: Any::global("ReportingObserver")
                .new(&[callback.into(), options.into()])
                .as_::<Any>(),
        }
    }
}
impl ReportingObserver {
    /// The observe method.
    /// [`ReportingObserver.observe`](https://developer.mozilla.org/en-US/docs/Web/API/ReportingObserver/observe)
    pub fn observe(&self) -> Undefined {
        self.inner.call("observe", &[]).as_::<Undefined>()
    }
}
impl ReportingObserver {
    /// The disconnect method.
    /// [`ReportingObserver.disconnect`](https://developer.mozilla.org/en-US/docs/Web/API/ReportingObserver/disconnect)
    pub fn disconnect(&self) -> Undefined {
        self.inner.call("disconnect", &[]).as_::<Undefined>()
    }
}
impl ReportingObserver {
    /// The takeRecords method.
    /// [`ReportingObserver.takeRecords`](https://developer.mozilla.org/en-US/docs/Web/API/ReportingObserver/takeRecords)
    pub fn take_records(&self) -> Any {
        self.inner.call("takeRecords", &[]).as_::<Any>()
    }
}
