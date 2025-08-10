use super::*;

/// The AudioWorklet class.
/// [`AudioWorklet`](https://developer.mozilla.org/en-US/docs/Web/API/AudioWorklet)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct AudioWorklet {
    inner: Worklet,
}

impl FromVal for AudioWorklet {
    fn from_val(v: &Any) -> Self {
        AudioWorklet {
            inner: Worklet::from_val(v),
        }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for AudioWorklet {
    type Target = Worklet;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for AudioWorklet {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for AudioWorklet {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for AudioWorklet {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}

impl From<AudioWorklet> for Any {
    fn from(s: AudioWorklet) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&AudioWorklet> for Any {
    fn from(s: &AudioWorklet) -> Any {
        s.inner.clone().into()
    }
}

jsbind::utils::impl_dyn_cast!(AudioWorklet);

impl AudioWorklet {
    /// Getter of the `port` attribute.
    /// [`AudioWorklet.port`](https://developer.mozilla.org/en-US/docs/Web/API/AudioWorklet/port)
    pub fn port(&self) -> MessagePort {
        self.inner.get("port").as_::<MessagePort>()
    }
}
