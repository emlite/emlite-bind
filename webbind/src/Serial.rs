use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct SerialPortRequestOptions {
    inner: emlite::Val,
}
impl FromVal for SerialPortRequestOptions {
    fn from_val(v: &emlite::Val) -> Self {
        SerialPortRequestOptions { inner: v.clone() }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for SerialPortRequestOptions {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for SerialPortRequestOptions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for SerialPortRequestOptions {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for SerialPortRequestOptions {
    fn as_mut(&mut self) -> &mut emlite::Val {
        &mut self.inner
    }
}
impl From<SerialPortRequestOptions> for emlite::Val {
    fn from(s: SerialPortRequestOptions) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl SerialPortRequestOptions {
    pub fn filters(&self) -> Sequence<Any> {
        self.inner.get("filters").as_::<Sequence<Any>>()
    }

    pub fn set_filters(&mut self, value: Sequence<Any>) {
        self.inner.set("filters", value);
    }
}
impl SerialPortRequestOptions {
    pub fn allowed_bluetooth_service_class_ids(&self) -> Sequence<Any> {
        self.inner
            .get("allowedBluetoothServiceClassIds")
            .as_::<Sequence<Any>>()
    }

    pub fn set_allowed_bluetooth_service_class_ids(&mut self, value: Sequence<Any>) {
        self.inner.set("allowedBluetoothServiceClassIds", value);
    }
}
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct Serial {
    inner: EventTarget,
}
impl FromVal for Serial {
    fn from_val(v: &emlite::Val) -> Self {
        Serial {
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
impl core::ops::Deref for Serial {
    type Target = EventTarget;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for Serial {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for Serial {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for Serial {
    fn as_mut(&mut self) -> &mut emlite::Val {
        &mut self.inner
    }
}
impl From<Serial> for emlite::Val {
    fn from(s: Serial) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}
jsbind::utils::impl_dyn_cast!(Serial);

impl Serial {
    pub fn onconnect(&self) -> Any {
        self.inner.get("onconnect").as_::<Any>()
    }

    pub fn set_onconnect(&mut self, value: Any) {
        self.inner.set("onconnect", value);
    }
}
impl Serial {
    pub fn ondisconnect(&self) -> Any {
        self.inner.get("ondisconnect").as_::<Any>()
    }

    pub fn set_ondisconnect(&mut self, value: Any) {
        self.inner.set("ondisconnect", value);
    }
}
impl Serial {
    pub fn get_ports(&self) -> Promise {
        self.inner.call("getPorts", &[]).as_::<Promise>()
    }
}
impl Serial {
    pub fn request_port0(&self) -> Promise {
        self.inner.call("requestPort", &[]).as_::<Promise>()
    }

    pub fn request_port1(&self, options: SerialPortRequestOptions) -> Promise {
        self.inner
            .call("requestPort", &[options.into()])
            .as_::<Promise>()
    }
}
