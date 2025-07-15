use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct RemotePlayback {
    inner: EventTarget,
}
impl FromVal for RemotePlayback {
    fn from_val(v: &emlite::Val) -> Self {
        RemotePlayback {
            inner: EventTarget::from_val(v),
        }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
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
impl AsRef<emlite::Val> for RemotePlayback {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for RemotePlayback {
    fn as_mut(&mut self) -> &mut emlite::Val {
        &mut self.inner
    }
}
impl From<RemotePlayback> for emlite::Val {
    fn from(s: RemotePlayback) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}
impl From<&RemotePlayback> for emlite::Val {
    fn from(s: &RemotePlayback) -> emlite::Val {
        s.inner.clone().into()
    }
}
jsbind::utils::impl_dyn_cast!(RemotePlayback);

impl RemotePlayback {
    pub fn watch_availability(&self, callback: &Function) -> Promise {
        self.inner
            .call("watchAvailability", &[callback.into()])
            .as_::<Promise>()
    }
}
impl RemotePlayback {
    pub fn cancel_watch_availability0(&self) -> Promise {
        self.inner
            .call("cancelWatchAvailability", &[])
            .as_::<Promise>()
    }

    pub fn cancel_watch_availability1(&self, id: i32) -> Promise {
        self.inner
            .call("cancelWatchAvailability", &[id.into()])
            .as_::<Promise>()
    }
}
impl RemotePlayback {
    pub fn state(&self) -> RemotePlaybackState {
        self.inner.get("state").as_::<RemotePlaybackState>()
    }
}
impl RemotePlayback {
    pub fn onconnecting(&self) -> Any {
        self.inner.get("onconnecting").as_::<Any>()
    }

    pub fn set_onconnecting(&mut self, value: &Any) {
        self.inner.set("onconnecting", value);
    }
}
impl RemotePlayback {
    pub fn onconnect(&self) -> Any {
        self.inner.get("onconnect").as_::<Any>()
    }

    pub fn set_onconnect(&mut self, value: &Any) {
        self.inner.set("onconnect", value);
    }
}
impl RemotePlayback {
    pub fn ondisconnect(&self) -> Any {
        self.inner.get("ondisconnect").as_::<Any>()
    }

    pub fn set_ondisconnect(&mut self, value: &Any) {
        self.inner.set("ondisconnect", value);
    }
}
impl RemotePlayback {
    pub fn prompt(&self) -> Promise {
        self.inner.call("prompt", &[]).as_::<Promise>()
    }
}
