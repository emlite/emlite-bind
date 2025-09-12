use super::*;

/// The CustomEvent class.
/// [`CustomEvent`](https://developer.mozilla.org/en-US/docs/Web/API/CustomEvent)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct CustomEvent {
    inner: Event,
}

impl FromVal for CustomEvent {
    fn from_val(v: &Any) -> Self {
        CustomEvent {
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

impl AsRef<Any> for CustomEvent {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for CustomEvent {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}

impl From<CustomEvent> for Any {
    fn from(s: CustomEvent) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&CustomEvent> for Any {
    fn from(s: &CustomEvent) -> Any {
        s.inner.clone().into()
    }
}

jsbind::utils::impl_dyn_cast!(CustomEvent);

impl CustomEvent {
    /// Getter of the `detail` attribute.
    /// [`CustomEvent.detail`](https://developer.mozilla.org/en-US/docs/Web/API/CustomEvent/detail)
    pub fn detail(&self) -> Any {
        self.inner.get("detail").as_::<Any>()
    }
}

impl CustomEvent {
    /// The `new CustomEvent(..)` constructor, creating a new CustomEvent instance
    pub fn new(type_: &JsString) -> CustomEvent {
        Self {
            inner: Any::global("CustomEvent")
                .new(&[type_.into()])
                .as_::<Event>(),
        }
    }
}

impl CustomEvent {
    /// The `new CustomEvent(..)` constructor, creating a new CustomEvent instance
    pub fn new_with_event_init_dict(
        type_: &JsString,
        event_init_dict: &CustomEventInit,
    ) -> CustomEvent {
        Self {
            inner: Any::global("CustomEvent")
                .new(&[type_.into(), event_init_dict.into()])
                .as_::<Event>(),
        }
    }
}

impl CustomEvent {
    /// The initCustomEvent method.
    /// [`CustomEvent.initCustomEvent`](https://developer.mozilla.org/en-US/docs/Web/API/CustomEvent/initCustomEvent)
    pub fn init_custom_event(&self, type_: &JsString) -> Undefined {
        self.inner
            .call("initCustomEvent", &[type_.into()])
            .as_::<Undefined>()
    }
}
impl CustomEvent {
    /// The initCustomEvent method.
    /// [`CustomEvent.initCustomEvent`](https://developer.mozilla.org/en-US/docs/Web/API/CustomEvent/initCustomEvent)
    pub fn init_custom_event_with_bubbles(&self, type_: &JsString, bubbles: bool) -> Undefined {
        self.inner
            .call("initCustomEvent", &[type_.into(), bubbles.into()])
            .as_::<Undefined>()
    }
}
impl CustomEvent {
    /// The initCustomEvent method.
    /// [`CustomEvent.initCustomEvent`](https://developer.mozilla.org/en-US/docs/Web/API/CustomEvent/initCustomEvent)
    pub fn init_custom_event_with_bubbles_and_cancelable(
        &self,
        type_: &JsString,
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
}
impl CustomEvent {
    /// The initCustomEvent method.
    /// [`CustomEvent.initCustomEvent`](https://developer.mozilla.org/en-US/docs/Web/API/CustomEvent/initCustomEvent)
    pub fn init_custom_event_with_bubbles_and_cancelable_and_detail(
        &self,
        type_: &JsString,
        bubbles: bool,
        cancelable: bool,
        detail: &Any,
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
