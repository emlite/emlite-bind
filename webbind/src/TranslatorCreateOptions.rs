use super::*;

/// The TranslatorCreateOptions dictionary.
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct TranslatorCreateOptions {
    inner: Any,
}

impl FromVal for TranslatorCreateOptions {
    fn from_val(v: &Any) -> Self {
        TranslatorCreateOptions { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for TranslatorCreateOptions {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for TranslatorCreateOptions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for TranslatorCreateOptions {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for TranslatorCreateOptions {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}

impl From<TranslatorCreateOptions> for Any {
    fn from(s: TranslatorCreateOptions) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&TranslatorCreateOptions> for Any {
    fn from(s: &TranslatorCreateOptions) -> Any {
        s.inner.clone()
    }
}

impl TranslatorCreateOptions {
    /// Getter of the `signal` attribute.
    pub fn signal(&self) -> AbortSignal {
        self.inner.get("signal").as_::<AbortSignal>()
    }

    /// Setter of the `signal` attribute.
    pub fn set_signal(&mut self, value: &AbortSignal) {
        self.inner.set("signal", value);
    }
}
impl TranslatorCreateOptions {
    /// Getter of the `monitor` attribute.
    pub fn monitor(&self) -> Function {
        self.inner.get("monitor").as_::<Function>()
    }

    /// Setter of the `monitor` attribute.
    pub fn set_monitor(&mut self, value: &Function) {
        self.inner.set("monitor", value);
    }
}
