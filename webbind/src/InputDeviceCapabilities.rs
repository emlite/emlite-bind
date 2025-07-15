use super::*;




#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct InputDeviceCapabilities {
    inner: emlite::Val,
}
impl FromVal for InputDeviceCapabilities {
    fn from_val(v: &emlite::Val) -> Self {
        InputDeviceCapabilities { inner: emlite::Val::from_val(v) }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for InputDeviceCapabilities {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for InputDeviceCapabilities {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for InputDeviceCapabilities {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for InputDeviceCapabilities {
    fn as_mut(&mut self) -> &mut emlite::Val {
      &mut self.inner
  }
}
impl From<InputDeviceCapabilities> for emlite::Val {
    fn from(s: InputDeviceCapabilities) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}
jsbind::utils::impl_dyn_cast!(InputDeviceCapabilities);



impl InputDeviceCapabilities {
    pub fn new0() -> InputDeviceCapabilities {
        Self {
            inner: emlite::Val::global("InputDeviceCapabilities").new(&[]).as_::<emlite::Val>(),
        }
    }

    pub fn new1(device_init_dict: Any) -> InputDeviceCapabilities {
        Self {
            inner: emlite::Val::global("InputDeviceCapabilities").new(&[device_init_dict.into()]).as_::<emlite::Val>(),
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
