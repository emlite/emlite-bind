use super::*;

/// The Event class.
/// [`Event`](https://developer.mozilla.org/en-US/docs/Web/API/Event)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct Event {
    inner: Any,
}
impl FromVal for Event {
    fn from_val(v: &Any) -> Self {
        Event {
            inner: Any::from_val(v),
        }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for Event {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for Event {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for Event {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for Event {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<Event> for Any {
    fn from(s: Event) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&Event> for Any {
    fn from(s: &Event) -> Any {
        s.inner.clone().into()
    }
}
jsbind::utils::impl_dyn_cast!(Event);

impl Event {
    /// The `new Event(..)` constructor, creating a new Event instance
    pub fn new0(type_: &JsString) -> Event {
        Self {
            inner: Any::global("Event").new(&[type_.into()]).as_::<Any>(),
        }
    }

    /// The `new Event(..)` constructor, creating a new Event instance
    pub fn new1(type_: &JsString, event_init_dict: &Any) -> Event {
        Self {
            inner: Any::global("Event")
                .new(&[type_.into(), event_init_dict.into()])
                .as_::<Any>(),
        }
    }
}
impl Event {
    /// Getter of the `type` attribute.
    /// [`Event.type`](https://developer.mozilla.org/en-US/docs/Web/API/Event/type)
    pub fn type_(&self) -> JsString {
        self.inner.get("type").as_::<JsString>()
    }
}
impl Event {
    /// Getter of the `target` attribute.
    /// [`Event.target`](https://developer.mozilla.org/en-US/docs/Web/API/Event/target)
    pub fn target(&self) -> EventTarget {
        self.inner.get("target").as_::<EventTarget>()
    }
}
impl Event {
    /// Getter of the `srcElement` attribute.
    /// [`Event.srcElement`](https://developer.mozilla.org/en-US/docs/Web/API/Event/srcElement)
    pub fn src_element(&self) -> EventTarget {
        self.inner.get("srcElement").as_::<EventTarget>()
    }
}
impl Event {
    /// Getter of the `currentTarget` attribute.
    /// [`Event.currentTarget`](https://developer.mozilla.org/en-US/docs/Web/API/Event/currentTarget)
    pub fn current_target(&self) -> EventTarget {
        self.inner.get("currentTarget").as_::<EventTarget>()
    }
}
impl Event {
    /// The composedPath method.
    /// [`Event.composedPath`](https://developer.mozilla.org/en-US/docs/Web/API/Event/composedPath)
    pub fn composed_path(&self) -> TypedArray<EventTarget> {
        self.inner
            .call("composedPath", &[])
            .as_::<TypedArray<EventTarget>>()
    }
}
impl Event {
    /// Getter of the `eventPhase` attribute.
    /// [`Event.eventPhase`](https://developer.mozilla.org/en-US/docs/Web/API/Event/eventPhase)
    pub fn event_phase(&self) -> u16 {
        self.inner.get("eventPhase").as_::<u16>()
    }
}
impl Event {
    /// The stopPropagation method.
    /// [`Event.stopPropagation`](https://developer.mozilla.org/en-US/docs/Web/API/Event/stopPropagation)
    pub fn stop_propagation(&self) -> Undefined {
        self.inner.call("stopPropagation", &[]).as_::<Undefined>()
    }
}
impl Event {
    /// Getter of the `cancelBubble` attribute.
    /// [`Event.cancelBubble`](https://developer.mozilla.org/en-US/docs/Web/API/Event/cancelBubble)
    pub fn cancel_bubble(&self) -> bool {
        self.inner.get("cancelBubble").as_::<bool>()
    }

    /// Setter of the `cancelBubble` attribute.
    /// [`Event.cancelBubble`](https://developer.mozilla.org/en-US/docs/Web/API/Event/cancelBubble)
    pub fn set_cancel_bubble(&mut self, value: bool) {
        self.inner.set("cancelBubble", value);
    }
}
impl Event {
    /// The stopImmediatePropagation method.
    /// [`Event.stopImmediatePropagation`](https://developer.mozilla.org/en-US/docs/Web/API/Event/stopImmediatePropagation)
    pub fn stop_immediate_propagation(&self) -> Undefined {
        self.inner
            .call("stopImmediatePropagation", &[])
            .as_::<Undefined>()
    }
}
impl Event {
    /// Getter of the `bubbles` attribute.
    /// [`Event.bubbles`](https://developer.mozilla.org/en-US/docs/Web/API/Event/bubbles)
    pub fn bubbles(&self) -> bool {
        self.inner.get("bubbles").as_::<bool>()
    }
}
impl Event {
    /// Getter of the `cancelable` attribute.
    /// [`Event.cancelable`](https://developer.mozilla.org/en-US/docs/Web/API/Event/cancelable)
    pub fn cancelable(&self) -> bool {
        self.inner.get("cancelable").as_::<bool>()
    }
}
impl Event {
    /// Getter of the `returnValue` attribute.
    /// [`Event.returnValue`](https://developer.mozilla.org/en-US/docs/Web/API/Event/returnValue)
    pub fn return_value(&self) -> bool {
        self.inner.get("returnValue").as_::<bool>()
    }

    /// Setter of the `returnValue` attribute.
    /// [`Event.returnValue`](https://developer.mozilla.org/en-US/docs/Web/API/Event/returnValue)
    pub fn set_return_value(&mut self, value: bool) {
        self.inner.set("returnValue", value);
    }
}
impl Event {
    /// The preventDefault method.
    /// [`Event.preventDefault`](https://developer.mozilla.org/en-US/docs/Web/API/Event/preventDefault)
    pub fn prevent_default(&self) -> Undefined {
        self.inner.call("preventDefault", &[]).as_::<Undefined>()
    }
}
impl Event {
    /// Getter of the `defaultPrevented` attribute.
    /// [`Event.defaultPrevented`](https://developer.mozilla.org/en-US/docs/Web/API/Event/defaultPrevented)
    pub fn default_prevented(&self) -> bool {
        self.inner.get("defaultPrevented").as_::<bool>()
    }
}
impl Event {
    /// Getter of the `composed` attribute.
    /// [`Event.composed`](https://developer.mozilla.org/en-US/docs/Web/API/Event/composed)
    pub fn composed(&self) -> bool {
        self.inner.get("composed").as_::<bool>()
    }
}
impl Event {
    /// Getter of the `isTrusted` attribute.
    /// [`Event.isTrusted`](https://developer.mozilla.org/en-US/docs/Web/API/Event/isTrusted)
    pub fn is_trusted(&self) -> bool {
        self.inner.get("isTrusted").as_::<bool>()
    }
}
impl Event {
    /// Getter of the `timeStamp` attribute.
    /// [`Event.timeStamp`](https://developer.mozilla.org/en-US/docs/Web/API/Event/timeStamp)
    pub fn time_stamp(&self) -> Any {
        self.inner.get("timeStamp").as_::<Any>()
    }
}
impl Event {
    /// The initEvent method.
    /// [`Event.initEvent`](https://developer.mozilla.org/en-US/docs/Web/API/Event/initEvent)
    pub fn init_event0(&self, type_: &JsString) -> Undefined {
        self.inner
            .call("initEvent", &[type_.into()])
            .as_::<Undefined>()
    }
    /// The initEvent method.
    /// [`Event.initEvent`](https://developer.mozilla.org/en-US/docs/Web/API/Event/initEvent)
    pub fn init_event1(&self, type_: &JsString, bubbles: bool) -> Undefined {
        self.inner
            .call("initEvent", &[type_.into(), bubbles.into()])
            .as_::<Undefined>()
    }
    /// The initEvent method.
    /// [`Event.initEvent`](https://developer.mozilla.org/en-US/docs/Web/API/Event/initEvent)
    pub fn init_event2(&self, type_: &JsString, bubbles: bool, cancelable: bool) -> Undefined {
        self.inner
            .call(
                "initEvent",
                &[type_.into(), bubbles.into(), cancelable.into()],
            )
            .as_::<Undefined>()
    }
}
