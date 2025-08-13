use super::*;




/// The LanguageDetectorCreateOptions dictionary.
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct LanguageDetectorCreateOptions {
    inner: Any,
}

impl FromVal for LanguageDetectorCreateOptions {
    fn from_val(v: &Any) -> Self {
        LanguageDetectorCreateOptions { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for LanguageDetectorCreateOptions {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for LanguageDetectorCreateOptions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for LanguageDetectorCreateOptions {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for LanguageDetectorCreateOptions {
    fn as_mut(&mut self) -> &mut Any {
      &mut self.inner
  }
}

impl From<LanguageDetectorCreateOptions> for Any {
    fn from(s: LanguageDetectorCreateOptions) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&LanguageDetectorCreateOptions> for Any {
    fn from(s: &LanguageDetectorCreateOptions) -> Any {
        s.inner.clone()
    }
}

impl LanguageDetectorCreateOptions {
    /// Getter of the `signal` attribute.
    pub fn signal(&self) -> AbortSignal {
        self.inner.get("signal").as_::<AbortSignal>()
    }

    /// Setter of the `signal` attribute.
    pub fn set_signal(&mut self, value: &AbortSignal) {
        self.inner.set("signal", value);
    }
}
impl LanguageDetectorCreateOptions {
    /// Getter of the `monitor` attribute.
    pub fn monitor(&self) -> Function {
        self.inner.get("monitor").as_::<Function>()
    }

    /// Setter of the `monitor` attribute.
    pub fn set_monitor(&mut self, value: &Function) {
        self.inner.set("monitor", value);
    }
}
