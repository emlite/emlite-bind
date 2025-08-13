use super::*;




/// The StreamPipeOptions dictionary.
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct StreamPipeOptions {
    inner: Any,
}

impl FromVal for StreamPipeOptions {
    fn from_val(v: &Any) -> Self {
        StreamPipeOptions { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for StreamPipeOptions {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for StreamPipeOptions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for StreamPipeOptions {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for StreamPipeOptions {
    fn as_mut(&mut self) -> &mut Any {
      &mut self.inner
  }
}

impl From<StreamPipeOptions> for Any {
    fn from(s: StreamPipeOptions) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&StreamPipeOptions> for Any {
    fn from(s: &StreamPipeOptions) -> Any {
        s.inner.clone()
    }
}

impl StreamPipeOptions {
    /// Getter of the `preventClose` attribute.
    pub fn prevent_close(&self) -> bool {
        self.inner.get("preventClose").as_::<bool>()
    }

    /// Setter of the `preventClose` attribute.
    pub fn set_prevent_close(&mut self, value: bool) {
        self.inner.set("preventClose", value);
    }
}
impl StreamPipeOptions {
    /// Getter of the `preventAbort` attribute.
    pub fn prevent_abort(&self) -> bool {
        self.inner.get("preventAbort").as_::<bool>()
    }

    /// Setter of the `preventAbort` attribute.
    pub fn set_prevent_abort(&mut self, value: bool) {
        self.inner.set("preventAbort", value);
    }
}
impl StreamPipeOptions {
    /// Getter of the `preventCancel` attribute.
    pub fn prevent_cancel(&self) -> bool {
        self.inner.get("preventCancel").as_::<bool>()
    }

    /// Setter of the `preventCancel` attribute.
    pub fn set_prevent_cancel(&mut self, value: bool) {
        self.inner.set("preventCancel", value);
    }
}
impl StreamPipeOptions {
    /// Getter of the `signal` attribute.
    pub fn signal(&self) -> AbortSignal {
        self.inner.get("signal").as_::<AbortSignal>()
    }

    /// Setter of the `signal` attribute.
    pub fn set_signal(&mut self, value: &AbortSignal) {
        self.inner.set("signal", value);
    }
}
