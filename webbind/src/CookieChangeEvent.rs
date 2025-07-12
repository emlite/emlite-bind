use super::*;

#[derive(Clone, Debug)]
pub struct CookieChangeEvent {
    inner: Event,
}
impl FromVal for CookieChangeEvent {
    fn from_val(v: &emlite::Val) -> Self {
        CookieChangeEvent {
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
impl std::ops::Deref for CookieChangeEvent {
    type Target = Event;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for CookieChangeEvent {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<CookieChangeEvent> for emlite::Val {
    fn from(s: CookieChangeEvent) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl CookieChangeEvent {
    pub fn new0(type_: jsbind::DOMString) -> CookieChangeEvent {
        Self {
            inner: emlite::Val::global("CookieChangeEvent")
                .new(&[type_.into()])
                .as_::<Event>(),
        }
    }

    pub fn new1(type_: jsbind::DOMString, event_init_dict: jsbind::Any) -> CookieChangeEvent {
        Self {
            inner: emlite::Val::global("CookieChangeEvent")
                .new(&[type_.into(), event_init_dict.into()])
                .as_::<Event>(),
        }
    }
}
impl CookieChangeEvent {
    pub fn changed(&self) -> jsbind::FrozenArray<CookieListItem> {
        self.inner
            .get("changed")
            .as_::<jsbind::FrozenArray<CookieListItem>>()
    }
}
impl CookieChangeEvent {
    pub fn deleted(&self) -> jsbind::FrozenArray<CookieListItem> {
        self.inner
            .get("deleted")
            .as_::<jsbind::FrozenArray<CookieListItem>>()
    }
}
