use super::*;




/// The RemotePlayback class.
/// [`RemotePlayback`](https://developer.mozilla.org/en-US/docs/Web/API/RemotePlayback)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct RemotePlayback {
    inner: EventTarget,
}

impl FromVal for RemotePlayback {
    fn from_val(v: &Any) -> Self {
        RemotePlayback { inner: EventTarget::from_val(v) }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for RemotePlayback {
    type Target = EventTarget;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for RemotePlayback {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for RemotePlayback {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for RemotePlayback {
    fn as_mut(&mut self) -> &mut Any {
      &mut self.inner
  }
}

impl From<RemotePlayback> for Any {
    fn from(s: RemotePlayback) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&RemotePlayback> for Any {
    fn from(s: &RemotePlayback) -> Any {
        s.inner.clone().into()
    }
}

jsbind::utils::impl_dyn_cast!(RemotePlayback);


impl RemotePlayback {
    /// The watchAvailability method.
    /// [`RemotePlayback.watchAvailability`](https://developer.mozilla.org/en-US/docs/Web/API/RemotePlayback/watchAvailability)
    pub fn watch_availability(&self, callback: &Function) -> Promise<i32> {
        self.inner.call("watchAvailability", &[callback.into(), ]).as_::<Promise<i32>>()
    }
}
impl RemotePlayback {
    /// The cancelWatchAvailability method.
    /// [`RemotePlayback.cancelWatchAvailability`](https://developer.mozilla.org/en-US/docs/Web/API/RemotePlayback/cancelWatchAvailability)
    pub fn cancel_watch_availability0(&self, ) -> Promise<Undefined> {
        self.inner.call("cancelWatchAvailability", &[]).as_::<Promise<Undefined>>()
    }
    /// The cancelWatchAvailability method.
    /// [`RemotePlayback.cancelWatchAvailability`](https://developer.mozilla.org/en-US/docs/Web/API/RemotePlayback/cancelWatchAvailability)
    pub fn cancel_watch_availability1(&self, id: i32) -> Promise<Undefined> {
        self.inner.call("cancelWatchAvailability", &[id.into(), ]).as_::<Promise<Undefined>>()
    }
}
impl RemotePlayback {
    /// Getter of the `state` attribute.
    /// [`RemotePlayback.state`](https://developer.mozilla.org/en-US/docs/Web/API/RemotePlayback/state)
    pub fn state(&self) -> RemotePlaybackState {
        self.inner.get("state").as_::<RemotePlaybackState>()
    }

}
impl RemotePlayback {
    /// Getter of the `onconnecting` attribute.
    /// [`RemotePlayback.onconnecting`](https://developer.mozilla.org/en-US/docs/Web/API/RemotePlayback/onconnecting)
    pub fn onconnecting(&self) -> Any {
        self.inner.get("onconnecting").as_::<Any>()
    }

    /// Setter of the `onconnecting` attribute.
    /// [`RemotePlayback.onconnecting`](https://developer.mozilla.org/en-US/docs/Web/API/RemotePlayback/onconnecting)
    pub fn set_onconnecting(&mut self, value: &Any) {
        self.inner.set("onconnecting", value);
    }
}
impl RemotePlayback {
    /// Getter of the `onconnect` attribute.
    /// [`RemotePlayback.onconnect`](https://developer.mozilla.org/en-US/docs/Web/API/RemotePlayback/onconnect)
    pub fn onconnect(&self) -> Any {
        self.inner.get("onconnect").as_::<Any>()
    }

    /// Setter of the `onconnect` attribute.
    /// [`RemotePlayback.onconnect`](https://developer.mozilla.org/en-US/docs/Web/API/RemotePlayback/onconnect)
    pub fn set_onconnect(&mut self, value: &Any) {
        self.inner.set("onconnect", value);
    }
}
impl RemotePlayback {
    /// Getter of the `ondisconnect` attribute.
    /// [`RemotePlayback.ondisconnect`](https://developer.mozilla.org/en-US/docs/Web/API/RemotePlayback/ondisconnect)
    pub fn ondisconnect(&self) -> Any {
        self.inner.get("ondisconnect").as_::<Any>()
    }

    /// Setter of the `ondisconnect` attribute.
    /// [`RemotePlayback.ondisconnect`](https://developer.mozilla.org/en-US/docs/Web/API/RemotePlayback/ondisconnect)
    pub fn set_ondisconnect(&mut self, value: &Any) {
        self.inner.set("ondisconnect", value);
    }
}
impl RemotePlayback {
    /// The prompt method.
    /// [`RemotePlayback.prompt`](https://developer.mozilla.org/en-US/docs/Web/API/RemotePlayback/prompt)
    pub fn prompt(&self, ) -> Promise<Undefined> {
        self.inner.call("prompt", &[]).as_::<Promise<Undefined>>()
    }
}
