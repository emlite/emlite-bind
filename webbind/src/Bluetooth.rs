use super::*;

/// The Bluetooth class.
/// [`Bluetooth`](https://developer.mozilla.org/en-US/docs/Web/API/Bluetooth)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct Bluetooth {
    inner: EventTarget,
}

impl FromVal for Bluetooth {
    fn from_val(v: &Any) -> Self {
        Bluetooth {
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

impl core::ops::Deref for Bluetooth {
    type Target = EventTarget;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for Bluetooth {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for Bluetooth {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for Bluetooth {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}

impl From<Bluetooth> for Any {
    fn from(s: Bluetooth) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&Bluetooth> for Any {
    fn from(s: &Bluetooth) -> Any {
        s.inner.clone().into()
    }
}

jsbind::utils::impl_dyn_cast!(Bluetooth);

impl Bluetooth {
    /// Getter of the `onavailabilitychanged` attribute.
    /// [`Bluetooth.onavailabilitychanged`](https://developer.mozilla.org/en-US/docs/Web/API/Bluetooth/onavailabilitychanged)
    pub fn onavailabilitychanged(&self) -> Any {
        self.inner.get("onavailabilitychanged").as_::<Any>()
    }

    /// Setter of the `onavailabilitychanged` attribute.
    /// [`Bluetooth.onavailabilitychanged`](https://developer.mozilla.org/en-US/docs/Web/API/Bluetooth/onavailabilitychanged)
    pub fn set_onavailabilitychanged(&mut self, value: &Any) {
        self.inner.set("onavailabilitychanged", value);
    }
}
impl Bluetooth {
    /// Getter of the `referringDevice` attribute.
    /// [`Bluetooth.referringDevice`](https://developer.mozilla.org/en-US/docs/Web/API/Bluetooth/referringDevice)
    pub fn referring_device(&self) -> BluetoothDevice {
        self.inner.get("referringDevice").as_::<BluetoothDevice>()
    }
}
impl Bluetooth {
    /// Getter of the `onadvertisementreceived` attribute.
    /// [`Bluetooth.onadvertisementreceived`](https://developer.mozilla.org/en-US/docs/Web/API/Bluetooth/onadvertisementreceived)
    pub fn onadvertisementreceived(&self) -> Any {
        self.inner.get("onadvertisementreceived").as_::<Any>()
    }

    /// Setter of the `onadvertisementreceived` attribute.
    /// [`Bluetooth.onadvertisementreceived`](https://developer.mozilla.org/en-US/docs/Web/API/Bluetooth/onadvertisementreceived)
    pub fn set_onadvertisementreceived(&mut self, value: &Any) {
        self.inner.set("onadvertisementreceived", value);
    }
}
impl Bluetooth {
    /// Getter of the `ongattserverdisconnected` attribute.
    /// [`Bluetooth.ongattserverdisconnected`](https://developer.mozilla.org/en-US/docs/Web/API/Bluetooth/ongattserverdisconnected)
    pub fn ongattserverdisconnected(&self) -> Any {
        self.inner.get("ongattserverdisconnected").as_::<Any>()
    }

    /// Setter of the `ongattserverdisconnected` attribute.
    /// [`Bluetooth.ongattserverdisconnected`](https://developer.mozilla.org/en-US/docs/Web/API/Bluetooth/ongattserverdisconnected)
    pub fn set_ongattserverdisconnected(&mut self, value: &Any) {
        self.inner.set("ongattserverdisconnected", value);
    }
}
impl Bluetooth {
    /// Getter of the `oncharacteristicvaluechanged` attribute.
    /// [`Bluetooth.oncharacteristicvaluechanged`](https://developer.mozilla.org/en-US/docs/Web/API/Bluetooth/oncharacteristicvaluechanged)
    pub fn oncharacteristicvaluechanged(&self) -> Any {
        self.inner.get("oncharacteristicvaluechanged").as_::<Any>()
    }

    /// Setter of the `oncharacteristicvaluechanged` attribute.
    /// [`Bluetooth.oncharacteristicvaluechanged`](https://developer.mozilla.org/en-US/docs/Web/API/Bluetooth/oncharacteristicvaluechanged)
    pub fn set_oncharacteristicvaluechanged(&mut self, value: &Any) {
        self.inner.set("oncharacteristicvaluechanged", value);
    }
}
impl Bluetooth {
    /// Getter of the `onserviceadded` attribute.
    /// [`Bluetooth.onserviceadded`](https://developer.mozilla.org/en-US/docs/Web/API/Bluetooth/onserviceadded)
    pub fn onserviceadded(&self) -> Any {
        self.inner.get("onserviceadded").as_::<Any>()
    }

    /// Setter of the `onserviceadded` attribute.
    /// [`Bluetooth.onserviceadded`](https://developer.mozilla.org/en-US/docs/Web/API/Bluetooth/onserviceadded)
    pub fn set_onserviceadded(&mut self, value: &Any) {
        self.inner.set("onserviceadded", value);
    }
}
impl Bluetooth {
    /// Getter of the `onservicechanged` attribute.
    /// [`Bluetooth.onservicechanged`](https://developer.mozilla.org/en-US/docs/Web/API/Bluetooth/onservicechanged)
    pub fn onservicechanged(&self) -> Any {
        self.inner.get("onservicechanged").as_::<Any>()
    }

    /// Setter of the `onservicechanged` attribute.
    /// [`Bluetooth.onservicechanged`](https://developer.mozilla.org/en-US/docs/Web/API/Bluetooth/onservicechanged)
    pub fn set_onservicechanged(&mut self, value: &Any) {
        self.inner.set("onservicechanged", value);
    }
}
impl Bluetooth {
    /// Getter of the `onserviceremoved` attribute.
    /// [`Bluetooth.onserviceremoved`](https://developer.mozilla.org/en-US/docs/Web/API/Bluetooth/onserviceremoved)
    pub fn onserviceremoved(&self) -> Any {
        self.inner.get("onserviceremoved").as_::<Any>()
    }

    /// Setter of the `onserviceremoved` attribute.
    /// [`Bluetooth.onserviceremoved`](https://developer.mozilla.org/en-US/docs/Web/API/Bluetooth/onserviceremoved)
    pub fn set_onserviceremoved(&mut self, value: &Any) {
        self.inner.set("onserviceremoved", value);
    }
}
impl Bluetooth {
    /// The getAvailability method.
    /// [`Bluetooth.getAvailability`](https://developer.mozilla.org/en-US/docs/Web/API/Bluetooth/getAvailability)
    pub fn get_availability(&self) -> Promise<bool> {
        self.inner
            .call("getAvailability", &[])
            .as_::<Promise<bool>>()
    }
}
impl Bluetooth {
    /// The getDevices method.
    /// [`Bluetooth.getDevices`](https://developer.mozilla.org/en-US/docs/Web/API/Bluetooth/getDevices)
    pub fn get_devices(&self) -> Promise<TypedArray<BluetoothDevice>> {
        self.inner
            .call("getDevices", &[])
            .as_::<Promise<TypedArray<BluetoothDevice>>>()
    }
}
impl Bluetooth {
    /// The requestDevice method.
    /// [`Bluetooth.requestDevice`](https://developer.mozilla.org/en-US/docs/Web/API/Bluetooth/requestDevice)
    pub fn request_device0(&self) -> Promise<BluetoothDevice> {
        self.inner
            .call("requestDevice", &[])
            .as_::<Promise<BluetoothDevice>>()
    }
    /// The requestDevice method.
    /// [`Bluetooth.requestDevice`](https://developer.mozilla.org/en-US/docs/Web/API/Bluetooth/requestDevice)
    pub fn request_device1(&self, options: &RequestDeviceOptions) -> Promise<BluetoothDevice> {
        self.inner
            .call("requestDevice", &[options.into()])
            .as_::<Promise<BluetoothDevice>>()
    }
}
impl Bluetooth {
    /// The requestLEScan method.
    /// [`Bluetooth.requestLEScan`](https://developer.mozilla.org/en-US/docs/Web/API/Bluetooth/requestLEScan)
    pub fn request_le_scan0(&self) -> Promise<BluetoothLEScan> {
        self.inner
            .call("requestLEScan", &[])
            .as_::<Promise<BluetoothLEScan>>()
    }
    /// The requestLEScan method.
    /// [`Bluetooth.requestLEScan`](https://developer.mozilla.org/en-US/docs/Web/API/Bluetooth/requestLEScan)
    pub fn request_le_scan1(&self, options: &BluetoothLEScanOptions) -> Promise<BluetoothLEScan> {
        self.inner
            .call("requestLEScan", &[options.into()])
            .as_::<Promise<BluetoothLEScan>>()
    }
}
