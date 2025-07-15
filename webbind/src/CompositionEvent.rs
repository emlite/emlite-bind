use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
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
impl core::ops::Deref for CompositionEvent {
    type Target = UIEvent;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for CompositionEvent {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for CompositionEvent {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for CompositionEvent {
    fn as_mut(&mut self) -> &mut emlite::Val {
        &mut self.inner
    }
}
impl From<CompositionEvent> for emlite::Val {
    fn from(s: CompositionEvent) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}
impl From<&CompositionEvent> for emlite::Val {
    fn from(s: &CompositionEvent) -> emlite::Val {
        s.inner.clone().into()
    }
}
jsbind::utils::impl_dyn_cast!(CompositionEvent);

impl CompositionEvent {
    pub fn new0(type_: &str) -> CompositionEvent {
        Self {
            inner: emlite::Val::global("CompositionEvent")
                .new(&[type_.into()])
                .as_::<UIEvent>(),
        }
    }

    pub fn new1(type_: &str, event_init_dict: &Any) -> CompositionEvent {
        Self {
            inner: emlite::Val::global("CompositionEvent")
                .new(&[type_.into(), event_init_dict.into()])
                .as_::<UIEvent>(),
        }
    }
}
impl CompositionEvent {
    pub fn data(&self) -> String {
        self.inner.get("data").as_::<String>()
    }
}
impl CompositionEvent {
    pub fn init_composition_event0(&self, type_arg: &str) -> Undefined {
        self.inner
            .call("initCompositionEvent", &[type_arg.into()])
            .as_::<Undefined>()
    }

    pub fn init_composition_event1(&self, type_arg: &str, bubbles_arg: bool) -> Undefined {
        self.inner
            .call(
                "initCompositionEvent",
                &[type_arg.into(), bubbles_arg.into()],
            )
            .as_::<Undefined>()
    }

    pub fn init_composition_event2(
        &self,
        type_arg: &str,
        bubbles_arg: bool,
        cancelable_arg: bool,
    ) -> Undefined {
        self.inner
            .call(
                "initCompositionEvent",
                &[type_arg.into(), bubbles_arg.into(), cancelable_arg.into()],
            )
            .as_::<Undefined>()
    }

    pub fn init_composition_event3(
        &self,
        type_arg: &str,
        bubbles_arg: bool,
        cancelable_arg: bool,
        view_arg: &Any,
    ) -> Undefined {
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
            .as_::<Undefined>()
    }

    pub fn init_composition_event4(
        &self,
        type_arg: &str,
        bubbles_arg: bool,
        cancelable_arg: bool,
        view_arg: &Any,
        data_arg: &str,
    ) -> Undefined {
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
            .as_::<Undefined>()
    }
}
