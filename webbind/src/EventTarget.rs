use super::*;

#[derive(Clone, Debug)]
pub struct ObservableEventListenerOptions {
    inner: emlite::Val,
}
impl FromVal for ObservableEventListenerOptions {
    fn from_val(v: &emlite::Val) -> Self {
        ObservableEventListenerOptions { inner: v.clone() }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl std::ops::Deref for ObservableEventListenerOptions {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for ObservableEventListenerOptions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<ObservableEventListenerOptions> for emlite::Val {
    fn from(s: ObservableEventListenerOptions) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl ObservableEventListenerOptions {
    pub fn capture(&self) -> bool {
        self.inner.get("capture").as_::<bool>()
    }

    pub fn set_capture(&mut self, value: bool) {
        self.inner.set("capture", value);
    }
}
impl ObservableEventListenerOptions {
    pub fn passive(&self) -> bool {
        self.inner.get("passive").as_::<bool>()
    }

    pub fn set_passive(&mut self, value: bool) {
        self.inner.set("passive", value);
    }
}
#[derive(Clone, Debug)]
pub struct EventTarget {
    inner: emlite::Val,
}
impl FromVal for EventTarget {
    fn from_val(v: &emlite::Val) -> Self {
        EventTarget {
            inner: emlite::Val::from_val(v),
        }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl std::ops::Deref for EventTarget {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for EventTarget {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<EventTarget> for emlite::Val {
    fn from(s: EventTarget) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl EventTarget {
    pub fn new() -> EventTarget {
        Self {
            inner: emlite::Val::global("EventTarget")
                .new(&[])
                .as_::<emlite::Val>(),
        }
    }
}
impl EventTarget {
    pub fn add_event_listener0(
        &self,
        type_: jsbind::DOMString,
        callback: jsbind::Function,
    ) -> jsbind::Undefined {
        self.inner
            .call("addEventListener", &[type_.into(), callback.into()])
            .as_::<jsbind::Undefined>()
    }

    pub fn add_event_listener1(
        &self,
        type_: jsbind::DOMString,
        callback: jsbind::Function,
        options: jsbind::Any,
    ) -> jsbind::Undefined {
        self.inner
            .call(
                "addEventListener",
                &[type_.into(), callback.into(), options.into()],
            )
            .as_::<jsbind::Undefined>()
    }
}
impl EventTarget {
    pub fn remove_event_listener0(
        &self,
        type_: jsbind::DOMString,
        callback: jsbind::Function,
    ) -> jsbind::Undefined {
        self.inner
            .call("removeEventListener", &[type_.into(), callback.into()])
            .as_::<jsbind::Undefined>()
    }

    pub fn remove_event_listener1(
        &self,
        type_: jsbind::DOMString,
        callback: jsbind::Function,
        options: jsbind::Any,
    ) -> jsbind::Undefined {
        self.inner
            .call(
                "removeEventListener",
                &[type_.into(), callback.into(), options.into()],
            )
            .as_::<jsbind::Undefined>()
    }
}
impl EventTarget {
    pub fn dispatch_event(&self, event: Event) -> bool {
        self.inner
            .call("dispatchEvent", &[event.into()])
            .as_::<bool>()
    }
}
impl EventTarget {
    pub fn when0(&self, type_: jsbind::DOMString) -> Observable {
        self.inner.call("when", &[type_.into()]).as_::<Observable>()
    }

    pub fn when1(
        &self,
        type_: jsbind::DOMString,
        options: ObservableEventListenerOptions,
    ) -> Observable {
        self.inner
            .call("when", &[type_.into(), options.into()])
            .as_::<Observable>()
    }
}
