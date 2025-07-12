use super::*;

#[derive(Clone, Debug)]
pub struct MediaKeySession {
    inner: EventTarget,
}
impl FromVal for MediaKeySession {
    fn from_val(v: &emlite::Val) -> Self {
        MediaKeySession {
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
impl std::ops::Deref for MediaKeySession {
    type Target = EventTarget;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for MediaKeySession {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<MediaKeySession> for emlite::Val {
    fn from(s: MediaKeySession) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl MediaKeySession {
    pub fn session_id(&self) -> jsbind::DOMString {
        self.inner.get("sessionId").as_::<jsbind::DOMString>()
    }
}
impl MediaKeySession {
    pub fn expiration(&self) -> f64 {
        self.inner.get("expiration").as_::<f64>()
    }
}
impl MediaKeySession {
    pub fn closed(&self) -> jsbind::Promise {
        self.inner.get("closed").as_::<jsbind::Promise>()
    }
}
impl MediaKeySession {
    pub fn key_statuses(&self) -> MediaKeyStatusMap {
        self.inner.get("keyStatuses").as_::<MediaKeyStatusMap>()
    }
}
impl MediaKeySession {
    pub fn onkeystatuseschange(&self) -> jsbind::Any {
        self.inner.get("onkeystatuseschange").as_::<jsbind::Any>()
    }

    pub fn set_onkeystatuseschange(&mut self, value: jsbind::Any) {
        self.inner.set("onkeystatuseschange", value);
    }
}
impl MediaKeySession {
    pub fn onmessage(&self) -> jsbind::Any {
        self.inner.get("onmessage").as_::<jsbind::Any>()
    }

    pub fn set_onmessage(&mut self, value: jsbind::Any) {
        self.inner.set("onmessage", value);
    }
}
impl MediaKeySession {
    pub fn generate_request(
        &self,
        init_data_type: jsbind::DOMString,
        init_data: jsbind::Any,
    ) -> jsbind::Promise {
        self.inner
            .call(
                "generateRequest",
                &[init_data_type.into(), init_data.into()],
            )
            .as_::<jsbind::Promise>()
    }
}
impl MediaKeySession {
    pub fn load(&self, session_id: jsbind::DOMString) -> jsbind::Promise {
        self.inner
            .call("load", &[session_id.into()])
            .as_::<jsbind::Promise>()
    }
}
impl MediaKeySession {
    pub fn update(&self, response: jsbind::Any) -> jsbind::Promise {
        self.inner
            .call("update", &[response.into()])
            .as_::<jsbind::Promise>()
    }
}
impl MediaKeySession {
    pub fn close(&self) -> jsbind::Promise {
        self.inner.call("close", &[]).as_::<jsbind::Promise>()
    }
}
impl MediaKeySession {
    pub fn remove(&self) -> jsbind::Promise {
        self.inner.call("remove", &[]).as_::<jsbind::Promise>()
    }
}
