use super::*;

#[derive(Clone, Debug)]
pub struct WebSocket {
    inner: EventTarget,
}
impl FromVal for WebSocket {
    fn from_val(v: &emlite::Val) -> Self {
        WebSocket {
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
impl std::ops::Deref for WebSocket {
    type Target = EventTarget;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for WebSocket {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<WebSocket> for emlite::Val {
    fn from(s: WebSocket) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl WebSocket {
    pub fn new0(url: jsbind::USVString) -> WebSocket {
        Self {
            inner: emlite::Val::global("WebSocket")
                .new(&[url.into()])
                .as_::<EventTarget>(),
        }
    }

    pub fn new1(url: jsbind::USVString, protocols: jsbind::Any) -> WebSocket {
        Self {
            inner: emlite::Val::global("WebSocket")
                .new(&[url.into(), protocols.into()])
                .as_::<EventTarget>(),
        }
    }
}
impl WebSocket {
    pub fn url(&self) -> jsbind::USVString {
        self.inner.get("url").as_::<jsbind::USVString>()
    }
}
impl WebSocket {
    pub fn ready_state(&self) -> u16 {
        self.inner.get("readyState").as_::<u16>()
    }
}
impl WebSocket {
    pub fn buffered_amount(&self) -> u64 {
        self.inner.get("bufferedAmount").as_::<u64>()
    }
}
impl WebSocket {
    pub fn onopen(&self) -> jsbind::Any {
        self.inner.get("onopen").as_::<jsbind::Any>()
    }

    pub fn set_onopen(&mut self, value: jsbind::Any) {
        self.inner.set("onopen", value);
    }
}
impl WebSocket {
    pub fn onerror(&self) -> jsbind::Any {
        self.inner.get("onerror").as_::<jsbind::Any>()
    }

    pub fn set_onerror(&mut self, value: jsbind::Any) {
        self.inner.set("onerror", value);
    }
}
impl WebSocket {
    pub fn onclose(&self) -> jsbind::Any {
        self.inner.get("onclose").as_::<jsbind::Any>()
    }

    pub fn set_onclose(&mut self, value: jsbind::Any) {
        self.inner.set("onclose", value);
    }
}
impl WebSocket {
    pub fn extensions(&self) -> jsbind::DOMString {
        self.inner.get("extensions").as_::<jsbind::DOMString>()
    }
}
impl WebSocket {
    pub fn protocol(&self) -> jsbind::DOMString {
        self.inner.get("protocol").as_::<jsbind::DOMString>()
    }
}
impl WebSocket {
    pub fn close0(&self) -> jsbind::Undefined {
        self.inner.call("close", &[]).as_::<jsbind::Undefined>()
    }

    pub fn close1(&self, code: u16) -> jsbind::Undefined {
        self.inner
            .call("close", &[code.into()])
            .as_::<jsbind::Undefined>()
    }

    pub fn close2(&self, code: u16, reason: jsbind::USVString) -> jsbind::Undefined {
        self.inner
            .call("close", &[code.into(), reason.into()])
            .as_::<jsbind::Undefined>()
    }
}
impl WebSocket {
    pub fn onmessage(&self) -> jsbind::Any {
        self.inner.get("onmessage").as_::<jsbind::Any>()
    }

    pub fn set_onmessage(&mut self, value: jsbind::Any) {
        self.inner.set("onmessage", value);
    }
}
impl WebSocket {
    pub fn binary_type(&self) -> BinaryType {
        self.inner.get("binaryType").as_::<BinaryType>()
    }

    pub fn set_binary_type(&mut self, value: BinaryType) {
        self.inner.set("binaryType", value);
    }
}
impl WebSocket {
    pub fn send(&self, data: jsbind::Any) -> jsbind::Undefined {
        self.inner
            .call("send", &[data.into()])
            .as_::<jsbind::Undefined>()
    }
}
