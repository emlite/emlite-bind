use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct WatchAdvertisementsOptions {
    inner: emlite::Val,
}
impl FromVal for WatchAdvertisementsOptions {
    fn from_val(v: &emlite::Val) -> Self {
        WatchAdvertisementsOptions { inner: v.clone() }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for WatchAdvertisementsOptions {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for WatchAdvertisementsOptions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for WatchAdvertisementsOptions {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for WatchAdvertisementsOptions {
    fn as_mut(&mut self) -> &mut emlite::Val {
        &mut self.inner
    }
}
impl From<WatchAdvertisementsOptions> for emlite::Val {
    fn from(s: WatchAdvertisementsOptions) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}
impl From<&WatchAdvertisementsOptions> for emlite::Val {
    fn from(s: &WatchAdvertisementsOptions) -> emlite::Val {
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
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct BluetoothDevice {
    inner: EventTarget,
}
impl FromVal for BluetoothDevice {
    fn from_val(v: &emlite::Val) -> Self {
        BluetoothDevice {
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
impl AsRef<emlite::Val> for BluetoothDevice {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for BluetoothDevice {
    fn as_mut(&mut self) -> &mut emlite::Val {
        &mut self.inner
    }
}
impl From<BluetoothDevice> for emlite::Val {
    fn from(s: BluetoothDevice) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}
impl From<&BluetoothDevice> for emlite::Val {
    fn from(s: &BluetoothDevice) -> emlite::Val {
        s.inner.clone().into()
    }
}
jsbind::utils::impl_dyn_cast!(BluetoothDevice);

impl BluetoothDevice {
    pub fn id(&self) -> String {
        self.inner.get("id").as_::<String>()
    }
}
impl BluetoothDevice {
    pub fn name(&self) -> String {
        self.inner.get("name").as_::<String>()
    }
}
impl BluetoothDevice {
    pub fn gatt(&self) -> BluetoothRemoteGATTServer {
        self.inner.get("gatt").as_::<BluetoothRemoteGATTServer>()
    }
}
impl BluetoothDevice {
    pub fn forget(&self) -> Promise {
        self.inner.call("forget", &[]).as_::<Promise>()
    }
}
impl BluetoothDevice {
    pub fn watch_advertisements0(&self) -> Promise {
        self.inner.call("watchAdvertisements", &[]).as_::<Promise>()
    }

    pub fn watch_advertisements1(&self, options: &WatchAdvertisementsOptions) -> Promise {
        self.inner
            .call("watchAdvertisements", &[options.into()])
            .as_::<Promise>()
    }
}
impl BluetoothDevice {
    pub fn watching_advertisements(&self) -> bool {
        self.inner.get("watchingAdvertisements").as_::<bool>()
    }
}
impl BluetoothDevice {
    pub fn onadvertisementreceived(&self) -> Any {
        self.inner.get("onadvertisementreceived").as_::<Any>()
    }

    pub fn set_onadvertisementreceived(&mut self, value: &Any) {
        self.inner.set("onadvertisementreceived", value);
    }
}
impl BluetoothDevice {
    pub fn ongattserverdisconnected(&self) -> Any {
        self.inner.get("ongattserverdisconnected").as_::<Any>()
    }

    pub fn set_ongattserverdisconnected(&mut self, value: &Any) {
        self.inner.set("ongattserverdisconnected", value);
    }
}
impl BluetoothDevice {
    pub fn oncharacteristicvaluechanged(&self) -> Any {
        self.inner.get("oncharacteristicvaluechanged").as_::<Any>()
    }

    pub fn set_oncharacteristicvaluechanged(&mut self, value: &Any) {
        self.inner.set("oncharacteristicvaluechanged", value);
    }
}
impl BluetoothDevice {
    pub fn onserviceadded(&self) -> Any {
        self.inner.get("onserviceadded").as_::<Any>()
    }

    pub fn set_onserviceadded(&mut self, value: &Any) {
        self.inner.set("onserviceadded", value);
    }
}
impl BluetoothDevice {
    pub fn onservicechanged(&self) -> Any {
        self.inner.get("onservicechanged").as_::<Any>()
    }

    pub fn set_onservicechanged(&mut self, value: &Any) {
        self.inner.set("onservicechanged", value);
    }
}
impl BluetoothDevice {
    pub fn onserviceremoved(&self) -> Any {
        self.inner.get("onserviceremoved").as_::<Any>()
    }

    pub fn set_onserviceremoved(&mut self, value: &Any) {
        self.inner.set("onserviceremoved", value);
    }
}
