use super::*;

#[derive(Clone, Debug)]
pub struct ContentIndexEvent {
    inner: ExtendableEvent,
}
impl FromVal for ContentIndexEvent {
    fn from_val(v: &emlite::Val) -> Self {
        ContentIndexEvent {
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
impl std::ops::Deref for ContentIndexEvent {
    type Target = ExtendableEvent;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for ContentIndexEvent {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<ContentIndexEvent> for emlite::Val {
    fn from(s: ContentIndexEvent) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl ContentIndexEvent {
    pub fn new(type_: jsbind::DOMString, init: jsbind::Any) -> ContentIndexEvent {
        Self {
            inner: emlite::Val::global("ContentIndexEvent")
                .new(&[type_.into(), init.into()])
                .as_::<ExtendableEvent>(),
        }
    }
}
impl ContentIndexEvent {
    pub fn id(&self) -> jsbind::DOMString {
        self.inner.get("id").as_::<jsbind::DOMString>()
    }
}
