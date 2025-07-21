use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct HIDDeviceRequestOptions {
    inner: Any,
}
impl FromVal for HIDDeviceRequestOptions {
    fn from_val(v: &Any) -> Self {
        HIDDeviceRequestOptions { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for HIDDeviceRequestOptions {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for HIDDeviceRequestOptions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for HIDDeviceRequestOptions {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for HIDDeviceRequestOptions {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<HIDDeviceRequestOptions> for Any {
    fn from(s: HIDDeviceRequestOptions) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&HIDDeviceRequestOptions> for Any {
    fn from(s: &HIDDeviceRequestOptions) -> Any {
        s.inner.clone()
    }
}

impl HIDDeviceRequestOptions {
    pub fn filters(&self) -> Sequence<Any> {
        self.inner.get("filters").as_::<Sequence<Any>>()
    }

    pub fn set_filters(&mut self, value: &Sequence<Any>) {
        self.inner.set("filters", value);
    }
}
impl HIDDeviceRequestOptions {
    pub fn exclusion_filters(&self) -> Sequence<Any> {
        self.inner.get("exclusionFilters").as_::<Sequence<Any>>()
    }

    pub fn set_exclusion_filters(&mut self, value: &Sequence<Any>) {
        self.inner.set("exclusionFilters", value);
    }
}
/// The HID class.
/// [`HID`](https://developer.mozilla.org/en-US/docs/Web/API/HID)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct HID {
    inner: EventTarget,
}
impl FromVal for HID {
    fn from_val(v: &Any) -> Self {
        HID {
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
impl AsRef<Any> for HID {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for HID {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<HID> for Any {
    fn from(s: HID) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&HID> for Any {
    fn from(s: &HID) -> Any {
        s.inner.clone().into()
    }
}
jsbind::utils::impl_dyn_cast!(HID);

impl HID {
    /// Getter of the `onconnect` attribute.
    /// [`HID.onconnect`](https://developer.mozilla.org/en-US/docs/Web/API/HID/onconnect)
    pub fn onconnect(&self) -> Any {
        self.inner.get("onconnect").as_::<Any>()
    }

    /// Setter of the `onconnect` attribute.
    /// [`HID.onconnect`](https://developer.mozilla.org/en-US/docs/Web/API/HID/onconnect)
    pub fn set_onconnect(&mut self, value: &Any) {
        self.inner.set("onconnect", value);
    }
}
impl HID {
    /// Getter of the `ondisconnect` attribute.
    /// [`HID.ondisconnect`](https://developer.mozilla.org/en-US/docs/Web/API/HID/ondisconnect)
    pub fn ondisconnect(&self) -> Any {
        self.inner.get("ondisconnect").as_::<Any>()
    }

    /// Setter of the `ondisconnect` attribute.
    /// [`HID.ondisconnect`](https://developer.mozilla.org/en-US/docs/Web/API/HID/ondisconnect)
    pub fn set_ondisconnect(&mut self, value: &Any) {
        self.inner.set("ondisconnect", value);
    }
}
impl HID {
    /// The getDevices method.
    /// [`HID.getDevices`](https://developer.mozilla.org/en-US/docs/Web/API/HID/getDevices)
    pub fn get_devices(&self) -> Promise<Sequence<HIDDevice>> {
        self.inner
            .call("getDevices", &[])
            .as_::<Promise<Sequence<HIDDevice>>>()
    }
}
impl HID {
    /// The requestDevice method.
    /// [`HID.requestDevice`](https://developer.mozilla.org/en-US/docs/Web/API/HID/requestDevice)
    pub fn request_device(
        &self,
        options: &HIDDeviceRequestOptions,
    ) -> Promise<Sequence<HIDDevice>> {
        self.inner
            .call("requestDevice", &[options.into()])
            .as_::<Promise<Sequence<HIDDevice>>>()
    }
}
