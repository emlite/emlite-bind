use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct ReportingObserverOptions {
    inner: Any,
}
impl FromVal for ReportingObserverOptions {
    fn from_val(v: &Any) -> Self {
        ReportingObserverOptions { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for ReportingObserverOptions {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for ReportingObserverOptions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for ReportingObserverOptions {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for ReportingObserverOptions {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<ReportingObserverOptions> for Any {
    fn from(s: ReportingObserverOptions) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&ReportingObserverOptions> for Any {
    fn from(s: &ReportingObserverOptions) -> Any {
        s.inner.clone()
    }
}

impl ReportingObserverOptions {
    pub fn types(&self) -> TypedArray<JsString> {
        self.inner.get("types").as_::<TypedArray<JsString>>()
    }

    pub fn set_types(&mut self, value: &TypedArray<JsString>) {
        self.inner.set("types", value);
    }
}
impl ReportingObserverOptions {
    pub fn buffered(&self) -> bool {
        self.inner.get("buffered").as_::<bool>()
    }

    pub fn set_buffered(&mut self, value: bool) {
        self.inner.set("buffered", value);
    }
}
