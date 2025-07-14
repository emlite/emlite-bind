use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct PushSubscriptionChangeEvent {
    inner: ExtendableEvent,
}
impl FromVal for PushSubscriptionChangeEvent {
    fn from_val(v: &emlite::Val) -> Self {
        PushSubscriptionChangeEvent {
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
impl core::ops::Deref for PushSubscriptionChangeEvent {
    type Target = ExtendableEvent;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for PushSubscriptionChangeEvent {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<PushSubscriptionChangeEvent> for emlite::Val {
    fn from(s: PushSubscriptionChangeEvent) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl PushSubscriptionChangeEvent {
    pub fn new0(type_: jsbind::DOMString) -> PushSubscriptionChangeEvent {
        Self {
            inner: emlite::Val::global("PushSubscriptionChangeEvent")
                .new(&[type_.into()])
                .as_::<ExtendableEvent>(),
        }
    }

    pub fn new1(
        type_: jsbind::DOMString,
        event_init_dict: jsbind::Any,
    ) -> PushSubscriptionChangeEvent {
        Self {
            inner: emlite::Val::global("PushSubscriptionChangeEvent")
                .new(&[type_.into(), event_init_dict.into()])
                .as_::<ExtendableEvent>(),
        }
    }
}
impl PushSubscriptionChangeEvent {
    pub fn new_subscription(&self) -> PushSubscription {
        self.inner.get("newSubscription").as_::<PushSubscription>()
    }
}
impl PushSubscriptionChangeEvent {
    pub fn old_subscription(&self) -> PushSubscription {
        self.inner.get("oldSubscription").as_::<PushSubscription>()
    }
}
