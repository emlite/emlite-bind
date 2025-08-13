use super::*;




/// The OfflineAudioCompletionEventInit dictionary.
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct OfflineAudioCompletionEventInit {
    inner: Any,
}

impl FromVal for OfflineAudioCompletionEventInit {
    fn from_val(v: &Any) -> Self {
        OfflineAudioCompletionEventInit { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for OfflineAudioCompletionEventInit {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for OfflineAudioCompletionEventInit {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for OfflineAudioCompletionEventInit {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for OfflineAudioCompletionEventInit {
    fn as_mut(&mut self) -> &mut Any {
      &mut self.inner
  }
}

impl From<OfflineAudioCompletionEventInit> for Any {
    fn from(s: OfflineAudioCompletionEventInit) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&OfflineAudioCompletionEventInit> for Any {
    fn from(s: &OfflineAudioCompletionEventInit) -> Any {
        s.inner.clone()
    }
}

impl OfflineAudioCompletionEventInit {
    /// Getter of the `renderedBuffer` attribute.
    pub fn rendered_buffer(&self) -> AudioBuffer {
        self.inner.get("renderedBuffer").as_::<AudioBuffer>()
    }

    /// Setter of the `renderedBuffer` attribute.
    pub fn set_rendered_buffer(&mut self, value: &AudioBuffer) {
        self.inner.set("renderedBuffer", value);
    }
}
