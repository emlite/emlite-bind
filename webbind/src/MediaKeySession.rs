use super::*;

/// The MediaKeySession class.
/// [`MediaKeySession`](https://developer.mozilla.org/en-US/docs/Web/API/MediaKeySession)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct MediaKeySession {
    inner: EventTarget,
}
impl FromVal for MediaKeySession {
    fn from_val(v: &Any) -> Self {
        MediaKeySession {
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
impl core::ops::Deref for MediaKeySession {
    type Target = EventTarget;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for MediaKeySession {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for MediaKeySession {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for MediaKeySession {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<MediaKeySession> for Any {
    fn from(s: MediaKeySession) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&MediaKeySession> for Any {
    fn from(s: &MediaKeySession) -> Any {
        s.inner.clone().into()
    }
}
jsbind::utils::impl_dyn_cast!(MediaKeySession);

impl MediaKeySession {
    /// Getter of the `sessionId` attribute.
    /// [`MediaKeySession.sessionId`](https://developer.mozilla.org/en-US/docs/Web/API/MediaKeySession/sessionId)
    pub fn session_id(&self) -> JsString {
        self.inner.get("sessionId").as_::<JsString>()
    }
}
impl MediaKeySession {
    /// Getter of the `expiration` attribute.
    /// [`MediaKeySession.expiration`](https://developer.mozilla.org/en-US/docs/Web/API/MediaKeySession/expiration)
    pub fn expiration(&self) -> f64 {
        self.inner.get("expiration").as_::<f64>()
    }
}
impl MediaKeySession {
    /// Getter of the `closed` attribute.
    /// [`MediaKeySession.closed`](https://developer.mozilla.org/en-US/docs/Web/API/MediaKeySession/closed)
    pub fn closed(&self) -> Promise<MediaKeySessionClosedReason> {
        self.inner
            .get("closed")
            .as_::<Promise<MediaKeySessionClosedReason>>()
    }
}
impl MediaKeySession {
    /// Getter of the `keyStatuses` attribute.
    /// [`MediaKeySession.keyStatuses`](https://developer.mozilla.org/en-US/docs/Web/API/MediaKeySession/keyStatuses)
    pub fn key_statuses(&self) -> MediaKeyStatusMap {
        self.inner.get("keyStatuses").as_::<MediaKeyStatusMap>()
    }
}
impl MediaKeySession {
    /// Getter of the `onkeystatuseschange` attribute.
    /// [`MediaKeySession.onkeystatuseschange`](https://developer.mozilla.org/en-US/docs/Web/API/MediaKeySession/onkeystatuseschange)
    pub fn onkeystatuseschange(&self) -> Any {
        self.inner.get("onkeystatuseschange").as_::<Any>()
    }

    /// Setter of the `onkeystatuseschange` attribute.
    /// [`MediaKeySession.onkeystatuseschange`](https://developer.mozilla.org/en-US/docs/Web/API/MediaKeySession/onkeystatuseschange)
    pub fn set_onkeystatuseschange(&mut self, value: &Any) {
        self.inner.set("onkeystatuseschange", value);
    }
}
impl MediaKeySession {
    /// Getter of the `onmessage` attribute.
    /// [`MediaKeySession.onmessage`](https://developer.mozilla.org/en-US/docs/Web/API/MediaKeySession/onmessage)
    pub fn onmessage(&self) -> Any {
        self.inner.get("onmessage").as_::<Any>()
    }

    /// Setter of the `onmessage` attribute.
    /// [`MediaKeySession.onmessage`](https://developer.mozilla.org/en-US/docs/Web/API/MediaKeySession/onmessage)
    pub fn set_onmessage(&mut self, value: &Any) {
        self.inner.set("onmessage", value);
    }
}
impl MediaKeySession {
    /// The generateRequest method.
    /// [`MediaKeySession.generateRequest`](https://developer.mozilla.org/en-US/docs/Web/API/MediaKeySession/generateRequest)
    pub fn generate_request(
        &self,
        init_data_type: &JsString,
        init_data: &Any,
    ) -> Promise<Undefined> {
        self.inner
            .call(
                "generateRequest",
                &[init_data_type.into(), init_data.into()],
            )
            .as_::<Promise<Undefined>>()
    }
}
impl MediaKeySession {
    /// The load method.
    /// [`MediaKeySession.load`](https://developer.mozilla.org/en-US/docs/Web/API/MediaKeySession/load)
    pub fn load(&self, session_id: &JsString) -> Promise<bool> {
        self.inner
            .call("load", &[session_id.into()])
            .as_::<Promise<bool>>()
    }
}
impl MediaKeySession {
    /// The update method.
    /// [`MediaKeySession.update`](https://developer.mozilla.org/en-US/docs/Web/API/MediaKeySession/update)
    pub fn update(&self, response: &Any) -> Promise<Undefined> {
        self.inner
            .call("update", &[response.into()])
            .as_::<Promise<Undefined>>()
    }
}
impl MediaKeySession {
    /// The close method.
    /// [`MediaKeySession.close`](https://developer.mozilla.org/en-US/docs/Web/API/MediaKeySession/close)
    pub fn close(&self) -> Promise<Undefined> {
        self.inner.call("close", &[]).as_::<Promise<Undefined>>()
    }
}
impl MediaKeySession {
    /// The remove method.
    /// [`MediaKeySession.remove`](https://developer.mozilla.org/en-US/docs/Web/API/MediaKeySession/remove)
    pub fn remove(&self) -> Promise<Undefined> {
        self.inner.call("remove", &[]).as_::<Promise<Undefined>>()
    }
}
