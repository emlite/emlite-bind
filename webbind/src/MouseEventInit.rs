use super::*;

/// The MouseEventInit dictionary.
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
    /// Getter of the `screenX` attribute.
    pub fn screen_x(&self) -> i32 {
        self.inner.get("screenX").as_::<i32>()
    }

    /// Setter of the `screenX` attribute.
    pub fn set_screen_x(&mut self, value: i32) {
        self.inner.set("screenX", value);
    }
}
impl MouseEventInit {
    /// Getter of the `screenY` attribute.
    pub fn screen_y(&self) -> i32 {
        self.inner.get("screenY").as_::<i32>()
    }

    /// Setter of the `screenY` attribute.
    pub fn set_screen_y(&mut self, value: i32) {
        self.inner.set("screenY", value);
    }
}
impl MouseEventInit {
    /// Getter of the `clientX` attribute.
    pub fn client_x(&self) -> i32 {
        self.inner.get("clientX").as_::<i32>()
    }

    /// Setter of the `clientX` attribute.
    pub fn set_client_x(&mut self, value: i32) {
        self.inner.set("clientX", value);
    }
}
impl MouseEventInit {
    /// Getter of the `clientY` attribute.
    pub fn client_y(&self) -> i32 {
        self.inner.get("clientY").as_::<i32>()
    }

    /// Setter of the `clientY` attribute.
    pub fn set_client_y(&mut self, value: i32) {
        self.inner.set("clientY", value);
    }
}
impl MouseEventInit {
    /// Getter of the `button` attribute.
    pub fn button(&self) -> i16 {
        self.inner.get("button").as_::<i16>()
    }

    /// Setter of the `button` attribute.
    pub fn set_button(&mut self, value: i16) {
        self.inner.set("button", value);
    }
}
impl MouseEventInit {
    /// Getter of the `buttons` attribute.
    pub fn buttons(&self) -> u16 {
        self.inner.get("buttons").as_::<u16>()
    }

    /// Setter of the `buttons` attribute.
    pub fn set_buttons(&mut self, value: u16) {
        self.inner.set("buttons", value);
    }
}
impl MouseEventInit {
    /// Getter of the `relatedTarget` attribute.
    pub fn related_target(&self) -> EventTarget {
        self.inner.get("relatedTarget").as_::<EventTarget>()
    }

    /// Setter of the `relatedTarget` attribute.
    pub fn set_related_target(&mut self, value: &EventTarget) {
        self.inner.set("relatedTarget", value);
    }
}
