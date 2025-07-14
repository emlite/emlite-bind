use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct TextEvent {
    inner: UIEvent,
}
impl FromVal for TextEvent {
    fn from_val(v: &emlite::Val) -> Self {
        TextEvent {
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
impl core::ops::Deref for TextEvent {
    type Target = UIEvent;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for TextEvent {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<TextEvent> for emlite::Val {
    fn from(s: TextEvent) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl TextEvent {
    pub fn data(&self) -> jsbind::DOMString {
        self.inner.get("data").as_::<jsbind::DOMString>()
    }
}
impl TextEvent {
    pub fn init_text_event0(&self, type_: jsbind::DOMString) -> jsbind::Undefined {
        self.inner
            .call("initTextEvent", &[type_.into()])
            .as_::<jsbind::Undefined>()
    }

    pub fn init_text_event1(&self, type_: jsbind::DOMString, bubbles: bool) -> jsbind::Undefined {
        self.inner
            .call("initTextEvent", &[type_.into(), bubbles.into()])
            .as_::<jsbind::Undefined>()
    }

    pub fn init_text_event2(
        &self,
        type_: jsbind::DOMString,
        bubbles: bool,
        cancelable: bool,
    ) -> jsbind::Undefined {
        self.inner
            .call(
                "initTextEvent",
                &[type_.into(), bubbles.into(), cancelable.into()],
            )
            .as_::<jsbind::Undefined>()
    }

    pub fn init_text_event3(
        &self,
        type_: jsbind::DOMString,
        bubbles: bool,
        cancelable: bool,
        view: Window,
    ) -> jsbind::Undefined {
        self.inner
            .call(
                "initTextEvent",
                &[type_.into(), bubbles.into(), cancelable.into(), view.into()],
            )
            .as_::<jsbind::Undefined>()
    }

    pub fn init_text_event4(
        &self,
        type_: jsbind::DOMString,
        bubbles: bool,
        cancelable: bool,
        view: Window,
        data: jsbind::DOMString,
    ) -> jsbind::Undefined {
        self.inner
            .call(
                "initTextEvent",
                &[
                    type_.into(),
                    bubbles.into(),
                    cancelable.into(),
                    view.into(),
                    data.into(),
                ],
            )
            .as_::<jsbind::Undefined>()
    }
}
