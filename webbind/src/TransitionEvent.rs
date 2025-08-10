use super::*;

/// The TransitionEvent class.
/// [`TransitionEvent`](https://developer.mozilla.org/en-US/docs/Web/API/TransitionEvent)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct TransitionEvent {
    inner: Event,
}

impl FromVal for TransitionEvent {
    fn from_val(v: &Any) -> Self {
        TransitionEvent {
            inner: Event::from_val(v),
        }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for TransitionEvent {
    type Target = Event;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for TransitionEvent {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for TransitionEvent {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for TransitionEvent {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}

impl From<TransitionEvent> for Any {
    fn from(s: TransitionEvent) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&TransitionEvent> for Any {
    fn from(s: &TransitionEvent) -> Any {
        s.inner.clone().into()
    }
}

jsbind::utils::impl_dyn_cast!(TransitionEvent);

impl TransitionEvent {
    /// The `new TransitionEvent(..)` constructor, creating a new TransitionEvent instance
    pub fn new0(type_: &JsString) -> TransitionEvent {
        Self {
            inner: Any::global("TransitionEvent")
                .new(&[type_.into()])
                .as_::<Event>(),
        }
    }

    /// The `new TransitionEvent(..)` constructor, creating a new TransitionEvent instance
    pub fn new1(
        type_: &JsString,
        transition_event_init_dict: &TransitionEventInit,
    ) -> TransitionEvent {
        Self {
            inner: Any::global("TransitionEvent")
                .new(&[type_.into(), transition_event_init_dict.into()])
                .as_::<Event>(),
        }
    }
}
impl TransitionEvent {
    /// Getter of the `propertyName` attribute.
    /// [`TransitionEvent.propertyName`](https://developer.mozilla.org/en-US/docs/Web/API/TransitionEvent/propertyName)
    pub fn property_name(&self) -> JsString {
        self.inner.get("propertyName").as_::<JsString>()
    }
}
impl TransitionEvent {
    /// Getter of the `elapsedTime` attribute.
    /// [`TransitionEvent.elapsedTime`](https://developer.mozilla.org/en-US/docs/Web/API/TransitionEvent/elapsedTime)
    pub fn elapsed_time(&self) -> f64 {
        self.inner.get("elapsedTime").as_::<f64>()
    }
}
impl TransitionEvent {
    /// Getter of the `pseudoElement` attribute.
    /// [`TransitionEvent.pseudoElement`](https://developer.mozilla.org/en-US/docs/Web/API/TransitionEvent/pseudoElement)
    pub fn pseudo_element(&self) -> JsString {
        self.inner.get("pseudoElement").as_::<JsString>()
    }
}
