use super::*;




/// The SpeechRecognitionResultList class.
/// [`SpeechRecognitionResultList`](https://developer.mozilla.org/en-US/docs/Web/API/SpeechRecognitionResultList)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct SpeechRecognitionResultList {
    inner: Any,
}

impl FromVal for SpeechRecognitionResultList {
    fn from_val(v: &Any) -> Self {
        SpeechRecognitionResultList { inner: Any::from_val(v) }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for SpeechRecognitionResultList {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for SpeechRecognitionResultList {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for SpeechRecognitionResultList {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for SpeechRecognitionResultList {
    fn as_mut(&mut self) -> &mut Any {
      &mut self.inner
  }
}

impl From<SpeechRecognitionResultList> for Any {
    fn from(s: SpeechRecognitionResultList) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&SpeechRecognitionResultList> for Any {
    fn from(s: &SpeechRecognitionResultList) -> Any {
        s.inner.clone().into()
    }
}

jsbind::utils::impl_dyn_cast!(SpeechRecognitionResultList);


impl SpeechRecognitionResultList {
    /// Getter of the `length` attribute.
    /// [`SpeechRecognitionResultList.length`](https://developer.mozilla.org/en-US/docs/Web/API/SpeechRecognitionResultList/length)
    pub fn length(&self) -> u32 {
        self.inner.get("length").as_::<u32>()
    }

}
impl SpeechRecognitionResultList {
    /// The item method.
    /// [`SpeechRecognitionResultList.item`](https://developer.mozilla.org/en-US/docs/Web/API/SpeechRecognitionResultList/item)
    pub fn item(&self, index: u32) -> SpeechRecognitionResult {
        self.inner.call("item", &[index.into(), ]).as_::<SpeechRecognitionResult>()
    }
}
