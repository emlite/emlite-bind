use super::*;

/// The ManagedSourceBuffer class.
/// [`ManagedSourceBuffer`](https://developer.mozilla.org/en-US/docs/Web/API/ManagedSourceBuffer)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct ManagedSourceBuffer {
    inner: SourceBuffer,
}

impl FromVal for ManagedSourceBuffer {
    fn from_val(v: &Any) -> Self {
        ManagedSourceBuffer {
            inner: SourceBuffer::from_val(v),
        }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for ManagedSourceBuffer {
    type Target = SourceBuffer;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for ManagedSourceBuffer {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for ManagedSourceBuffer {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for ManagedSourceBuffer {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}

impl From<ManagedSourceBuffer> for Any {
    fn from(s: ManagedSourceBuffer) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&ManagedSourceBuffer> for Any {
    fn from(s: &ManagedSourceBuffer) -> Any {
        s.inner.clone().into()
    }
}

jsbind::utils::impl_dyn_cast!(ManagedSourceBuffer);

impl ManagedSourceBuffer {
    /// Getter of the `onbufferedchange` attribute.
    /// [`ManagedSourceBuffer.onbufferedchange`](https://developer.mozilla.org/en-US/docs/Web/API/ManagedSourceBuffer/onbufferedchange)
    pub fn onbufferedchange(&self) -> Any {
        self.inner.get("onbufferedchange").as_::<Any>()
    }

    /// Setter of the `onbufferedchange` attribute.
    /// [`ManagedSourceBuffer.onbufferedchange`](https://developer.mozilla.org/en-US/docs/Web/API/ManagedSourceBuffer/onbufferedchange)
    pub fn set_onbufferedchange(&mut self, value: &Any) {
        self.inner.set("onbufferedchange", value);
    }
}
