use super::*;

/// The WriterCreateOptions dictionary.
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct WriterCreateOptions {
    inner: Any,
}

impl FromVal for WriterCreateOptions {
    fn from_val(v: &Any) -> Self {
        WriterCreateOptions { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for WriterCreateOptions {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for WriterCreateOptions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for WriterCreateOptions {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for WriterCreateOptions {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}

impl From<WriterCreateOptions> for Any {
    fn from(s: WriterCreateOptions) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&WriterCreateOptions> for Any {
    fn from(s: &WriterCreateOptions) -> Any {
        s.inner.clone()
    }
}

impl WriterCreateOptions {
    /// Getter of the `signal` attribute.
    pub fn signal(&self) -> AbortSignal {
        self.inner.get("signal").as_::<AbortSignal>()
    }

    /// Setter of the `signal` attribute.
    pub fn set_signal(&mut self, value: &AbortSignal) {
        self.inner.set("signal", value);
    }
}
impl WriterCreateOptions {
    /// Getter of the `monitor` attribute.
    pub fn monitor(&self) -> Function {
        self.inner.get("monitor").as_::<Function>()
    }

    /// Setter of the `monitor` attribute.
    pub fn set_monitor(&mut self, value: &Function) {
        self.inner.set("monitor", value);
    }
}
impl WriterCreateOptions {
    /// Getter of the `sharedContext` attribute.
    pub fn shared_context(&self) -> JsString {
        self.inner.get("sharedContext").as_::<JsString>()
    }

    /// Setter of the `sharedContext` attribute.
    pub fn set_shared_context(&mut self, value: &JsString) {
        self.inner.set("sharedContext", value);
    }
}
