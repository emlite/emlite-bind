use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct MouseEvent {
    inner: UIEvent,
}
impl FromVal for MouseEvent {
    fn from_val(v: &emlite::Val) -> Self {
        MouseEvent {
            inner: UIEvent::from_val(v),
        }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
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
impl AsRef<emlite::Val> for MouseEvent {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for MouseEvent {
    fn as_mut(&mut self) -> &mut emlite::Val {
        &mut self.inner
    }
}
impl From<MouseEvent> for emlite::Val {
    fn from(s: MouseEvent) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}
impl From<&MouseEvent> for emlite::Val {
    fn from(s: &MouseEvent) -> emlite::Val {
        s.inner.clone().into()
    }
}
jsbind::utils::impl_dyn_cast!(MouseEvent);

impl MouseEvent {
    pub fn new0(type_: &str) -> MouseEvent {
        Self {
            inner: emlite::Val::global("MouseEvent")
                .new(&[type_.into()])
                .as_::<UIEvent>(),
        }
    }

    pub fn new1(type_: &str, event_init_dict: &Any) -> MouseEvent {
        Self {
            inner: emlite::Val::global("MouseEvent")
                .new(&[type_.into(), event_init_dict.into()])
                .as_::<UIEvent>(),
        }
    }
}
impl MouseEvent {
    pub fn screen_x(&self) -> i32 {
        self.inner.get("screenX").as_::<i32>()
    }
}
impl MouseEvent {
    pub fn screen_y(&self) -> i32 {
        self.inner.get("screenY").as_::<i32>()
    }
}
impl MouseEvent {
    pub fn client_x(&self) -> i32 {
        self.inner.get("clientX").as_::<i32>()
    }
}
impl MouseEvent {
    pub fn client_y(&self) -> i32 {
        self.inner.get("clientY").as_::<i32>()
    }
}
impl MouseEvent {
    pub fn layer_x(&self) -> i32 {
        self.inner.get("layerX").as_::<i32>()
    }
}
impl MouseEvent {
    pub fn layer_y(&self) -> i32 {
        self.inner.get("layerY").as_::<i32>()
    }
}
impl MouseEvent {
    pub fn ctrl_key(&self) -> bool {
        self.inner.get("ctrlKey").as_::<bool>()
    }
}
impl MouseEvent {
    pub fn shift_key(&self) -> bool {
        self.inner.get("shiftKey").as_::<bool>()
    }
}
impl MouseEvent {
    pub fn alt_key(&self) -> bool {
        self.inner.get("altKey").as_::<bool>()
    }
}
impl MouseEvent {
    pub fn meta_key(&self) -> bool {
        self.inner.get("metaKey").as_::<bool>()
    }
}
impl MouseEvent {
    pub fn button(&self) -> i16 {
        self.inner.get("button").as_::<i16>()
    }
}
impl MouseEvent {
    pub fn buttons(&self) -> u16 {
        self.inner.get("buttons").as_::<u16>()
    }
}
impl MouseEvent {
    pub fn related_target(&self) -> EventTarget {
        self.inner.get("relatedTarget").as_::<EventTarget>()
    }
}
impl MouseEvent {
    pub fn get_modifier_state(&self, key_arg: &str) -> bool {
        self.inner
            .call("getModifierState", &[key_arg.into()])
            .as_::<bool>()
    }
}
impl MouseEvent {
    pub fn page_x(&self) -> f64 {
        self.inner.get("pageX").as_::<f64>()
    }
}
impl MouseEvent {
    pub fn page_y(&self) -> f64 {
        self.inner.get("pageY").as_::<f64>()
    }
}
impl MouseEvent {
    pub fn x(&self) -> f64 {
        self.inner.get("x").as_::<f64>()
    }
}
impl MouseEvent {
    pub fn y(&self) -> f64 {
        self.inner.get("y").as_::<f64>()
    }
}
impl MouseEvent {
    pub fn offset_x(&self) -> f64 {
        self.inner.get("offsetX").as_::<f64>()
    }
}
impl MouseEvent {
    pub fn offset_y(&self) -> f64 {
        self.inner.get("offsetY").as_::<f64>()
    }
}
impl MouseEvent {
    pub fn movement_x(&self) -> f64 {
        self.inner.get("movementX").as_::<f64>()
    }
}
impl MouseEvent {
    pub fn movement_y(&self) -> f64 {
        self.inner.get("movementY").as_::<f64>()
    }
}
impl MouseEvent {
    pub fn init_mouse_event0(&self, type_arg: &str) -> Undefined {
        self.inner
            .call("initMouseEvent", &[type_arg.into()])
            .as_::<Undefined>()
    }

    pub fn init_mouse_event1(&self, type_arg: &str, bubbles_arg: bool) -> Undefined {
        self.inner
            .call("initMouseEvent", &[type_arg.into(), bubbles_arg.into()])
            .as_::<Undefined>()
    }

    pub fn init_mouse_event2(
        &self,
        type_arg: &str,
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

    pub fn init_mouse_event3(
        &self,
        type_arg: &str,
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

    pub fn init_mouse_event4(
        &self,
        type_arg: &str,
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

    pub fn init_mouse_event5(
        &self,
        type_arg: &str,
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

    pub fn init_mouse_event6(
        &self,
        type_arg: &str,
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

    pub fn init_mouse_event7(
        &self,
        type_arg: &str,
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

    pub fn init_mouse_event8(
        &self,
        type_arg: &str,
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

    pub fn init_mouse_event9(
        &self,
        type_arg: &str,
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

    pub fn init_mouse_event10(
        &self,
        type_arg: &str,
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

    pub fn init_mouse_event11(
        &self,
        type_arg: &str,
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

    pub fn init_mouse_event12(
        &self,
        type_arg: &str,
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

    pub fn init_mouse_event13(
        &self,
        type_arg: &str,
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

    pub fn init_mouse_event14(
        &self,
        type_arg: &str,
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
