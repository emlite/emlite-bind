use super::*;

/// The CompositionEvent class.
/// [`CompositionEvent`](https://developer.mozilla.org/en-US/docs/Web/API/CompositionEvent)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct CompositionEvent {
    inner: UIEvent,
}
impl FromVal for CompositionEvent {
    fn from_val(v: &Any) -> Self {
        CompositionEvent {
            inner: UIEvent::from_val(v),
        }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
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
impl AsRef<Any> for CompositionEvent {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for CompositionEvent {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<CompositionEvent> for Any {
    fn from(s: CompositionEvent) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&CompositionEvent> for Any {
    fn from(s: &CompositionEvent) -> Any {
        s.inner.clone().into()
    }
}
jsbind::utils::impl_dyn_cast!(CompositionEvent);

impl CompositionEvent {
    /// The `new CompositionEvent(..)` constructor, creating a new CompositionEvent instance
    pub fn new0(type_: &JsString) -> CompositionEvent {
        Self {
            inner: Any::global("CompositionEvent")
                .new(&[type_.into()])
                .as_::<UIEvent>(),
        }
    }

    /// The `new CompositionEvent(..)` constructor, creating a new CompositionEvent instance
    pub fn new1(type_: &JsString, event_init_dict: &CompositionEventInit) -> CompositionEvent {
        Self {
            inner: Any::global("CompositionEvent")
                .new(&[type_.into(), event_init_dict.into()])
                .as_::<UIEvent>(),
        }
    }
}
impl CompositionEvent {
    /// Getter of the `data` attribute.
    /// [`CompositionEvent.data`](https://developer.mozilla.org/en-US/docs/Web/API/CompositionEvent/data)
    pub fn data(&self) -> JsString {
        self.inner.get("data").as_::<JsString>()
    }
}
impl CompositionEvent {
    /// The initCompositionEvent method.
    /// [`CompositionEvent.initCompositionEvent`](https://developer.mozilla.org/en-US/docs/Web/API/CompositionEvent/initCompositionEvent)
    pub fn init_composition_event0(&self, type_arg: &JsString) -> Undefined {
        self.inner
            .call("initCompositionEvent", &[type_arg.into()])
            .as_::<Undefined>()
    }
    /// The initCompositionEvent method.
    /// [`CompositionEvent.initCompositionEvent`](https://developer.mozilla.org/en-US/docs/Web/API/CompositionEvent/initCompositionEvent)
    pub fn init_composition_event1(&self, type_arg: &JsString, bubbles_arg: bool) -> Undefined {
        self.inner
            .call(
                "initCompositionEvent",
                &[type_arg.into(), bubbles_arg.into()],
            )
            .as_::<Undefined>()
    }
    /// The initCompositionEvent method.
    /// [`CompositionEvent.initCompositionEvent`](https://developer.mozilla.org/en-US/docs/Web/API/CompositionEvent/initCompositionEvent)
    pub fn init_composition_event2(
        &self,
        type_arg: &JsString,
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
    /// The initCompositionEvent method.
    /// [`CompositionEvent.initCompositionEvent`](https://developer.mozilla.org/en-US/docs/Web/API/CompositionEvent/initCompositionEvent)
    pub fn init_composition_event3(
        &self,
        type_arg: &JsString,
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
    /// The initCompositionEvent method.
    /// [`CompositionEvent.initCompositionEvent`](https://developer.mozilla.org/en-US/docs/Web/API/CompositionEvent/initCompositionEvent)
    pub fn init_composition_event4(
        &self,
        type_arg: &JsString,
        bubbles_arg: bool,
        cancelable_arg: bool,
        view_arg: &Any,
        data_arg: &JsString,
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
