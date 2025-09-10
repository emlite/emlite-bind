use super::*;

/// The EventTarget class.
/// [`EventTarget`](https://developer.mozilla.org/en-US/docs/Web/API/EventTarget)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct EventTarget {
    inner: Any,
}

impl FromVal for EventTarget {
    fn from_val(v: &Any) -> Self {
        EventTarget {
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

impl core::ops::Deref for EventTarget {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for EventTarget {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for EventTarget {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for EventTarget {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}

impl From<EventTarget> for Any {
    fn from(s: EventTarget) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&EventTarget> for Any {
    fn from(s: &EventTarget) -> Any {
        s.inner.clone().into()
    }
}

jsbind::utils::impl_dyn_cast!(EventTarget);

impl EventTarget {
    /// The `new EventTarget(..)` constructor, creating a new EventTarget instance
    pub fn new() -> EventTarget {
        Self {
            inner: Any::global("EventTarget").new(&[]).as_::<Any>(),
        }
    }
}
impl EventTarget {
    /// The addEventListener method.
    /// [`EventTarget.addEventListener`](https://developer.mozilla.org/en-US/docs/Web/API/EventTarget/addEventListener)
    pub fn add_event_listener0(&self, type_: &JsString, callback: &Function) -> Undefined {
        self.inner
            .call("addEventListener", &[type_.into(), callback.into()])
            .as_::<Undefined>()
    }
    /// The addEventListener method.
    /// [`EventTarget.addEventListener`](https://developer.mozilla.org/en-US/docs/Web/API/EventTarget/addEventListener)
    pub fn add_event_listener1(
        &self,
        type_: &JsString,
        callback: &Function,
        options: &Any,
    ) -> Undefined {
        self.inner
            .call(
                "addEventListener",
                &[type_.into(), callback.into(), options.into()],
            )
            .as_::<Undefined>()
    }
}
impl EventTarget {
    /// The removeEventListener method.
    /// [`EventTarget.removeEventListener`](https://developer.mozilla.org/en-US/docs/Web/API/EventTarget/removeEventListener)
    pub fn remove_event_listener0(&self, type_: &JsString, callback: &Function) -> Undefined {
        self.inner
            .call("removeEventListener", &[type_.into(), callback.into()])
            .as_::<Undefined>()
    }
    /// The removeEventListener method.
    /// [`EventTarget.removeEventListener`](https://developer.mozilla.org/en-US/docs/Web/API/EventTarget/removeEventListener)
    pub fn remove_event_listener1(
        &self,
        type_: &JsString,
        callback: &Function,
        options: &Any,
    ) -> Undefined {
        self.inner
            .call(
                "removeEventListener",
                &[type_.into(), callback.into(), options.into()],
            )
            .as_::<Undefined>()
    }
}
impl EventTarget {
    /// The dispatchEvent method.
    /// [`EventTarget.dispatchEvent`](https://developer.mozilla.org/en-US/docs/Web/API/EventTarget/dispatchEvent)
    pub fn dispatch_event(&self, event: &Event) -> bool {
        self.inner
            .call("dispatchEvent", &[event.into()])
            .as_::<bool>()
    }
}
impl EventTarget {
    /// The when method.
    /// [`EventTarget.when`](https://developer.mozilla.org/en-US/docs/Web/API/EventTarget/when)
    pub fn when0(&self, type_: &JsString) -> Observable {
        self.inner.call("when", &[type_.into()]).as_::<Observable>()
    }
    /// The when method.
    /// [`EventTarget.when`](https://developer.mozilla.org/en-US/docs/Web/API/EventTarget/when)
    pub fn when1(&self, type_: &JsString, options: &ObservableEventListenerOptions) -> Observable {
        self.inner
            .call("when", &[type_.into(), options.into()])
            .as_::<Observable>()
    }
}
