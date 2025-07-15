use super::*;




#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct HIDDeviceRequestOptions {
    inner: emlite::Val,
}
impl FromVal for HIDDeviceRequestOptions {
    fn from_val(v: &emlite::Val) -> Self {
        HIDDeviceRequestOptions { inner: v.clone() }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for HIDDeviceRequestOptions {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for HIDDeviceRequestOptions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for HIDDeviceRequestOptions {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for HIDDeviceRequestOptions {
    fn as_mut(&mut self) -> &mut emlite::Val {
      &mut self.inner
  }
}
impl From<HIDDeviceRequestOptions> for emlite::Val {
    fn from(s: HIDDeviceRequestOptions) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl HIDDeviceRequestOptions {
    pub fn filters(&self) -> Sequence<Any> {
        self.inner.get("filters").as_::<Sequence<Any>>()
    }

    pub fn set_filters(&mut self, value: Sequence<Any>) {
        self.inner.set("filters", value);
    }

}
impl HIDDeviceRequestOptions {
    pub fn exclusion_filters(&self) -> Sequence<Any> {
        self.inner.get("exclusionFilters").as_::<Sequence<Any>>()
    }

    pub fn set_exclusion_filters(&mut self, value: Sequence<Any>) {
        self.inner.set("exclusionFilters", value);
    }

}
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct HID {
    inner: EventTarget,
}
impl FromVal for HID {
    fn from_val(v: &emlite::Val) -> Self {
        HID { inner: EventTarget::from_val(v) }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for HID {
    type Target = EventTarget;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for HID {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for HID {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for HID {
    fn as_mut(&mut self) -> &mut emlite::Val {
      &mut self.inner
  }
}
impl From<HID> for emlite::Val {
    fn from(s: HID) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}
jsbind::utils::impl_dyn_cast!(HID);


impl HID {
    pub fn onconnect(&self) -> Any {
        self.inner.get("onconnect").as_::<Any>()
    }

    pub fn set_onconnect(&mut self, value: Any) {
        self.inner.set("onconnect", value);
    }

}
impl HID {
    pub fn ondisconnect(&self) -> Any {
        self.inner.get("ondisconnect").as_::<Any>()
    }

    pub fn set_ondisconnect(&mut self, value: Any) {
        self.inner.set("ondisconnect", value);
    }

}
impl HID {
    pub fn get_devices(&self, ) -> Promise {
        self.inner.call("getDevices", &[]).as_::<Promise>()
    }

}
impl HID {
    pub fn request_device(&self, options: HIDDeviceRequestOptions) -> Promise {
        self.inner.call("requestDevice", &[options.into(), ]).as_::<Promise>()
    }

}
