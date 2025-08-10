use super::*;

/// The PerformanceObserverInit dictionary.
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct PerformanceObserverInit {
    inner: Any,
}

impl FromVal for PerformanceObserverInit {
    fn from_val(v: &Any) -> Self {
        PerformanceObserverInit { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for PerformanceObserverInit {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for PerformanceObserverInit {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for PerformanceObserverInit {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for PerformanceObserverInit {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}

impl From<PerformanceObserverInit> for Any {
    fn from(s: PerformanceObserverInit) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&PerformanceObserverInit> for Any {
    fn from(s: &PerformanceObserverInit) -> Any {
        s.inner.clone()
    }
}

impl PerformanceObserverInit {
    /// Getter of the `entryTypes` attribute.
    pub fn entry_types(&self) -> TypedArray<JsString> {
        self.inner.get("entryTypes").as_::<TypedArray<JsString>>()
    }

    /// Setter of the `entryTypes` attribute.
    pub fn set_entry_types(&mut self, value: &TypedArray<JsString>) {
        self.inner.set("entryTypes", value);
    }
}
impl PerformanceObserverInit {
    /// Getter of the `type` attribute.
    pub fn type_(&self) -> JsString {
        self.inner.get("type").as_::<JsString>()
    }

    /// Setter of the `type` attribute.
    pub fn set_type_(&mut self, value: &JsString) {
        self.inner.set("type", value);
    }
}
impl PerformanceObserverInit {
    /// Getter of the `buffered` attribute.
    pub fn buffered(&self) -> bool {
        self.inner.get("buffered").as_::<bool>()
    }

    /// Setter of the `buffered` attribute.
    pub fn set_buffered(&mut self, value: bool) {
        self.inner.set("buffered", value);
    }
}
