use super::*;

/// The MouseEvent class.
/// [`MouseEvent`](https://developer.mozilla.org/en-US/docs/Web/API/MouseEvent)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct MouseEvent {
    inner: UIEvent,
}
impl FromVal for MouseEvent {
    fn from_val(v: &Any) -> Self {
        MouseEvent {
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
impl core::ops::Deref for MouseEvent {
    type Target = UIEvent;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for MouseEvent {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for MouseEvent {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for MouseEvent {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<MouseEvent> for Any {
    fn from(s: MouseEvent) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&MouseEvent> for Any {
    fn from(s: &MouseEvent) -> Any {
        s.inner.clone().into()
    }
}
jsbind::utils::impl_dyn_cast!(MouseEvent);

impl MouseEvent {
    /// The `new MouseEvent(..)` constructor, creating a new MouseEvent instance
    pub fn new0(type_: &JsString) -> MouseEvent {
        Self {
            inner: Any::global("MouseEvent")
                .new(&[type_.into()])
                .as_::<UIEvent>(),
        }
    }

    /// The `new MouseEvent(..)` constructor, creating a new MouseEvent instance
    pub fn new1(type_: &JsString, event_init_dict: &MouseEventInit) -> MouseEvent {
        Self {
            inner: Any::global("MouseEvent")
                .new(&[type_.into(), event_init_dict.into()])
                .as_::<UIEvent>(),
        }
    }
}
impl MouseEvent {
    /// Getter of the `screenX` attribute.
    /// [`MouseEvent.screenX`](https://developer.mozilla.org/en-US/docs/Web/API/MouseEvent/screenX)
    pub fn screen_x(&self) -> i32 {
        self.inner.get("screenX").as_::<i32>()
    }
}
impl MouseEvent {
    /// Getter of the `screenY` attribute.
    /// [`MouseEvent.screenY`](https://developer.mozilla.org/en-US/docs/Web/API/MouseEvent/screenY)
    pub fn screen_y(&self) -> i32 {
        self.inner.get("screenY").as_::<i32>()
    }
}
impl MouseEvent {
    /// Getter of the `clientX` attribute.
    /// [`MouseEvent.clientX`](https://developer.mozilla.org/en-US/docs/Web/API/MouseEvent/clientX)
    pub fn client_x(&self) -> i32 {
        self.inner.get("clientX").as_::<i32>()
    }
}
impl MouseEvent {
    /// Getter of the `clientY` attribute.
    /// [`MouseEvent.clientY`](https://developer.mozilla.org/en-US/docs/Web/API/MouseEvent/clientY)
    pub fn client_y(&self) -> i32 {
        self.inner.get("clientY").as_::<i32>()
    }
}
impl MouseEvent {
    /// Getter of the `layerX` attribute.
    /// [`MouseEvent.layerX`](https://developer.mozilla.org/en-US/docs/Web/API/MouseEvent/layerX)
    pub fn layer_x(&self) -> i32 {
        self.inner.get("layerX").as_::<i32>()
    }
}
impl MouseEvent {
    /// Getter of the `layerY` attribute.
    /// [`MouseEvent.layerY`](https://developer.mozilla.org/en-US/docs/Web/API/MouseEvent/layerY)
    pub fn layer_y(&self) -> i32 {
        self.inner.get("layerY").as_::<i32>()
    }
}
impl MouseEvent {
    /// Getter of the `ctrlKey` attribute.
    /// [`MouseEvent.ctrlKey`](https://developer.mozilla.org/en-US/docs/Web/API/MouseEvent/ctrlKey)
    pub fn ctrl_key(&self) -> bool {
        self.inner.get("ctrlKey").as_::<bool>()
    }
}
impl MouseEvent {
    /// Getter of the `shiftKey` attribute.
    /// [`MouseEvent.shiftKey`](https://developer.mozilla.org/en-US/docs/Web/API/MouseEvent/shiftKey)
    pub fn shift_key(&self) -> bool {
        self.inner.get("shiftKey").as_::<bool>()
    }
}
impl MouseEvent {
    /// Getter of the `altKey` attribute.
    /// [`MouseEvent.altKey`](https://developer.mozilla.org/en-US/docs/Web/API/MouseEvent/altKey)
    pub fn alt_key(&self) -> bool {
        self.inner.get("altKey").as_::<bool>()
    }
}
impl MouseEvent {
    /// Getter of the `metaKey` attribute.
    /// [`MouseEvent.metaKey`](https://developer.mozilla.org/en-US/docs/Web/API/MouseEvent/metaKey)
    pub fn meta_key(&self) -> bool {
        self.inner.get("metaKey").as_::<bool>()
    }
}
impl MouseEvent {
    /// Getter of the `button` attribute.
    /// [`MouseEvent.button`](https://developer.mozilla.org/en-US/docs/Web/API/MouseEvent/button)
    pub fn button(&self) -> i16 {
        self.inner.get("button").as_::<i16>()
    }
}
impl MouseEvent {
    /// Getter of the `buttons` attribute.
    /// [`MouseEvent.buttons`](https://developer.mozilla.org/en-US/docs/Web/API/MouseEvent/buttons)
    pub fn buttons(&self) -> u16 {
        self.inner.get("buttons").as_::<u16>()
    }
}
impl MouseEvent {
    /// Getter of the `relatedTarget` attribute.
    /// [`MouseEvent.relatedTarget`](https://developer.mozilla.org/en-US/docs/Web/API/MouseEvent/relatedTarget)
    pub fn related_target(&self) -> EventTarget {
        self.inner.get("relatedTarget").as_::<EventTarget>()
    }
}
impl MouseEvent {
    /// The getModifierState method.
    /// [`MouseEvent.getModifierState`](https://developer.mozilla.org/en-US/docs/Web/API/MouseEvent/getModifierState)
    pub fn get_modifier_state(&self, key_arg: &JsString) -> bool {
        self.inner
            .call("getModifierState", &[key_arg.into()])
            .as_::<bool>()
    }
}
impl MouseEvent {
    /// Getter of the `pageX` attribute.
    /// [`MouseEvent.pageX`](https://developer.mozilla.org/en-US/docs/Web/API/MouseEvent/pageX)
    pub fn page_x(&self) -> f64 {
        self.inner.get("pageX").as_::<f64>()
    }
}
impl MouseEvent {
    /// Getter of the `pageY` attribute.
    /// [`MouseEvent.pageY`](https://developer.mozilla.org/en-US/docs/Web/API/MouseEvent/pageY)
    pub fn page_y(&self) -> f64 {
        self.inner.get("pageY").as_::<f64>()
    }
}
impl MouseEvent {
    /// Getter of the `x` attribute.
    /// [`MouseEvent.x`](https://developer.mozilla.org/en-US/docs/Web/API/MouseEvent/x)
    pub fn x(&self) -> f64 {
        self.inner.get("x").as_::<f64>()
    }
}
impl MouseEvent {
    /// Getter of the `y` attribute.
    /// [`MouseEvent.y`](https://developer.mozilla.org/en-US/docs/Web/API/MouseEvent/y)
    pub fn y(&self) -> f64 {
        self.inner.get("y").as_::<f64>()
    }
}
impl MouseEvent {
    /// Getter of the `offsetX` attribute.
    /// [`MouseEvent.offsetX`](https://developer.mozilla.org/en-US/docs/Web/API/MouseEvent/offsetX)
    pub fn offset_x(&self) -> f64 {
        self.inner.get("offsetX").as_::<f64>()
    }
}
impl MouseEvent {
    /// Getter of the `offsetY` attribute.
    /// [`MouseEvent.offsetY`](https://developer.mozilla.org/en-US/docs/Web/API/MouseEvent/offsetY)
    pub fn offset_y(&self) -> f64 {
        self.inner.get("offsetY").as_::<f64>()
    }
}
impl MouseEvent {
    /// Getter of the `movementX` attribute.
    /// [`MouseEvent.movementX`](https://developer.mozilla.org/en-US/docs/Web/API/MouseEvent/movementX)
    pub fn movement_x(&self) -> f64 {
        self.inner.get("movementX").as_::<f64>()
    }
}
impl MouseEvent {
    /// Getter of the `movementY` attribute.
    /// [`MouseEvent.movementY`](https://developer.mozilla.org/en-US/docs/Web/API/MouseEvent/movementY)
    pub fn movement_y(&self) -> f64 {
        self.inner.get("movementY").as_::<f64>()
    }
}
impl MouseEvent {
    /// The initMouseEvent method.
    /// [`MouseEvent.initMouseEvent`](https://developer.mozilla.org/en-US/docs/Web/API/MouseEvent/initMouseEvent)
    pub fn init_mouse_event0(&self, type_arg: &JsString) -> Undefined {
        self.inner
            .call("initMouseEvent", &[type_arg.into()])
            .as_::<Undefined>()
    }
    /// The initMouseEvent method.
    /// [`MouseEvent.initMouseEvent`](https://developer.mozilla.org/en-US/docs/Web/API/MouseEvent/initMouseEvent)
    pub fn init_mouse_event1(&self, type_arg: &JsString, bubbles_arg: bool) -> Undefined {
        self.inner
            .call("initMouseEvent", &[type_arg.into(), bubbles_arg.into()])
            .as_::<Undefined>()
    }
    /// The initMouseEvent method.
    /// [`MouseEvent.initMouseEvent`](https://developer.mozilla.org/en-US/docs/Web/API/MouseEvent/initMouseEvent)
    pub fn init_mouse_event2(
        &self,
        type_arg: &JsString,
        bubbles_arg: bool,
        cancelable_arg: bool,
    ) -> Undefined {
        self.inner
            .call(
                "initMouseEvent",
                &[type_arg.into(), bubbles_arg.into(), cancelable_arg.into()],
            )
            .as_::<Undefined>()
    }
    /// The initMouseEvent method.
    /// [`MouseEvent.initMouseEvent`](https://developer.mozilla.org/en-US/docs/Web/API/MouseEvent/initMouseEvent)
    pub fn init_mouse_event3(
        &self,
        type_arg: &JsString,
        bubbles_arg: bool,
        cancelable_arg: bool,
        view_arg: &Window,
    ) -> Undefined {
        self.inner
            .call(
                "initMouseEvent",
                &[
                    type_arg.into(),
                    bubbles_arg.into(),
                    cancelable_arg.into(),
                    view_arg.into(),
                ],
            )
            .as_::<Undefined>()
    }
    /// The initMouseEvent method.
    /// [`MouseEvent.initMouseEvent`](https://developer.mozilla.org/en-US/docs/Web/API/MouseEvent/initMouseEvent)
    pub fn init_mouse_event4(
        &self,
        type_arg: &JsString,
        bubbles_arg: bool,
        cancelable_arg: bool,
        view_arg: &Window,
        detail_arg: i32,
    ) -> Undefined {
        self.inner
            .call(
                "initMouseEvent",
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
    /// The initMouseEvent method.
    /// [`MouseEvent.initMouseEvent`](https://developer.mozilla.org/en-US/docs/Web/API/MouseEvent/initMouseEvent)
    pub fn init_mouse_event5(
        &self,
        type_arg: &JsString,
        bubbles_arg: bool,
        cancelable_arg: bool,
        view_arg: &Window,
        detail_arg: i32,
        screen_x_arg: i32,
    ) -> Undefined {
        self.inner
            .call(
                "initMouseEvent",
                &[
                    type_arg.into(),
                    bubbles_arg.into(),
                    cancelable_arg.into(),
                    view_arg.into(),
                    detail_arg.into(),
                    screen_x_arg.into(),
                ],
            )
            .as_::<Undefined>()
    }
    /// The initMouseEvent method.
    /// [`MouseEvent.initMouseEvent`](https://developer.mozilla.org/en-US/docs/Web/API/MouseEvent/initMouseEvent)
    pub fn init_mouse_event6(
        &self,
        type_arg: &JsString,
        bubbles_arg: bool,
        cancelable_arg: bool,
        view_arg: &Window,
        detail_arg: i32,
        screen_x_arg: i32,
        screen_y_arg: i32,
    ) -> Undefined {
        self.inner
            .call(
                "initMouseEvent",
                &[
                    type_arg.into(),
                    bubbles_arg.into(),
                    cancelable_arg.into(),
                    view_arg.into(),
                    detail_arg.into(),
                    screen_x_arg.into(),
                    screen_y_arg.into(),
                ],
            )
            .as_::<Undefined>()
    }
    /// The initMouseEvent method.
    /// [`MouseEvent.initMouseEvent`](https://developer.mozilla.org/en-US/docs/Web/API/MouseEvent/initMouseEvent)
    pub fn init_mouse_event7(
        &self,
        type_arg: &JsString,
        bubbles_arg: bool,
        cancelable_arg: bool,
        view_arg: &Window,
        detail_arg: i32,
        screen_x_arg: i32,
        screen_y_arg: i32,
        client_x_arg: i32,
    ) -> Undefined {
        self.inner
            .call(
                "initMouseEvent",
                &[
                    type_arg.into(),
                    bubbles_arg.into(),
                    cancelable_arg.into(),
                    view_arg.into(),
                    detail_arg.into(),
                    screen_x_arg.into(),
                    screen_y_arg.into(),
                    client_x_arg.into(),
                ],
            )
            .as_::<Undefined>()
    }
    /// The initMouseEvent method.
    /// [`MouseEvent.initMouseEvent`](https://developer.mozilla.org/en-US/docs/Web/API/MouseEvent/initMouseEvent)
    pub fn init_mouse_event8(
        &self,
        type_arg: &JsString,
        bubbles_arg: bool,
        cancelable_arg: bool,
        view_arg: &Window,
        detail_arg: i32,
        screen_x_arg: i32,
        screen_y_arg: i32,
        client_x_arg: i32,
        client_y_arg: i32,
    ) -> Undefined {
        self.inner
            .call(
                "initMouseEvent",
                &[
                    type_arg.into(),
                    bubbles_arg.into(),
                    cancelable_arg.into(),
                    view_arg.into(),
                    detail_arg.into(),
                    screen_x_arg.into(),
                    screen_y_arg.into(),
                    client_x_arg.into(),
                    client_y_arg.into(),
                ],
            )
            .as_::<Undefined>()
    }
    /// The initMouseEvent method.
    /// [`MouseEvent.initMouseEvent`](https://developer.mozilla.org/en-US/docs/Web/API/MouseEvent/initMouseEvent)
    pub fn init_mouse_event9(
        &self,
        type_arg: &JsString,
        bubbles_arg: bool,
        cancelable_arg: bool,
        view_arg: &Window,
        detail_arg: i32,
        screen_x_arg: i32,
        screen_y_arg: i32,
        client_x_arg: i32,
        client_y_arg: i32,
        ctrl_key_arg: bool,
    ) -> Undefined {
        self.inner
            .call(
                "initMouseEvent",
                &[
                    type_arg.into(),
                    bubbles_arg.into(),
                    cancelable_arg.into(),
                    view_arg.into(),
                    detail_arg.into(),
                    screen_x_arg.into(),
                    screen_y_arg.into(),
                    client_x_arg.into(),
                    client_y_arg.into(),
                    ctrl_key_arg.into(),
                ],
            )
            .as_::<Undefined>()
    }
    /// The initMouseEvent method.
    /// [`MouseEvent.initMouseEvent`](https://developer.mozilla.org/en-US/docs/Web/API/MouseEvent/initMouseEvent)
    pub fn init_mouse_event10(
        &self,
        type_arg: &JsString,
        bubbles_arg: bool,
        cancelable_arg: bool,
        view_arg: &Window,
        detail_arg: i32,
        screen_x_arg: i32,
        screen_y_arg: i32,
        client_x_arg: i32,
        client_y_arg: i32,
        ctrl_key_arg: bool,
        alt_key_arg: bool,
    ) -> Undefined {
        self.inner
            .call(
                "initMouseEvent",
                &[
                    type_arg.into(),
                    bubbles_arg.into(),
                    cancelable_arg.into(),
                    view_arg.into(),
                    detail_arg.into(),
                    screen_x_arg.into(),
                    screen_y_arg.into(),
                    client_x_arg.into(),
                    client_y_arg.into(),
                    ctrl_key_arg.into(),
                    alt_key_arg.into(),
                ],
            )
            .as_::<Undefined>()
    }
    /// The initMouseEvent method.
    /// [`MouseEvent.initMouseEvent`](https://developer.mozilla.org/en-US/docs/Web/API/MouseEvent/initMouseEvent)
    pub fn init_mouse_event11(
        &self,
        type_arg: &JsString,
        bubbles_arg: bool,
        cancelable_arg: bool,
        view_arg: &Window,
        detail_arg: i32,
        screen_x_arg: i32,
        screen_y_arg: i32,
        client_x_arg: i32,
        client_y_arg: i32,
        ctrl_key_arg: bool,
        alt_key_arg: bool,
        shift_key_arg: bool,
    ) -> Undefined {
        self.inner
            .call(
                "initMouseEvent",
                &[
                    type_arg.into(),
                    bubbles_arg.into(),
                    cancelable_arg.into(),
                    view_arg.into(),
                    detail_arg.into(),
                    screen_x_arg.into(),
                    screen_y_arg.into(),
                    client_x_arg.into(),
                    client_y_arg.into(),
                    ctrl_key_arg.into(),
                    alt_key_arg.into(),
                    shift_key_arg.into(),
                ],
            )
            .as_::<Undefined>()
    }
    /// The initMouseEvent method.
    /// [`MouseEvent.initMouseEvent`](https://developer.mozilla.org/en-US/docs/Web/API/MouseEvent/initMouseEvent)
    pub fn init_mouse_event12(
        &self,
        type_arg: &JsString,
        bubbles_arg: bool,
        cancelable_arg: bool,
        view_arg: &Window,
        detail_arg: i32,
        screen_x_arg: i32,
        screen_y_arg: i32,
        client_x_arg: i32,
        client_y_arg: i32,
        ctrl_key_arg: bool,
        alt_key_arg: bool,
        shift_key_arg: bool,
        meta_key_arg: bool,
    ) -> Undefined {
        self.inner
            .call(
                "initMouseEvent",
                &[
                    type_arg.into(),
                    bubbles_arg.into(),
                    cancelable_arg.into(),
                    view_arg.into(),
                    detail_arg.into(),
                    screen_x_arg.into(),
                    screen_y_arg.into(),
                    client_x_arg.into(),
                    client_y_arg.into(),
                    ctrl_key_arg.into(),
                    alt_key_arg.into(),
                    shift_key_arg.into(),
                    meta_key_arg.into(),
                ],
            )
            .as_::<Undefined>()
    }
    /// The initMouseEvent method.
    /// [`MouseEvent.initMouseEvent`](https://developer.mozilla.org/en-US/docs/Web/API/MouseEvent/initMouseEvent)
    pub fn init_mouse_event13(
        &self,
        type_arg: &JsString,
        bubbles_arg: bool,
        cancelable_arg: bool,
        view_arg: &Window,
        detail_arg: i32,
        screen_x_arg: i32,
        screen_y_arg: i32,
        client_x_arg: i32,
        client_y_arg: i32,
        ctrl_key_arg: bool,
        alt_key_arg: bool,
        shift_key_arg: bool,
        meta_key_arg: bool,
        button_arg: i16,
    ) -> Undefined {
        self.inner
            .call(
                "initMouseEvent",
                &[
                    type_arg.into(),
                    bubbles_arg.into(),
                    cancelable_arg.into(),
                    view_arg.into(),
                    detail_arg.into(),
                    screen_x_arg.into(),
                    screen_y_arg.into(),
                    client_x_arg.into(),
                    client_y_arg.into(),
                    ctrl_key_arg.into(),
                    alt_key_arg.into(),
                    shift_key_arg.into(),
                    meta_key_arg.into(),
                    button_arg.into(),
                ],
            )
            .as_::<Undefined>()
    }
    /// The initMouseEvent method.
    /// [`MouseEvent.initMouseEvent`](https://developer.mozilla.org/en-US/docs/Web/API/MouseEvent/initMouseEvent)
    pub fn init_mouse_event14(
        &self,
        type_arg: &JsString,
        bubbles_arg: bool,
        cancelable_arg: bool,
        view_arg: &Window,
        detail_arg: i32,
        screen_x_arg: i32,
        screen_y_arg: i32,
        client_x_arg: i32,
        client_y_arg: i32,
        ctrl_key_arg: bool,
        alt_key_arg: bool,
        shift_key_arg: bool,
        meta_key_arg: bool,
        button_arg: i16,
        related_target_arg: &EventTarget,
    ) -> Undefined {
        self.inner
            .call(
                "initMouseEvent",
                &[
                    type_arg.into(),
                    bubbles_arg.into(),
                    cancelable_arg.into(),
                    view_arg.into(),
                    detail_arg.into(),
                    screen_x_arg.into(),
                    screen_y_arg.into(),
                    client_x_arg.into(),
                    client_y_arg.into(),
                    ctrl_key_arg.into(),
                    alt_key_arg.into(),
                    shift_key_arg.into(),
                    meta_key_arg.into(),
                    button_arg.into(),
                    related_target_arg.into(),
                ],
            )
            .as_::<Undefined>()
    }
}
