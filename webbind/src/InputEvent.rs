use super::*;

/// The InputEvent class.
/// [`InputEvent`](https://developer.mozilla.org/en-US/docs/Web/API/InputEvent)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct InputEvent {
    inner: UIEvent,
}

impl FromVal for InputEvent {
    fn from_val(v: &Any) -> Self {
        InputEvent {
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

impl core::ops::Deref for InputEvent {
    type Target = UIEvent;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for InputEvent {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for InputEvent {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for InputEvent {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}

impl From<InputEvent> for Any {
    fn from(s: InputEvent) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&InputEvent> for Any {
    fn from(s: &InputEvent) -> Any {
        s.inner.clone().into()
    }
}

jsbind::utils::impl_dyn_cast!(InputEvent);

impl InputEvent {
    /// Getter of the `data` attribute.
    /// [`InputEvent.data`](https://developer.mozilla.org/en-US/docs/Web/API/InputEvent/data)
    pub fn data(&self) -> JsString {
        self.inner.get("data").as_::<JsString>()
    }
}
impl InputEvent {
    /// Getter of the `isComposing` attribute.
    /// [`InputEvent.isComposing`](https://developer.mozilla.org/en-US/docs/Web/API/InputEvent/isComposing)
    pub fn is_composing(&self) -> bool {
        self.inner.get("isComposing").as_::<bool>()
    }
}
impl InputEvent {
    /// Getter of the `inputType` attribute.
    /// [`InputEvent.inputType`](https://developer.mozilla.org/en-US/docs/Web/API/InputEvent/inputType)
    pub fn input_type(&self) -> JsString {
        self.inner.get("inputType").as_::<JsString>()
    }
}
impl InputEvent {
    /// Getter of the `dataTransfer` attribute.
    /// [`InputEvent.dataTransfer`](https://developer.mozilla.org/en-US/docs/Web/API/InputEvent/dataTransfer)
    pub fn data_transfer(&self) -> DataTransfer {
        self.inner.get("dataTransfer").as_::<DataTransfer>()
    }
}

impl InputEvent {
    /// The `new InputEvent(..)` constructor, creating a new InputEvent instance
    pub fn new0(type_: &JsString) -> InputEvent {
        Self {
            inner: Any::global("InputEvent")
                .new(&[type_.into()])
                .as_::<UIEvent>(),
        }
    }

    /// The `new InputEvent(..)` constructor, creating a new InputEvent instance
    pub fn new1(type_: &JsString, event_init_dict: &InputEventInit) -> InputEvent {
        Self {
            inner: Any::global("InputEvent")
                .new(&[type_.into(), event_init_dict.into()])
                .as_::<UIEvent>(),
        }
    }
}
impl InputEvent {
    /// The getTargetRanges method.
    /// [`InputEvent.getTargetRanges`](https://developer.mozilla.org/en-US/docs/Web/API/InputEvent/getTargetRanges)
    pub fn get_target_ranges(&self) -> TypedArray<StaticRange> {
        self.inner
            .call("getTargetRanges", &[])
            .as_::<TypedArray<StaticRange>>()
    }
}
