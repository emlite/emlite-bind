use super::*;

#[derive(Clone, Debug)]
pub struct ExtendableCookieChangeEvent {
    inner: ExtendableEvent,
}
impl FromVal for ExtendableCookieChangeEvent {
    fn from_val(v: &emlite::Val) -> Self {
        ExtendableCookieChangeEvent {
            inner: ExtendableEvent::from_val(v),
        }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl std::ops::Deref for ExtendableCookieChangeEvent {
    type Target = ExtendableEvent;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for ExtendableCookieChangeEvent {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<ExtendableCookieChangeEvent> for emlite::Val {
    fn from(s: ExtendableCookieChangeEvent) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl ExtendableCookieChangeEvent {
    pub fn new0(type_: jsbind::DOMString) -> ExtendableCookieChangeEvent {
        Self {
            inner: emlite::Val::global("ExtendableCookieChangeEvent")
                .new(&[type_.into()])
                .as_::<ExtendableEvent>(),
        }
    }

    pub fn new1(
        type_: jsbind::DOMString,
        event_init_dict: jsbind::Any,
    ) -> ExtendableCookieChangeEvent {
        Self {
            inner: emlite::Val::global("ExtendableCookieChangeEvent")
                .new(&[type_.into(), event_init_dict.into()])
                .as_::<ExtendableEvent>(),
        }
    }
}
impl ExtendableCookieChangeEvent {
    pub fn changed(&self) -> jsbind::FrozenArray<CookieListItem> {
        self.inner
            .get("changed")
            .as_::<jsbind::FrozenArray<CookieListItem>>()
    }
}
impl ExtendableCookieChangeEvent {
    pub fn deleted(&self) -> jsbind::FrozenArray<CookieListItem> {
        self.inner
            .get("deleted")
            .as_::<jsbind::FrozenArray<CookieListItem>>()
    }
}
