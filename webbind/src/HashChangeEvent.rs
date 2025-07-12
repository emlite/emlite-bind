use super::*;

#[derive(Clone, Debug)]
pub struct HashChangeEvent {
    inner: Event,
}
impl FromVal for HashChangeEvent {
    fn from_val(v: &emlite::Val) -> Self {
        HashChangeEvent {
            inner: Event::from_val(v),
        }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl std::ops::Deref for HashChangeEvent {
    type Target = Event;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for HashChangeEvent {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<HashChangeEvent> for emlite::Val {
    fn from(s: HashChangeEvent) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl HashChangeEvent {
    pub fn new0(type_: jsbind::DOMString) -> HashChangeEvent {
        Self {
            inner: emlite::Val::global("HashChangeEvent")
                .new(&[type_.into()])
                .as_::<Event>(),
        }
    }

    pub fn new1(type_: jsbind::DOMString, event_init_dict: jsbind::Any) -> HashChangeEvent {
        Self {
            inner: emlite::Val::global("HashChangeEvent")
                .new(&[type_.into(), event_init_dict.into()])
                .as_::<Event>(),
        }
    }
}
impl HashChangeEvent {
    pub fn old_url(&self) -> jsbind::USVString {
        self.inner.get("oldURL").as_::<jsbind::USVString>()
    }
}
impl HashChangeEvent {
    pub fn new_url(&self) -> jsbind::USVString {
        self.inner.get("newURL").as_::<jsbind::USVString>()
    }
}
