use super::*;

#[derive(Clone, Debug)]
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
impl std::ops::Deref for SerialPortRequestOptions {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for SerialPortRequestOptions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<SerialPortRequestOptions> for emlite::Val {
    fn from(s: SerialPortRequestOptions) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl SerialPortRequestOptions {
    pub fn filters(&self) -> jsbind::Sequence<jsbind::Any> {
        self.inner
            .get("filters")
            .as_::<jsbind::Sequence<jsbind::Any>>()
    }

    pub fn set_filters(&mut self, value: jsbind::Sequence<jsbind::Any>) {
        self.inner.set("filters", value);
    }
}
impl SerialPortRequestOptions {
    pub fn allowed_bluetooth_service_class_ids(&self) -> jsbind::Sequence<jsbind::Any> {
        self.inner
            .get("allowedBluetoothServiceClassIds")
            .as_::<jsbind::Sequence<jsbind::Any>>()
    }

    pub fn set_allowed_bluetooth_service_class_ids(
        &mut self,
        value: jsbind::Sequence<jsbind::Any>,
    ) {
        self.inner.set("allowedBluetoothServiceClassIds", value);
    }
}
#[derive(Clone, Debug)]
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
impl std::ops::Deref for Serial {
    type Target = EventTarget;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for Serial {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<Serial> for emlite::Val {
    fn from(s: Serial) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl Serial {
    pub fn onconnect(&self) -> jsbind::Any {
        self.inner.get("onconnect").as_::<jsbind::Any>()
    }

    pub fn set_onconnect(&mut self, value: jsbind::Any) {
        self.inner.set("onconnect", value);
    }
}
impl Serial {
    pub fn ondisconnect(&self) -> jsbind::Any {
        self.inner.get("ondisconnect").as_::<jsbind::Any>()
    }

    pub fn set_ondisconnect(&mut self, value: jsbind::Any) {
        self.inner.set("ondisconnect", value);
    }
}
impl Serial {
    pub fn get_ports(&self) -> jsbind::Promise {
        self.inner.call("getPorts", &[]).as_::<jsbind::Promise>()
    }
}
impl Serial {
    pub fn request_port0(&self) -> jsbind::Promise {
        self.inner.call("requestPort", &[]).as_::<jsbind::Promise>()
    }

    pub fn request_port1(&self, options: SerialPortRequestOptions) -> jsbind::Promise {
        self.inner
            .call("requestPort", &[options.into()])
            .as_::<jsbind::Promise>()
    }
}
