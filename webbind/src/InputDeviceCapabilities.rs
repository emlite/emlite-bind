use super::*;

#[derive(Clone, Debug)]
pub struct InputDeviceCapabilities {
    inner: emlite::Val,
}
impl FromVal for InputDeviceCapabilities {
    fn from_val(v: &emlite::Val) -> Self {
        InputDeviceCapabilities {
            inner: emlite::Val::from_val(v),
        }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl std::ops::Deref for InputDeviceCapabilities {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for InputDeviceCapabilities {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<InputDeviceCapabilities> for emlite::Val {
    fn from(s: InputDeviceCapabilities) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl InputDeviceCapabilities {
    pub fn new0() -> InputDeviceCapabilities {
        Self {
            inner: emlite::Val::global("InputDeviceCapabilities")
                .new(&[])
                .as_::<emlite::Val>(),
        }
    }

    pub fn new1(device_init_dict: jsbind::Any) -> InputDeviceCapabilities {
        Self {
            inner: emlite::Val::global("InputDeviceCapabilities")
                .new(&[device_init_dict.into()])
                .as_::<emlite::Val>(),
        }
    }
}
impl InputDeviceCapabilities {
    pub fn fires_touch_events(&self) -> bool {
        self.inner.get("firesTouchEvents").as_::<bool>()
    }
}
impl InputDeviceCapabilities {
    pub fn pointer_movement_scrolls(&self) -> bool {
        self.inner.get("pointerMovementScrolls").as_::<bool>()
    }
}
