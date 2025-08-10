use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct SpeechSynthesisErrorEventInit {
    inner: Any,
}
impl FromVal for SpeechSynthesisErrorEventInit {
    fn from_val(v: &Any) -> Self {
        SpeechSynthesisErrorEventInit { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for SpeechSynthesisErrorEventInit {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for SpeechSynthesisErrorEventInit {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for SpeechSynthesisErrorEventInit {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for SpeechSynthesisErrorEventInit {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<SpeechSynthesisErrorEventInit> for Any {
    fn from(s: SpeechSynthesisErrorEventInit) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&SpeechSynthesisErrorEventInit> for Any {
    fn from(s: &SpeechSynthesisErrorEventInit) -> Any {
        s.inner.clone()
    }
}

impl SpeechSynthesisErrorEventInit {
    pub fn error(&self) -> SpeechSynthesisErrorCode {
        self.inner.get("error").as_::<SpeechSynthesisErrorCode>()
    }

    pub fn set_error(&mut self, value: &SpeechSynthesisErrorCode) {
        self.inner.set("error", value);
    }
}
