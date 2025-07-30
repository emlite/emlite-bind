use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct WatchAdvertisementsOptions {
    inner: Any,
}
impl FromVal for WatchAdvertisementsOptions {
    fn from_val(v: &Any) -> Self {
        WatchAdvertisementsOptions { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for WatchAdvertisementsOptions {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for WatchAdvertisementsOptions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for WatchAdvertisementsOptions {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for WatchAdvertisementsOptions {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<WatchAdvertisementsOptions> for Any {
    fn from(s: WatchAdvertisementsOptions) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&WatchAdvertisementsOptions> for Any {
    fn from(s: &WatchAdvertisementsOptions) -> Any {
        s.inner.clone()
    }
}

impl WatchAdvertisementsOptions {
    pub fn signal(&self) -> AbortSignal {
        self.inner.get("signal").as_::<AbortSignal>()
    }

    pub fn set_signal(&mut self, value: &AbortSignal) {
        self.inner.set("signal", value);
    }
}
/// The BluetoothDevice class.
/// [`BluetoothDevice`](https://developer.mozilla.org/en-US/docs/Web/API/BluetoothDevice)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct BluetoothDevice {
    inner: EventTarget,
}
impl FromVal for BluetoothDevice {
    fn from_val(v: &Any) -> Self {
        BluetoothDevice {
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
impl core::ops::Deref for BluetoothDevice {
    type Target = EventTarget;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for BluetoothDevice {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for BluetoothDevice {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for BluetoothDevice {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<BluetoothDevice> for Any {
    fn from(s: BluetoothDevice) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&BluetoothDevice> for Any {
    fn from(s: &BluetoothDevice) -> Any {
        s.inner.clone().into()
    }
}
jsbind::utils::impl_dyn_cast!(BluetoothDevice);

impl BluetoothDevice {
    /// Getter of the `id` attribute.
    /// [`BluetoothDevice.id`](https://developer.mozilla.org/en-US/docs/Web/API/BluetoothDevice/id)
    pub fn id(&self) -> JsString {
        self.inner.get("id").as_::<JsString>()
    }
}
impl BluetoothDevice {
    /// Getter of the `name` attribute.
    /// [`BluetoothDevice.name`](https://developer.mozilla.org/en-US/docs/Web/API/BluetoothDevice/name)
    pub fn name(&self) -> JsString {
        self.inner.get("name").as_::<JsString>()
    }
}
impl BluetoothDevice {
    /// Getter of the `gatt` attribute.
    /// [`BluetoothDevice.gatt`](https://developer.mozilla.org/en-US/docs/Web/API/BluetoothDevice/gatt)
    pub fn gatt(&self) -> BluetoothRemoteGATTServer {
        self.inner.get("gatt").as_::<BluetoothRemoteGATTServer>()
    }
}
impl BluetoothDevice {
    /// The forget method.
    /// [`BluetoothDevice.forget`](https://developer.mozilla.org/en-US/docs/Web/API/BluetoothDevice/forget)
    pub fn forget(&self) -> Promise<Undefined> {
        self.inner.call("forget", &[]).as_::<Promise<Undefined>>()
    }
}
impl BluetoothDevice {
    /// The watchAdvertisements method.
    /// [`BluetoothDevice.watchAdvertisements`](https://developer.mozilla.org/en-US/docs/Web/API/BluetoothDevice/watchAdvertisements)
    pub fn watch_advertisements0(&self) -> Promise<Undefined> {
        self.inner
            .call("watchAdvertisements", &[])
            .as_::<Promise<Undefined>>()
    }
    /// The watchAdvertisements method.
    /// [`BluetoothDevice.watchAdvertisements`](https://developer.mozilla.org/en-US/docs/Web/API/BluetoothDevice/watchAdvertisements)
    pub fn watch_advertisements1(
        &self,
        options: &WatchAdvertisementsOptions,
    ) -> Promise<Undefined> {
        self.inner
            .call("watchAdvertisements", &[options.into()])
            .as_::<Promise<Undefined>>()
    }
}
impl BluetoothDevice {
    /// Getter of the `watchingAdvertisements` attribute.
    /// [`BluetoothDevice.watchingAdvertisements`](https://developer.mozilla.org/en-US/docs/Web/API/BluetoothDevice/watchingAdvertisements)
    pub fn watching_advertisements(&self) -> bool {
        self.inner.get("watchingAdvertisements").as_::<bool>()
    }
}
impl BluetoothDevice {
    /// Getter of the `onadvertisementreceived` attribute.
    /// [`BluetoothDevice.onadvertisementreceived`](https://developer.mozilla.org/en-US/docs/Web/API/BluetoothDevice/onadvertisementreceived)
    pub fn onadvertisementreceived(&self) -> Any {
        self.inner.get("onadvertisementreceived").as_::<Any>()
    }

    /// Setter of the `onadvertisementreceived` attribute.
    /// [`BluetoothDevice.onadvertisementreceived`](https://developer.mozilla.org/en-US/docs/Web/API/BluetoothDevice/onadvertisementreceived)
    pub fn set_onadvertisementreceived(&mut self, value: &Any) {
        self.inner.set("onadvertisementreceived", value);
    }
}
impl BluetoothDevice {
    /// Getter of the `ongattserverdisconnected` attribute.
    /// [`BluetoothDevice.ongattserverdisconnected`](https://developer.mozilla.org/en-US/docs/Web/API/BluetoothDevice/ongattserverdisconnected)
    pub fn ongattserverdisconnected(&self) -> Any {
        self.inner.get("ongattserverdisconnected").as_::<Any>()
    }

    /// Setter of the `ongattserverdisconnected` attribute.
    /// [`BluetoothDevice.ongattserverdisconnected`](https://developer.mozilla.org/en-US/docs/Web/API/BluetoothDevice/ongattserverdisconnected)
    pub fn set_ongattserverdisconnected(&mut self, value: &Any) {
        self.inner.set("ongattserverdisconnected", value);
    }
}
impl BluetoothDevice {
    /// Getter of the `oncharacteristicvaluechanged` attribute.
    /// [`BluetoothDevice.oncharacteristicvaluechanged`](https://developer.mozilla.org/en-US/docs/Web/API/BluetoothDevice/oncharacteristicvaluechanged)
    pub fn oncharacteristicvaluechanged(&self) -> Any {
        self.inner.get("oncharacteristicvaluechanged").as_::<Any>()
    }

    /// Setter of the `oncharacteristicvaluechanged` attribute.
    /// [`BluetoothDevice.oncharacteristicvaluechanged`](https://developer.mozilla.org/en-US/docs/Web/API/BluetoothDevice/oncharacteristicvaluechanged)
    pub fn set_oncharacteristicvaluechanged(&mut self, value: &Any) {
        self.inner.set("oncharacteristicvaluechanged", value);
    }
}
impl BluetoothDevice {
    /// Getter of the `onserviceadded` attribute.
    /// [`BluetoothDevice.onserviceadded`](https://developer.mozilla.org/en-US/docs/Web/API/BluetoothDevice/onserviceadded)
    pub fn onserviceadded(&self) -> Any {
        self.inner.get("onserviceadded").as_::<Any>()
    }

    /// Setter of the `onserviceadded` attribute.
    /// [`BluetoothDevice.onserviceadded`](https://developer.mozilla.org/en-US/docs/Web/API/BluetoothDevice/onserviceadded)
    pub fn set_onserviceadded(&mut self, value: &Any) {
        self.inner.set("onserviceadded", value);
    }
}
impl BluetoothDevice {
    /// Getter of the `onservicechanged` attribute.
    /// [`BluetoothDevice.onservicechanged`](https://developer.mozilla.org/en-US/docs/Web/API/BluetoothDevice/onservicechanged)
    pub fn onservicechanged(&self) -> Any {
        self.inner.get("onservicechanged").as_::<Any>()
    }

    /// Setter of the `onservicechanged` attribute.
    /// [`BluetoothDevice.onservicechanged`](https://developer.mozilla.org/en-US/docs/Web/API/BluetoothDevice/onservicechanged)
    pub fn set_onservicechanged(&mut self, value: &Any) {
        self.inner.set("onservicechanged", value);
    }
}
impl BluetoothDevice {
    /// Getter of the `onserviceremoved` attribute.
    /// [`BluetoothDevice.onserviceremoved`](https://developer.mozilla.org/en-US/docs/Web/API/BluetoothDevice/onserviceremoved)
    pub fn onserviceremoved(&self) -> Any {
        self.inner.get("onserviceremoved").as_::<Any>()
    }

    /// Setter of the `onserviceremoved` attribute.
    /// [`BluetoothDevice.onserviceremoved`](https://developer.mozilla.org/en-US/docs/Web/API/BluetoothDevice/onserviceremoved)
    pub fn set_onserviceremoved(&mut self, value: &Any) {
        self.inner.set("onserviceremoved", value);
    }
}
