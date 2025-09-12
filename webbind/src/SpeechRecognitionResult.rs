use super::*;

/// The SpeechRecognitionResult class.
/// [`SpeechRecognitionResult`](https://developer.mozilla.org/en-US/docs/Web/API/SpeechRecognitionResult)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct SpeechRecognitionResult {
    inner: Any,
}

impl FromVal for SpeechRecognitionResult {
    fn from_val(v: &Any) -> Self {
        SpeechRecognitionResult {
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

impl core::ops::Deref for SpeechRecognitionResult {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for SpeechRecognitionResult {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for SpeechRecognitionResult {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for SpeechRecognitionResult {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}

impl From<SpeechRecognitionResult> for Any {
    fn from(s: SpeechRecognitionResult) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&SpeechRecognitionResult> for Any {
    fn from(s: &SpeechRecognitionResult) -> Any {
        s.inner.clone().into()
    }
}

jsbind::utils::impl_dyn_cast!(SpeechRecognitionResult);

impl SpeechRecognitionResult {
    /// Getter of the `length` attribute.
    /// [`SpeechRecognitionResult.length`](https://developer.mozilla.org/en-US/docs/Web/API/SpeechRecognitionResult/length)
    pub fn length(&self) -> u32 {
        self.inner.get("length").as_::<u32>()
    }
}
impl SpeechRecognitionResult {
    /// Getter of the `isFinal` attribute.
    /// [`SpeechRecognitionResult.isFinal`](https://developer.mozilla.org/en-US/docs/Web/API/SpeechRecognitionResult/isFinal)
    pub fn is_final(&self) -> bool {
        self.inner.get("isFinal").as_::<bool>()
    }
}
impl SpeechRecognitionResult {
    /// The item method.
    /// [`SpeechRecognitionResult.item`](https://developer.mozilla.org/en-US/docs/Web/API/SpeechRecognitionResult/item)
    pub fn item(&self, index: u32) -> SpeechRecognitionAlternative {
        self.inner
            .call("item", &[index.into()])
            .as_::<SpeechRecognitionAlternative>()
    }
}
