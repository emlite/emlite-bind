use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct UIEvent {
    inner: Event,
}
impl FromVal for UIEvent {
    fn from_val(v: &emlite::Val) -> Self {
        UIEvent {
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
impl core::ops::Deref for UIEvent {
    type Target = Event;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for UIEvent {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for UIEvent {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for UIEvent {
    fn as_mut(&mut self) -> &mut emlite::Val {
        &mut self.inner
    }
}
impl From<UIEvent> for emlite::Val {
    fn from(s: UIEvent) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}
impl From<&UIEvent> for emlite::Val {
    fn from(s: &UIEvent) -> emlite::Val {
        s.inner.clone().into()
    }
}
jsbind::utils::impl_dyn_cast!(UIEvent);

impl UIEvent {
    pub fn new0(type_: &str) -> UIEvent {
        Self {
            inner: emlite::Val::global("UIEvent")
                .new(&[type_.into()])
                .as_::<Event>(),
        }
    }

    pub fn new1(type_: &str, event_init_dict: &Any) -> UIEvent {
        Self {
            inner: emlite::Val::global("UIEvent")
                .new(&[type_.into(), event_init_dict.into()])
                .as_::<Event>(),
        }
    }
}
impl UIEvent {
    pub fn view(&self) -> Window {
        self.inner.get("view").as_::<Window>()
    }
}
impl UIEvent {
    pub fn detail(&self) -> i32 {
        self.inner.get("detail").as_::<i32>()
    }
}
impl UIEvent {
    pub fn source_capabilities(&self) -> InputDeviceCapabilities {
        self.inner
            .get("sourceCapabilities")
            .as_::<InputDeviceCapabilities>()
    }
}
impl UIEvent {
    pub fn init_ui_event0(&self, type_arg: &str) -> Undefined {
        self.inner
            .call("initUIEvent", &[type_arg.into()])
            .as_::<Undefined>()
    }

    pub fn init_ui_event1(&self, type_arg: &str, bubbles_arg: bool) -> Undefined {
        self.inner
            .call("initUIEvent", &[type_arg.into(), bubbles_arg.into()])
            .as_::<Undefined>()
    }

    pub fn init_ui_event2(
        &self,
        type_arg: &str,
        bubbles_arg: bool,
        cancelable_arg: bool,
    ) -> Undefined {
        self.inner
            .call(
                "initUIEvent",
                &[type_arg.into(), bubbles_arg.into(), cancelable_arg.into()],
            )
            .as_::<Undefined>()
    }

    pub fn init_ui_event3(
        &self,
        type_arg: &str,
        bubbles_arg: bool,
        cancelable_arg: bool,
        view_arg: &Window,
    ) -> Undefined {
        self.inner
            .call(
                "initUIEvent",
                &[
                    type_arg.into(),
                    bubbles_arg.into(),
                    cancelable_arg.into(),
                    view_arg.into(),
                ],
            )
            .as_::<Undefined>()
    }

    pub fn init_ui_event4(
        &self,
        type_arg: &str,
        bubbles_arg: bool,
        cancelable_arg: bool,
        view_arg: &Window,
        detail_arg: i32,
    ) -> Undefined {
        self.inner
            .call(
                "initUIEvent",
                &[
                    type_arg.into(),
                    bubbles_arg.into(),
                    cancelable_arg.into(),
                    view_arg.into(),
                    detail_arg.into(),
                ],
            )
            .as_::<Undefined>()
    }
}
impl UIEvent {
    pub fn which(&self) -> u32 {
        self.inner.get("which").as_::<u32>()
    }
}
