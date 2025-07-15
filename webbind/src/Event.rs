use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct Event {
    inner: emlite::Val,
}
impl FromVal for Event {
    fn from_val(v: &emlite::Val) -> Self {
        Event {
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
impl core::ops::Deref for Event {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for Event {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for Event {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for Event {
    fn as_mut(&mut self) -> &mut emlite::Val {
        &mut self.inner
    }
}
impl From<Event> for emlite::Val {
    fn from(s: Event) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}
jsbind::utils::impl_dyn_cast!(Event);

impl Event {
    pub fn new0(type_: DOMString) -> Event {
        Self {
            inner: emlite::Val::global("Event")
                .new(&[type_.into()])
                .as_::<emlite::Val>(),
        }
    }

    pub fn new1(type_: DOMString, event_init_dict: Any) -> Event {
        Self {
            inner: emlite::Val::global("Event")
                .new(&[type_.into(), event_init_dict.into()])
                .as_::<emlite::Val>(),
        }
    }
}
impl Event {
    pub fn type_(&self) -> DOMString {
        self.inner.get("type").as_::<DOMString>()
    }
}
impl Event {
    pub fn target(&self) -> EventTarget {
        self.inner.get("target").as_::<EventTarget>()
    }
}
impl Event {
    pub fn src_element(&self) -> EventTarget {
        self.inner.get("srcElement").as_::<EventTarget>()
    }
}
impl Event {
    pub fn current_target(&self) -> EventTarget {
        self.inner.get("currentTarget").as_::<EventTarget>()
    }
}
impl Event {
    pub fn composed_path(&self) -> Sequence<EventTarget> {
        self.inner
            .call("composedPath", &[])
            .as_::<Sequence<EventTarget>>()
    }
}
impl Event {
    pub fn event_phase(&self) -> u16 {
        self.inner.get("eventPhase").as_::<u16>()
    }
}
impl Event {
    pub fn stop_propagation(&self) -> Undefined {
        self.inner.call("stopPropagation", &[]).as_::<Undefined>()
    }
}
impl Event {
    pub fn cancel_bubble(&self) -> bool {
        self.inner.get("cancelBubble").as_::<bool>()
    }

    pub fn set_cancel_bubble(&mut self, value: bool) {
        self.inner.set("cancelBubble", value);
    }
}
impl Event {
    pub fn stop_immediate_propagation(&self) -> Undefined {
        self.inner
            .call("stopImmediatePropagation", &[])
            .as_::<Undefined>()
    }
}
impl Event {
    pub fn bubbles(&self) -> bool {
        self.inner.get("bubbles").as_::<bool>()
    }
}
impl Event {
    pub fn cancelable(&self) -> bool {
        self.inner.get("cancelable").as_::<bool>()
    }
}
impl Event {
    pub fn return_value(&self) -> bool {
        self.inner.get("returnValue").as_::<bool>()
    }

    pub fn set_return_value(&mut self, value: bool) {
        self.inner.set("returnValue", value);
    }
}
impl Event {
    pub fn prevent_default(&self) -> Undefined {
        self.inner.call("preventDefault", &[]).as_::<Undefined>()
    }
}
impl Event {
    pub fn default_prevented(&self) -> bool {
        self.inner.get("defaultPrevented").as_::<bool>()
    }
}
impl Event {
    pub fn composed(&self) -> bool {
        self.inner.get("composed").as_::<bool>()
    }
}
impl Event {
    pub fn is_trusted(&self) -> bool {
        self.inner.get("isTrusted").as_::<bool>()
    }
}
impl Event {
    pub fn time_stamp(&self) -> Any {
        self.inner.get("timeStamp").as_::<Any>()
    }
}
impl Event {
    pub fn init_event0(&self, type_: DOMString) -> Undefined {
        self.inner
            .call("initEvent", &[type_.into()])
            .as_::<Undefined>()
    }

    pub fn init_event1(&self, type_: DOMString, bubbles: bool) -> Undefined {
        self.inner
            .call("initEvent", &[type_.into(), bubbles.into()])
            .as_::<Undefined>()
    }

    pub fn init_event2(&self, type_: DOMString, bubbles: bool, cancelable: bool) -> Undefined {
        self.inner
            .call(
                "initEvent",
                &[type_.into(), bubbles.into(), cancelable.into()],
            )
            .as_::<Undefined>()
    }
}
