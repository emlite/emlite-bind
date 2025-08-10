use super::*;

/// The BluetoothRemoteGATTServer class.
/// [`BluetoothRemoteGATTServer`](https://developer.mozilla.org/en-US/docs/Web/API/BluetoothRemoteGATTServer)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct BluetoothRemoteGATTServer {
    inner: Any,
}

impl FromVal for BluetoothRemoteGATTServer {
    fn from_val(v: &Any) -> Self {
        BluetoothRemoteGATTServer {
            inner: Any::from_val(v),
        }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for BluetoothRemoteGATTServer {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for BluetoothRemoteGATTServer {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for BluetoothRemoteGATTServer {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for BluetoothRemoteGATTServer {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}

impl From<BluetoothRemoteGATTServer> for Any {
    fn from(s: BluetoothRemoteGATTServer) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&BluetoothRemoteGATTServer> for Any {
    fn from(s: &BluetoothRemoteGATTServer) -> Any {
        s.inner.clone().into()
    }
}

jsbind::utils::impl_dyn_cast!(BluetoothRemoteGATTServer);

impl BluetoothRemoteGATTServer {
    /// Getter of the `device` attribute.
    /// [`BluetoothRemoteGATTServer.device`](https://developer.mozilla.org/en-US/docs/Web/API/BluetoothRemoteGATTServer/device)
    pub fn device(&self) -> BluetoothDevice {
        self.inner.get("device").as_::<BluetoothDevice>()
    }
}
impl BluetoothRemoteGATTServer {
    /// Getter of the `connected` attribute.
    /// [`BluetoothRemoteGATTServer.connected`](https://developer.mozilla.org/en-US/docs/Web/API/BluetoothRemoteGATTServer/connected)
    pub fn connected(&self) -> bool {
        self.inner.get("connected").as_::<bool>()
    }
}
impl BluetoothRemoteGATTServer {
    /// The connect method.
    /// [`BluetoothRemoteGATTServer.connect`](https://developer.mozilla.org/en-US/docs/Web/API/BluetoothRemoteGATTServer/connect)
    pub fn connect(&self) -> Promise<BluetoothRemoteGATTServer> {
        self.inner
            .call("connect", &[])
            .as_::<Promise<BluetoothRemoteGATTServer>>()
    }
}
impl BluetoothRemoteGATTServer {
    /// The disconnect method.
    /// [`BluetoothRemoteGATTServer.disconnect`](https://developer.mozilla.org/en-US/docs/Web/API/BluetoothRemoteGATTServer/disconnect)
    pub fn disconnect(&self) -> Undefined {
        self.inner.call("disconnect", &[]).as_::<Undefined>()
    }
}
impl BluetoothRemoteGATTServer {
    /// The getPrimaryService method.
    /// [`BluetoothRemoteGATTServer.getPrimaryService`](https://developer.mozilla.org/en-US/docs/Web/API/BluetoothRemoteGATTServer/getPrimaryService)
    pub fn get_primary_service(&self, service: &Any) -> Promise<BluetoothRemoteGATTService> {
        self.inner
            .call("getPrimaryService", &[service.into()])
            .as_::<Promise<BluetoothRemoteGATTService>>()
    }
}
impl BluetoothRemoteGATTServer {
    /// The getPrimaryServices method.
    /// [`BluetoothRemoteGATTServer.getPrimaryServices`](https://developer.mozilla.org/en-US/docs/Web/API/BluetoothRemoteGATTServer/getPrimaryServices)
    pub fn get_primary_services0(&self) -> Promise<TypedArray<BluetoothRemoteGATTService>> {
        self.inner
            .call("getPrimaryServices", &[])
            .as_::<Promise<TypedArray<BluetoothRemoteGATTService>>>()
    }
    /// The getPrimaryServices method.
    /// [`BluetoothRemoteGATTServer.getPrimaryServices`](https://developer.mozilla.org/en-US/docs/Web/API/BluetoothRemoteGATTServer/getPrimaryServices)
    pub fn get_primary_services1(
        &self,
        service: &Any,
    ) -> Promise<TypedArray<BluetoothRemoteGATTService>> {
        self.inner
            .call("getPrimaryServices", &[service.into()])
            .as_::<Promise<TypedArray<BluetoothRemoteGATTService>>>()
    }
}
