use super::*;




/// The SpeechRecognitionOptions dictionary.
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct SpeechRecognitionOptions {
    inner: Any,
}

impl FromVal for SpeechRecognitionOptions {
    fn from_val(v: &Any) -> Self {
        SpeechRecognitionOptions { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for SpeechRecognitionOptions {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for SpeechRecognitionOptions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for SpeechRecognitionOptions {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for SpeechRecognitionOptions {
    fn as_mut(&mut self) -> &mut Any {
      &mut self.inner
  }
}

impl From<SpeechRecognitionOptions> for Any {
    fn from(s: SpeechRecognitionOptions) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&SpeechRecognitionOptions> for Any {
    fn from(s: &SpeechRecognitionOptions) -> Any {
        s.inner.clone()
    }
}

impl SpeechRecognitionOptions {
    /// Getter of the `langs` attribute.
    pub fn langs(&self) -> TypedArray<JsString> {
        self.inner.get("langs").as_::<TypedArray<JsString>>()
    }

    /// Setter of the `langs` attribute.
    pub fn set_langs(&mut self, value: &TypedArray<JsString>) {
        self.inner.set("langs", value);
    }
}
impl SpeechRecognitionOptions {
    /// Getter of the `processLocally` attribute.
    pub fn process_locally(&self) -> bool {
        self.inner.get("processLocally").as_::<bool>()
    }

    /// Setter of the `processLocally` attribute.
    pub fn set_process_locally(&mut self, value: bool) {
        self.inner.set("processLocally", value);
    }
}
