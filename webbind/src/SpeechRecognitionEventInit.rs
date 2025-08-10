use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct SpeechRecognitionEventInit {
    inner: Any,
}
impl FromVal for SpeechRecognitionEventInit {
    fn from_val(v: &Any) -> Self {
        SpeechRecognitionEventInit { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for SpeechRecognitionEventInit {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for SpeechRecognitionEventInit {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for SpeechRecognitionEventInit {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for SpeechRecognitionEventInit {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<SpeechRecognitionEventInit> for Any {
    fn from(s: SpeechRecognitionEventInit) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&SpeechRecognitionEventInit> for Any {
    fn from(s: &SpeechRecognitionEventInit) -> Any {
        s.inner.clone()
    }
}

impl SpeechRecognitionEventInit {
    pub fn result_index(&self) -> u32 {
        self.inner.get("resultIndex").as_::<u32>()
    }

    pub fn set_result_index(&mut self, value: u32) {
        self.inner.set("resultIndex", value);
    }
}
impl SpeechRecognitionEventInit {
    pub fn results(&self) -> SpeechRecognitionResultList {
        self.inner
            .get("results")
            .as_::<SpeechRecognitionResultList>()
    }

    pub fn set_results(&mut self, value: &SpeechRecognitionResultList) {
        self.inner.set("results", value);
    }
}
