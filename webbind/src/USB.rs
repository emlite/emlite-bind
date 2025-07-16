use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct USBDeviceRequestOptions {
    inner: Any,
}
impl FromVal for USBDeviceRequestOptions {
    fn from_val(v: &Any) -> Self {
        USBDeviceRequestOptions { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for USBDeviceRequestOptions {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for USBDeviceRequestOptions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for USBDeviceRequestOptions {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for USBDeviceRequestOptions {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<USBDeviceRequestOptions> for Any {
    fn from(s: USBDeviceRequestOptions) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&USBDeviceRequestOptions> for Any {
    fn from(s: &USBDeviceRequestOptions) -> Any {
        s.inner.clone()
    }
}

impl USBDeviceRequestOptions {
    pub fn filters(&self) -> Sequence<Any> {
        self.inner.get("filters").as_::<Sequence<Any>>()
    }

    pub fn set_filters(&mut self, value: &Sequence<Any>) {
        self.inner.set("filters", value);
    }
}
impl USBDeviceRequestOptions {
    pub fn exclusion_filters(&self) -> Sequence<Any> {
        self.inner.get("exclusionFilters").as_::<Sequence<Any>>()
    }

    pub fn set_exclusion_filters(&mut self, value: &Sequence<Any>) {
        self.inner.set("exclusionFilters", value);
    }
}
/// The USB class.
/// [`USB`](https://developer.mozilla.org/en-US/docs/Web/API/USB)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct USB {
    inner: EventTarget,
}
impl FromVal for USB {
    fn from_val(v: &Any) -> Self {
        USB {
            inner: EventTarget::from_val(v),
        }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
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
impl AsRef<Any> for USB {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for USB {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<USB> for Any {
    fn from(s: USB) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&USB> for Any {
    fn from(s: &USB) -> Any {
        s.inner.clone().into()
    }
}
jsbind::utils::impl_dyn_cast!(USB);

impl USB {
    /// Getter of the `onconnect` attribute.
    /// [`USB.onconnect`](https://developer.mozilla.org/en-US/docs/Web/API/USB/onconnect)
    pub fn onconnect(&self) -> Any {
        self.inner.get("onconnect").as_::<Any>()
    }

    /// Setter of the `onconnect` attribute.
    /// [`USB.onconnect`](https://developer.mozilla.org/en-US/docs/Web/API/USB/onconnect)
    pub fn set_onconnect(&mut self, value: &Any) {
        self.inner.set("onconnect", value);
    }
}
impl USB {
    /// Getter of the `ondisconnect` attribute.
    /// [`USB.ondisconnect`](https://developer.mozilla.org/en-US/docs/Web/API/USB/ondisconnect)
    pub fn ondisconnect(&self) -> Any {
        self.inner.get("ondisconnect").as_::<Any>()
    }

    /// Setter of the `ondisconnect` attribute.
    /// [`USB.ondisconnect`](https://developer.mozilla.org/en-US/docs/Web/API/USB/ondisconnect)
    pub fn set_ondisconnect(&mut self, value: &Any) {
        self.inner.set("ondisconnect", value);
    }
}
impl USB {
    /// The getDevices method.
    /// [`USB.getDevices`](https://developer.mozilla.org/en-US/docs/Web/API/USB/getDevices)
    pub fn get_devices(&self) -> Promise {
        self.inner.call("getDevices", &[]).as_::<Promise>()
    }
}
impl USB {
    /// The requestDevice method.
    /// [`USB.requestDevice`](https://developer.mozilla.org/en-US/docs/Web/API/USB/requestDevice)
    pub fn request_device(&self, options: &USBDeviceRequestOptions) -> Promise {
        self.inner
            .call("requestDevice", &[options.into()])
            .as_::<Promise>()
    }
}
