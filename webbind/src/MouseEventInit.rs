use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct MouseEventInit {
    inner: Any,
}
impl FromVal for MouseEventInit {
    fn from_val(v: &Any) -> Self {
        MouseEventInit { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for MouseEventInit {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for MouseEventInit {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for MouseEventInit {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for MouseEventInit {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<MouseEventInit> for Any {
    fn from(s: MouseEventInit) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&MouseEventInit> for Any {
    fn from(s: &MouseEventInit) -> Any {
        s.inner.clone()
    }
}

impl MouseEventInit {
    pub fn screen_x(&self) -> i32 {
        self.inner.get("screenX").as_::<i32>()
    }

    pub fn set_screen_x(&mut self, value: i32) {
        self.inner.set("screenX", value);
    }
}
impl MouseEventInit {
    pub fn screen_y(&self) -> i32 {
        self.inner.get("screenY").as_::<i32>()
    }

    pub fn set_screen_y(&mut self, value: i32) {
        self.inner.set("screenY", value);
    }
}
impl MouseEventInit {
    pub fn client_x(&self) -> i32 {
        self.inner.get("clientX").as_::<i32>()
    }

    pub fn set_client_x(&mut self, value: i32) {
        self.inner.set("clientX", value);
    }
}
impl MouseEventInit {
    pub fn client_y(&self) -> i32 {
        self.inner.get("clientY").as_::<i32>()
    }

    pub fn set_client_y(&mut self, value: i32) {
        self.inner.set("clientY", value);
    }
}
impl MouseEventInit {
    pub fn button(&self) -> i16 {
        self.inner.get("button").as_::<i16>()
    }

    pub fn set_button(&mut self, value: i16) {
        self.inner.set("button", value);
    }
}
impl MouseEventInit {
    pub fn buttons(&self) -> u16 {
        self.inner.get("buttons").as_::<u16>()
    }

    pub fn set_buttons(&mut self, value: u16) {
        self.inner.set("buttons", value);
    }
}
impl MouseEventInit {
    pub fn related_target(&self) -> EventTarget {
        self.inner.get("relatedTarget").as_::<EventTarget>()
    }

    pub fn set_related_target(&mut self, value: &EventTarget) {
        self.inner.set("relatedTarget", value);
    }
}
