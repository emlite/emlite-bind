use super::*;

#[derive(Clone, Debug)]
pub struct CustomEvent {
    inner: Event,
}
impl FromVal for CustomEvent {
    fn from_val(v: &emlite::Val) -> Self {
        CustomEvent {
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
impl std::ops::Deref for CustomEvent {
    type Target = Event;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for CustomEvent {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<CustomEvent> for emlite::Val {
    fn from(s: CustomEvent) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl CustomEvent {
    pub fn new0(type_: jsbind::DOMString) -> CustomEvent {
        Self {
            inner: emlite::Val::global("CustomEvent")
                .new(&[type_.into()])
                .as_::<Event>(),
        }
    }

    pub fn new1(type_: jsbind::DOMString, event_init_dict: jsbind::Any) -> CustomEvent {
        Self {
            inner: emlite::Val::global("CustomEvent")
                .new(&[type_.into(), event_init_dict.into()])
                .as_::<Event>(),
        }
    }
}
impl CustomEvent {
    pub fn detail(&self) -> jsbind::Any {
        self.inner.get("detail").as_::<jsbind::Any>()
    }
}
impl CustomEvent {
    pub fn init_custom_event0(&self, type_: jsbind::DOMString) -> jsbind::Undefined {
        self.inner
            .call("initCustomEvent", &[type_.into()])
            .as_::<jsbind::Undefined>()
    }

    pub fn init_custom_event1(&self, type_: jsbind::DOMString, bubbles: bool) -> jsbind::Undefined {
        self.inner
            .call("initCustomEvent", &[type_.into(), bubbles.into()])
            .as_::<jsbind::Undefined>()
    }

    pub fn init_custom_event2(
        &self,
        type_: jsbind::DOMString,
        bubbles: bool,
        cancelable: bool,
    ) -> jsbind::Undefined {
        self.inner
            .call(
                "initCustomEvent",
                &[type_.into(), bubbles.into(), cancelable.into()],
            )
            .as_::<jsbind::Undefined>()
    }

    pub fn init_custom_event3(
        &self,
        type_: jsbind::DOMString,
        bubbles: bool,
        cancelable: bool,
        detail: jsbind::Any,
    ) -> jsbind::Undefined {
        self.inner
            .call(
                "initCustomEvent",
                &[
                    type_.into(),
                    bubbles.into(),
                    cancelable.into(),
                    detail.into(),
                ],
            )
            .as_::<jsbind::Undefined>()
    }
}
