use super::*;




#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct MediaKeySession {
    inner: EventTarget,
}
impl FromVal for MediaKeySession {
    fn from_val(v: &emlite::Val) -> Self {
        MediaKeySession { inner: EventTarget::from_val(v) }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
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
impl AsRef<emlite::Val> for MediaKeySession {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for MediaKeySession {
    fn as_mut(&mut self) -> &mut emlite::Val {
      &mut self.inner
  }
}
impl From<MediaKeySession> for emlite::Val {
    fn from(s: MediaKeySession) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}
jsbind::utils::impl_dyn_cast!(MediaKeySession);


impl MediaKeySession {
    pub fn session_id(&self) -> DOMString {
        self.inner.get("sessionId").as_::<DOMString>()
    }

}
impl MediaKeySession {
    pub fn expiration(&self) -> f64 {
        self.inner.get("expiration").as_::<f64>()
    }

}
impl MediaKeySession {
    pub fn closed(&self) -> Promise {
        self.inner.get("closed").as_::<Promise>()
    }

}
impl MediaKeySession {
    pub fn key_statuses(&self) -> MediaKeyStatusMap {
        self.inner.get("keyStatuses").as_::<MediaKeyStatusMap>()
    }

}
impl MediaKeySession {
    pub fn onkeystatuseschange(&self) -> Any {
        self.inner.get("onkeystatuseschange").as_::<Any>()
    }

    pub fn set_onkeystatuseschange(&mut self, value: Any) {
        self.inner.set("onkeystatuseschange", value);
    }

}
impl MediaKeySession {
    pub fn onmessage(&self) -> Any {
        self.inner.get("onmessage").as_::<Any>()
    }

    pub fn set_onmessage(&mut self, value: Any) {
        self.inner.set("onmessage", value);
    }

}
impl MediaKeySession {
    pub fn generate_request(&self, init_data_type: DOMString, init_data: Any) -> Promise {
        self.inner.call("generateRequest", &[init_data_type.into(), init_data.into(), ]).as_::<Promise>()
    }

}
impl MediaKeySession {
    pub fn load(&self, session_id: DOMString) -> Promise {
        self.inner.call("load", &[session_id.into(), ]).as_::<Promise>()
    }

}
impl MediaKeySession {
    pub fn update(&self, response: Any) -> Promise {
        self.inner.call("update", &[response.into(), ]).as_::<Promise>()
    }

}
impl MediaKeySession {
    pub fn close(&self, ) -> Promise {
        self.inner.call("close", &[]).as_::<Promise>()
    }

}
impl MediaKeySession {
    pub fn remove(&self, ) -> Promise {
        self.inner.call("remove", &[]).as_::<Promise>()
    }

}
