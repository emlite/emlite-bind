use super::*;




/// The SummarizerSummarizeOptions dictionary.
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct SummarizerSummarizeOptions {
    inner: Any,
}

impl FromVal for SummarizerSummarizeOptions {
    fn from_val(v: &Any) -> Self {
        SummarizerSummarizeOptions { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for SummarizerSummarizeOptions {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for SummarizerSummarizeOptions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for SummarizerSummarizeOptions {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for SummarizerSummarizeOptions {
    fn as_mut(&mut self) -> &mut Any {
      &mut self.inner
  }
}

impl From<SummarizerSummarizeOptions> for Any {
    fn from(s: SummarizerSummarizeOptions) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&SummarizerSummarizeOptions> for Any {
    fn from(s: &SummarizerSummarizeOptions) -> Any {
        s.inner.clone()
    }
}

impl SummarizerSummarizeOptions {
    /// Getter of the `signal` attribute.
    pub fn signal(&self) -> AbortSignal {
        self.inner.get("signal").as_::<AbortSignal>()
    }

    /// Setter of the `signal` attribute.
    pub fn set_signal(&mut self, value: &AbortSignal) {
        self.inner.set("signal", value);
    }
}
impl SummarizerSummarizeOptions {
    /// Getter of the `context` attribute.
    pub fn context(&self) -> JsString {
        self.inner.get("context").as_::<JsString>()
    }

    /// Setter of the `context` attribute.
    pub fn set_context(&mut self, value: &JsString) {
        self.inner.set("context", value);
    }
}
