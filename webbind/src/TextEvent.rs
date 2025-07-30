use super::*;

/// The TextEvent class.
/// [`TextEvent`](https://developer.mozilla.org/en-US/docs/Web/API/TextEvent)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct TextEvent {
    inner: UIEvent,
}
impl FromVal for TextEvent {
    fn from_val(v: &Any) -> Self {
        TextEvent {
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
impl AsRef<Any> for TextEvent {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for TextEvent {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<TextEvent> for Any {
    fn from(s: TextEvent) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&TextEvent> for Any {
    fn from(s: &TextEvent) -> Any {
        s.inner.clone().into()
    }
}
jsbind::utils::impl_dyn_cast!(TextEvent);

impl TextEvent {
    /// Getter of the `data` attribute.
    /// [`TextEvent.data`](https://developer.mozilla.org/en-US/docs/Web/API/TextEvent/data)
    pub fn data(&self) -> JsString {
        self.inner.get("data").as_::<JsString>()
    }
}
impl TextEvent {
    /// The initTextEvent method.
    /// [`TextEvent.initTextEvent`](https://developer.mozilla.org/en-US/docs/Web/API/TextEvent/initTextEvent)
    pub fn init_text_event0(&self, type_: &JsString) -> Undefined {
        self.inner
            .call("initTextEvent", &[type_.into()])
            .as_::<Undefined>()
    }
    /// The initTextEvent method.
    /// [`TextEvent.initTextEvent`](https://developer.mozilla.org/en-US/docs/Web/API/TextEvent/initTextEvent)
    pub fn init_text_event1(&self, type_: &JsString, bubbles: bool) -> Undefined {
        self.inner
            .call("initTextEvent", &[type_.into(), bubbles.into()])
            .as_::<Undefined>()
    }
    /// The initTextEvent method.
    /// [`TextEvent.initTextEvent`](https://developer.mozilla.org/en-US/docs/Web/API/TextEvent/initTextEvent)
    pub fn init_text_event2(&self, type_: &JsString, bubbles: bool, cancelable: bool) -> Undefined {
        self.inner
            .call(
                "initTextEvent",
                &[type_.into(), bubbles.into(), cancelable.into()],
            )
            .as_::<Undefined>()
    }
    /// The initTextEvent method.
    /// [`TextEvent.initTextEvent`](https://developer.mozilla.org/en-US/docs/Web/API/TextEvent/initTextEvent)
    pub fn init_text_event3(
        &self,
        type_: &JsString,
        bubbles: bool,
        cancelable: bool,
        view: &Window,
    ) -> Undefined {
        self.inner
            .call(
                "initTextEvent",
                &[type_.into(), bubbles.into(), cancelable.into(), view.into()],
            )
            .as_::<Undefined>()
    }
    /// The initTextEvent method.
    /// [`TextEvent.initTextEvent`](https://developer.mozilla.org/en-US/docs/Web/API/TextEvent/initTextEvent)
    pub fn init_text_event4(
        &self,
        type_: &JsString,
        bubbles: bool,
        cancelable: bool,
        view: &Window,
        data: &JsString,
    ) -> Undefined {
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
            .as_::<Undefined>()
    }
}
