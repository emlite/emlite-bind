use super::*;

#[derive(Clone, Debug)]
pub struct NotificationEvent {
    inner: ExtendableEvent,
}
impl FromVal for NotificationEvent {
    fn from_val(v: &emlite::Val) -> Self {
        NotificationEvent {
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
impl std::ops::Deref for NotificationEvent {
    type Target = ExtendableEvent;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for NotificationEvent {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<NotificationEvent> for emlite::Val {
    fn from(s: NotificationEvent) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl NotificationEvent {
    pub fn new(type_: jsbind::DOMString, event_init_dict: jsbind::Any) -> NotificationEvent {
        Self {
            inner: emlite::Val::global("NotificationEvent")
                .new(&[type_.into(), event_init_dict.into()])
                .as_::<ExtendableEvent>(),
        }
    }
}
impl NotificationEvent {
    pub fn notification(&self) -> Notification {
        self.inner.get("notification").as_::<Notification>()
    }
}
impl NotificationEvent {
    pub fn action(&self) -> jsbind::DOMString {
        self.inner.get("action").as_::<jsbind::DOMString>()
    }
}
