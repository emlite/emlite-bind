use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
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
impl core::ops::Deref for CustomEvent {
    type Target = Event;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for CustomEvent {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for CustomEvent {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for CustomEvent {
    fn as_mut(&mut self) -> &mut emlite::Val {
        &mut self.inner
    }
}
impl From<CustomEvent> for emlite::Val {
    fn from(s: CustomEvent) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}
jsbind::utils::impl_dyn_cast!(CustomEvent);

impl CustomEvent {
    pub fn new0(type_: DOMString) -> CustomEvent {
        Self {
            inner: emlite::Val::global("CustomEvent")
                .new(&[type_.into()])
                .as_::<Event>(),
        }
    }

    pub fn new1(type_: DOMString, event_init_dict: Any) -> CustomEvent {
        Self {
            inner: emlite::Val::global("CustomEvent")
                .new(&[type_.into(), event_init_dict.into()])
                .as_::<Event>(),
        }
    }
}
impl CustomEvent {
    pub fn detail(&self) -> Any {
        self.inner.get("detail").as_::<Any>()
    }
}
impl CustomEvent {
    pub fn init_custom_event0(&self, type_: DOMString) -> Undefined {
        self.inner
            .call("initCustomEvent", &[type_.into()])
            .as_::<Undefined>()
    }

    pub fn init_custom_event1(&self, type_: DOMString, bubbles: bool) -> Undefined {
        self.inner
            .call("initCustomEvent", &[type_.into(), bubbles.into()])
            .as_::<Undefined>()
    }

    pub fn init_custom_event2(
        &self,
        type_: DOMString,
        bubbles: bool,
        cancelable: bool,
    ) -> Undefined {
        self.inner
            .call(
                "initCustomEvent",
                &[type_.into(), bubbles.into(), cancelable.into()],
            )
            .as_::<Undefined>()
    }

    pub fn init_custom_event3(
        &self,
        type_: DOMString,
        bubbles: bool,
        cancelable: bool,
        detail: Any,
    ) -> Undefined {
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
            .as_::<Undefined>()
    }
}
