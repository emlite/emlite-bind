use super::*;

#[derive(Clone, Debug)]
pub struct PresentationConnection {
    inner: EventTarget,
}
impl FromVal for PresentationConnection {
    fn from_val(v: &emlite::Val) -> Self {
        PresentationConnection {
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
impl std::ops::Deref for PresentationConnection {
    type Target = EventTarget;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for PresentationConnection {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<PresentationConnection> for emlite::Val {
    fn from(s: PresentationConnection) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl PresentationConnection {
    pub fn id(&self) -> jsbind::USVString {
        self.inner.get("id").as_::<jsbind::USVString>()
    }
}
impl PresentationConnection {
    pub fn url(&self) -> jsbind::USVString {
        self.inner.get("url").as_::<jsbind::USVString>()
    }
}
impl PresentationConnection {
    pub fn state(&self) -> PresentationConnectionState {
        self.inner.get("state").as_::<PresentationConnectionState>()
    }
}
impl PresentationConnection {
    pub fn close(&self) -> jsbind::Undefined {
        self.inner.call("close", &[]).as_::<jsbind::Undefined>()
    }
}
impl PresentationConnection {
    pub fn terminate(&self) -> jsbind::Undefined {
        self.inner.call("terminate", &[]).as_::<jsbind::Undefined>()
    }
}
impl PresentationConnection {
    pub fn onconnect(&self) -> jsbind::Any {
        self.inner.get("onconnect").as_::<jsbind::Any>()
    }

    pub fn set_onconnect(&mut self, value: jsbind::Any) {
        self.inner.set("onconnect", value);
    }
}
impl PresentationConnection {
    pub fn onclose(&self) -> jsbind::Any {
        self.inner.get("onclose").as_::<jsbind::Any>()
    }

    pub fn set_onclose(&mut self, value: jsbind::Any) {
        self.inner.set("onclose", value);
    }
}
impl PresentationConnection {
    pub fn onterminate(&self) -> jsbind::Any {
        self.inner.get("onterminate").as_::<jsbind::Any>()
    }

    pub fn set_onterminate(&mut self, value: jsbind::Any) {
        self.inner.set("onterminate", value);
    }
}
impl PresentationConnection {
    pub fn binary_type(&self) -> BinaryType {
        self.inner.get("binaryType").as_::<BinaryType>()
    }

    pub fn set_binary_type(&mut self, value: BinaryType) {
        self.inner.set("binaryType", value);
    }
}
impl PresentationConnection {
    pub fn onmessage(&self) -> jsbind::Any {
        self.inner.get("onmessage").as_::<jsbind::Any>()
    }

    pub fn set_onmessage(&mut self, value: jsbind::Any) {
        self.inner.set("onmessage", value);
    }
}
impl PresentationConnection {
    pub fn send(&self, data: jsbind::Any) -> jsbind::Undefined {
        self.inner
            .call("send", &[data.into()])
            .as_::<jsbind::Undefined>()
    }
}
