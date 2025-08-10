use super::*;

/// The UIEvent class.
/// [`UIEvent`](https://developer.mozilla.org/en-US/docs/Web/API/UIEvent)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct UIEvent {
    inner: Event,
}
impl FromVal for UIEvent {
    fn from_val(v: &Any) -> Self {
        UIEvent {
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
impl core::ops::Deref for UIEvent {
    type Target = Event;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for UIEvent {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for UIEvent {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for UIEvent {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<UIEvent> for Any {
    fn from(s: UIEvent) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&UIEvent> for Any {
    fn from(s: &UIEvent) -> Any {
        s.inner.clone().into()
    }
}
jsbind::utils::impl_dyn_cast!(UIEvent);

impl UIEvent {
    /// The `new UIEvent(..)` constructor, creating a new UIEvent instance
    pub fn new0(type_: &JsString) -> UIEvent {
        Self {
            inner: Any::global("UIEvent").new(&[type_.into()]).as_::<Event>(),
        }
    }

    /// The `new UIEvent(..)` constructor, creating a new UIEvent instance
    pub fn new1(type_: &JsString, event_init_dict: &UIEventInit) -> UIEvent {
        Self {
            inner: Any::global("UIEvent")
                .new(&[type_.into(), event_init_dict.into()])
                .as_::<Event>(),
        }
    }
}
impl UIEvent {
    /// Getter of the `view` attribute.
    /// [`UIEvent.view`](https://developer.mozilla.org/en-US/docs/Web/API/UIEvent/view)
    pub fn view(&self) -> Window {
        self.inner.get("view").as_::<Window>()
    }
}
impl UIEvent {
    /// Getter of the `detail` attribute.
    /// [`UIEvent.detail`](https://developer.mozilla.org/en-US/docs/Web/API/UIEvent/detail)
    pub fn detail(&self) -> i32 {
        self.inner.get("detail").as_::<i32>()
    }
}
impl UIEvent {
    /// Getter of the `sourceCapabilities` attribute.
    /// [`UIEvent.sourceCapabilities`](https://developer.mozilla.org/en-US/docs/Web/API/UIEvent/sourceCapabilities)
    pub fn source_capabilities(&self) -> InputDeviceCapabilities {
        self.inner
            .get("sourceCapabilities")
            .as_::<InputDeviceCapabilities>()
    }
}
impl UIEvent {
    /// The initUIEvent method.
    /// [`UIEvent.initUIEvent`](https://developer.mozilla.org/en-US/docs/Web/API/UIEvent/initUIEvent)
    pub fn init_ui_event0(&self, type_arg: &JsString) -> Undefined {
        self.inner
            .call("initUIEvent", &[type_arg.into()])
            .as_::<Undefined>()
    }
    /// The initUIEvent method.
    /// [`UIEvent.initUIEvent`](https://developer.mozilla.org/en-US/docs/Web/API/UIEvent/initUIEvent)
    pub fn init_ui_event1(&self, type_arg: &JsString, bubbles_arg: bool) -> Undefined {
        self.inner
            .call("initUIEvent", &[type_arg.into(), bubbles_arg.into()])
            .as_::<Undefined>()
    }
    /// The initUIEvent method.
    /// [`UIEvent.initUIEvent`](https://developer.mozilla.org/en-US/docs/Web/API/UIEvent/initUIEvent)
    pub fn init_ui_event2(
        &self,
        type_arg: &JsString,
        bubbles_arg: bool,
        cancelable_arg: bool,
    ) -> Undefined {
        self.inner
            .call(
                "initUIEvent",
                &[type_arg.into(), bubbles_arg.into(), cancelable_arg.into()],
            )
            .as_::<Undefined>()
    }
    /// The initUIEvent method.
    /// [`UIEvent.initUIEvent`](https://developer.mozilla.org/en-US/docs/Web/API/UIEvent/initUIEvent)
    pub fn init_ui_event3(
        &self,
        type_arg: &JsString,
        bubbles_arg: bool,
        cancelable_arg: bool,
        view_arg: &Window,
    ) -> Undefined {
        self.inner
            .call(
                "initUIEvent",
                &[
                    type_arg.into(),
                    bubbles_arg.into(),
                    cancelable_arg.into(),
                    view_arg.into(),
                ],
            )
            .as_::<Undefined>()
    }
    /// The initUIEvent method.
    /// [`UIEvent.initUIEvent`](https://developer.mozilla.org/en-US/docs/Web/API/UIEvent/initUIEvent)
    pub fn init_ui_event4(
        &self,
        type_arg: &JsString,
        bubbles_arg: bool,
        cancelable_arg: bool,
        view_arg: &Window,
        detail_arg: i32,
    ) -> Undefined {
        self.inner
            .call(
                "initUIEvent",
                &[
                    type_arg.into(),
                    bubbles_arg.into(),
                    cancelable_arg.into(),
                    view_arg.into(),
                    detail_arg.into(),
                ],
            )
            .as_::<Undefined>()
    }
}
impl UIEvent {
    /// Getter of the `which` attribute.
    /// [`UIEvent.which`](https://developer.mozilla.org/en-US/docs/Web/API/UIEvent/which)
    pub fn which(&self) -> u32 {
        self.inner.get("which").as_::<u32>()
    }
}
