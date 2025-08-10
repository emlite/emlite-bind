use super::*;

/// The AudioSession class.
/// [`AudioSession`](https://developer.mozilla.org/en-US/docs/Web/API/AudioSession)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct AudioSession {
    inner: EventTarget,
}

impl FromVal for AudioSession {
    fn from_val(v: &Any) -> Self {
        AudioSession {
            inner: EventTarget::from_val(v),
        }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for AudioSession {
    type Target = EventTarget;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for AudioSession {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for AudioSession {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for AudioSession {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}

impl From<AudioSession> for Any {
    fn from(s: AudioSession) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&AudioSession> for Any {
    fn from(s: &AudioSession) -> Any {
        s.inner.clone().into()
    }
}

jsbind::utils::impl_dyn_cast!(AudioSession);

impl AudioSession {
    /// Getter of the `type` attribute.
    /// [`AudioSession.type`](https://developer.mozilla.org/en-US/docs/Web/API/AudioSession/type)
    pub fn type_(&self) -> AudioSessionType {
        self.inner.get("type").as_::<AudioSessionType>()
    }

    /// Setter of the `type` attribute.
    /// [`AudioSession.type`](https://developer.mozilla.org/en-US/docs/Web/API/AudioSession/type)
    pub fn set_type_(&mut self, value: &AudioSessionType) {
        self.inner.set("type", value);
    }
}
impl AudioSession {
    /// Getter of the `state` attribute.
    /// [`AudioSession.state`](https://developer.mozilla.org/en-US/docs/Web/API/AudioSession/state)
    pub fn state(&self) -> AudioSessionState {
        self.inner.get("state").as_::<AudioSessionState>()
    }
}
impl AudioSession {
    /// Getter of the `onstatechange` attribute.
    /// [`AudioSession.onstatechange`](https://developer.mozilla.org/en-US/docs/Web/API/AudioSession/onstatechange)
    pub fn onstatechange(&self) -> Any {
        self.inner.get("onstatechange").as_::<Any>()
    }

    /// Setter of the `onstatechange` attribute.
    /// [`AudioSession.onstatechange`](https://developer.mozilla.org/en-US/docs/Web/API/AudioSession/onstatechange)
    pub fn set_onstatechange(&mut self, value: &Any) {
        self.inner.set("onstatechange", value);
    }
}
