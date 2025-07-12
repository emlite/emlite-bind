use super::*;

#[derive(Clone, Debug)]
pub struct EventSource {
    inner: EventTarget,
}
impl FromVal for EventSource {
    fn from_val(v: &emlite::Val) -> Self {
        EventSource {
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
impl std::ops::Deref for EventSource {
    type Target = EventTarget;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for EventSource {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<EventSource> for emlite::Val {
    fn from(s: EventSource) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl EventSource {
    pub fn new0(url: jsbind::USVString) -> EventSource {
        Self {
            inner: emlite::Val::global("EventSource")
                .new(&[url.into()])
                .as_::<EventTarget>(),
        }
    }

    pub fn new1(url: jsbind::USVString, event_source_init_dict: jsbind::Any) -> EventSource {
        Self {
            inner: emlite::Val::global("EventSource")
                .new(&[url.into(), event_source_init_dict.into()])
                .as_::<EventTarget>(),
        }
    }
}
impl EventSource {
    pub fn url(&self) -> jsbind::USVString {
        self.inner.get("url").as_::<jsbind::USVString>()
    }
}
impl EventSource {
    pub fn with_credentials(&self) -> bool {
        self.inner.get("withCredentials").as_::<bool>()
    }
}
impl EventSource {
    pub fn ready_state(&self) -> u16 {
        self.inner.get("readyState").as_::<u16>()
    }
}
impl EventSource {
    pub fn onopen(&self) -> jsbind::Any {
        self.inner.get("onopen").as_::<jsbind::Any>()
    }

    pub fn set_onopen(&mut self, value: jsbind::Any) {
        self.inner.set("onopen", value);
    }
}
impl EventSource {
    pub fn onmessage(&self) -> jsbind::Any {
        self.inner.get("onmessage").as_::<jsbind::Any>()
    }

    pub fn set_onmessage(&mut self, value: jsbind::Any) {
        self.inner.set("onmessage", value);
    }
}
impl EventSource {
    pub fn onerror(&self) -> jsbind::Any {
        self.inner.get("onerror").as_::<jsbind::Any>()
    }

    pub fn set_onerror(&mut self, value: jsbind::Any) {
        self.inner.set("onerror", value);
    }
}
impl EventSource {
    pub fn close(&self) -> jsbind::Undefined {
        self.inner.call("close", &[]).as_::<jsbind::Undefined>()
    }
}
