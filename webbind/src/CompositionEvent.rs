use super::*;

#[derive(Clone, Debug)]
pub struct CompositionEvent {
    inner: UIEvent,
}
impl FromVal for CompositionEvent {
    fn from_val(v: &emlite::Val) -> Self {
        CompositionEvent {
            inner: UIEvent::from_val(v),
        }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl std::ops::Deref for CompositionEvent {
    type Target = UIEvent;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for CompositionEvent {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<CompositionEvent> for emlite::Val {
    fn from(s: CompositionEvent) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl CompositionEvent {
    pub fn new0(type_: jsbind::DOMString) -> CompositionEvent {
        Self {
            inner: emlite::Val::global("CompositionEvent")
                .new(&[type_.into()])
                .as_::<UIEvent>(),
        }
    }

    pub fn new1(type_: jsbind::DOMString, event_init_dict: jsbind::Any) -> CompositionEvent {
        Self {
            inner: emlite::Val::global("CompositionEvent")
                .new(&[type_.into(), event_init_dict.into()])
                .as_::<UIEvent>(),
        }
    }
}
impl CompositionEvent {
    pub fn data(&self) -> jsbind::USVString {
        self.inner.get("data").as_::<jsbind::USVString>()
    }
}
impl CompositionEvent {
    pub fn init_composition_event0(&self, type_arg: jsbind::DOMString) -> jsbind::Undefined {
        self.inner
            .call("initCompositionEvent", &[type_arg.into()])
            .as_::<jsbind::Undefined>()
    }

    pub fn init_composition_event1(
        &self,
        type_arg: jsbind::DOMString,
        bubbles_arg: bool,
    ) -> jsbind::Undefined {
        self.inner
            .call(
                "initCompositionEvent",
                &[type_arg.into(), bubbles_arg.into()],
            )
            .as_::<jsbind::Undefined>()
    }

    pub fn init_composition_event2(
        &self,
        type_arg: jsbind::DOMString,
        bubbles_arg: bool,
        cancelable_arg: bool,
    ) -> jsbind::Undefined {
        self.inner
            .call(
                "initCompositionEvent",
                &[type_arg.into(), bubbles_arg.into(), cancelable_arg.into()],
            )
            .as_::<jsbind::Undefined>()
    }

    pub fn init_composition_event3(
        &self,
        type_arg: jsbind::DOMString,
        bubbles_arg: bool,
        cancelable_arg: bool,
        view_arg: jsbind::Any,
    ) -> jsbind::Undefined {
        self.inner
            .call(
                "initCompositionEvent",
                &[
                    type_arg.into(),
                    bubbles_arg.into(),
                    cancelable_arg.into(),
                    view_arg.into(),
                ],
            )
            .as_::<jsbind::Undefined>()
    }

    pub fn init_composition_event4(
        &self,
        type_arg: jsbind::DOMString,
        bubbles_arg: bool,
        cancelable_arg: bool,
        view_arg: jsbind::Any,
        data_arg: jsbind::DOMString,
    ) -> jsbind::Undefined {
        self.inner
            .call(
                "initCompositionEvent",
                &[
                    type_arg.into(),
                    bubbles_arg.into(),
                    cancelable_arg.into(),
                    view_arg.into(),
                    data_arg.into(),
                ],
            )
            .as_::<jsbind::Undefined>()
    }
}
