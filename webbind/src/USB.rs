use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct USBDeviceRequestOptions {
    inner: emlite::Val,
}
impl FromVal for USBDeviceRequestOptions {
    fn from_val(v: &emlite::Val) -> Self {
        USBDeviceRequestOptions { inner: v.clone() }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for USBDeviceRequestOptions {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for USBDeviceRequestOptions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for USBDeviceRequestOptions {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for USBDeviceRequestOptions {
    fn as_mut(&mut self) -> &mut emlite::Val {
        &mut self.inner
    }
}
impl From<USBDeviceRequestOptions> for emlite::Val {
    fn from(s: USBDeviceRequestOptions) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl USBDeviceRequestOptions {
    pub fn filters(&self) -> Sequence<Any> {
        self.inner.get("filters").as_::<Sequence<Any>>()
    }

    pub fn set_filters(&mut self, value: Sequence<Any>) {
        self.inner.set("filters", value);
    }
}
impl USBDeviceRequestOptions {
    pub fn exclusion_filters(&self) -> Sequence<Any> {
        self.inner.get("exclusionFilters").as_::<Sequence<Any>>()
    }

    pub fn set_exclusion_filters(&mut self, value: Sequence<Any>) {
        self.inner.set("exclusionFilters", value);
    }
}
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct USB {
    inner: EventTarget,
}
impl FromVal for USB {
    fn from_val(v: &emlite::Val) -> Self {
        USB {
            inner: EventTarget::from_val(v),
        }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for USB {
    type Target = EventTarget;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for USB {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for USB {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for USB {
    fn as_mut(&mut self) -> &mut emlite::Val {
        &mut self.inner
    }
}
impl From<USB> for emlite::Val {
    fn from(s: USB) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}
jsbind::utils::impl_dyn_cast!(USB);

impl USB {
    pub fn onconnect(&self) -> Any {
        self.inner.get("onconnect").as_::<Any>()
    }

    pub fn set_onconnect(&mut self, value: Any) {
        self.inner.set("onconnect", value);
    }
}
impl USB {
    pub fn ondisconnect(&self) -> Any {
        self.inner.get("ondisconnect").as_::<Any>()
    }

    pub fn set_ondisconnect(&mut self, value: Any) {
        self.inner.set("ondisconnect", value);
    }
}
impl USB {
    pub fn get_devices(&self) -> Promise {
        self.inner.call("getDevices", &[]).as_::<Promise>()
    }
}
impl USB {
    pub fn request_device(&self, options: USBDeviceRequestOptions) -> Promise {
        self.inner
            .call("requestDevice", &[options.into()])
            .as_::<Promise>()
    }
}
