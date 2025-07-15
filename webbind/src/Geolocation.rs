use super::*;




#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct PositionOptions {
    inner: emlite::Val,
}
impl FromVal for PositionOptions {
    fn from_val(v: &emlite::Val) -> Self {
        PositionOptions { inner: v.clone() }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for PositionOptions {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for PositionOptions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for PositionOptions {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for PositionOptions {
    fn as_mut(&mut self) -> &mut emlite::Val {
      &mut self.inner
  }
}
impl From<PositionOptions> for emlite::Val {
    fn from(s: PositionOptions) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl PositionOptions {
    pub fn enable_high_accuracy(&self) -> bool {
        self.inner.get("enableHighAccuracy").as_::<bool>()
    }

    pub fn set_enable_high_accuracy(&mut self, value: bool) {
        self.inner.set("enableHighAccuracy", value);
    }

}
impl PositionOptions {
    pub fn timeout(&self) -> u32 {
        self.inner.get("timeout").as_::<u32>()
    }

    pub fn set_timeout(&mut self, value: u32) {
        self.inner.set("timeout", value);
    }

}
impl PositionOptions {
    pub fn maximum_age(&self) -> u32 {
        self.inner.get("maximumAge").as_::<u32>()
    }

    pub fn set_maximum_age(&mut self, value: u32) {
        self.inner.set("maximumAge", value);
    }

}
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct Geolocation {
    inner: emlite::Val,
}
impl FromVal for Geolocation {
    fn from_val(v: &emlite::Val) -> Self {
        Geolocation { inner: emlite::Val::from_val(v) }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for Geolocation {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for Geolocation {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for Geolocation {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for Geolocation {
    fn as_mut(&mut self) -> &mut emlite::Val {
      &mut self.inner
  }
}
impl From<Geolocation> for emlite::Val {
    fn from(s: Geolocation) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}
jsbind::utils::impl_dyn_cast!(Geolocation);


impl Geolocation {
    pub fn get_current_position0(&self, success_callback: Function) -> Undefined {
        self.inner.call("getCurrentPosition", &[success_callback.into(), ]).as_::<Undefined>()
    }

    pub fn get_current_position1(&self, success_callback: Function, error_callback: Function) -> Undefined {
        self.inner.call("getCurrentPosition", &[success_callback.into(), error_callback.into(), ]).as_::<Undefined>()
    }

    pub fn get_current_position2(&self, success_callback: Function, error_callback: Function, options: PositionOptions) -> Undefined {
        self.inner.call("getCurrentPosition", &[success_callback.into(), error_callback.into(), options.into(), ]).as_::<Undefined>()
    }

}
impl Geolocation {
    pub fn watch_position0(&self, success_callback: Function) -> i32 {
        self.inner.call("watchPosition", &[success_callback.into(), ]).as_::<i32>()
    }

    pub fn watch_position1(&self, success_callback: Function, error_callback: Function) -> i32 {
        self.inner.call("watchPosition", &[success_callback.into(), error_callback.into(), ]).as_::<i32>()
    }

    pub fn watch_position2(&self, success_callback: Function, error_callback: Function, options: PositionOptions) -> i32 {
        self.inner.call("watchPosition", &[success_callback.into(), error_callback.into(), options.into(), ]).as_::<i32>()
    }

}
impl Geolocation {
    pub fn clear_watch(&self, watch_id: i32) -> Undefined {
        self.inner.call("clearWatch", &[watch_id.into(), ]).as_::<Undefined>()
    }

}
