use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
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
impl core::ops::Deref for NotificationEvent {
    type Target = ExtendableEvent;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for NotificationEvent {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for NotificationEvent {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for NotificationEvent {
    fn as_mut(&mut self) -> &mut emlite::Val {
        &mut self.inner
    }
}
impl From<NotificationEvent> for emlite::Val {
    fn from(s: NotificationEvent) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}
impl From<&NotificationEvent> for emlite::Val {
    fn from(s: &NotificationEvent) -> emlite::Val {
        s.inner.clone().into()
    }
}
jsbind::utils::impl_dyn_cast!(NotificationEvent);

impl NotificationEvent {
    pub fn new(type_: &str, event_init_dict: &Any) -> NotificationEvent {
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
    pub fn action(&self) -> String {
        self.inner.get("action").as_::<String>()
    }
}
