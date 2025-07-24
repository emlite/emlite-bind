use super::*;

/// The BluetoothLEScanFilter class.
/// [`BluetoothLEScanFilter`](https://developer.mozilla.org/en-US/docs/Web/API/BluetoothLEScanFilter)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct BluetoothLEScanFilter {
    inner: Any,
}
impl FromVal for BluetoothLEScanFilter {
    fn from_val(v: &Any) -> Self {
        BluetoothLEScanFilter {
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
impl core::ops::Deref for BluetoothLEScanFilter {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for BluetoothLEScanFilter {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for BluetoothLEScanFilter {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for BluetoothLEScanFilter {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<BluetoothLEScanFilter> for Any {
    fn from(s: BluetoothLEScanFilter) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&BluetoothLEScanFilter> for Any {
    fn from(s: &BluetoothLEScanFilter) -> Any {
        s.inner.clone().into()
    }
}
jsbind::utils::impl_dyn_cast!(BluetoothLEScanFilter);

impl BluetoothLEScanFilter {
    /// The `new BluetoothLEScanFilter(..)` constructor, creating a new BluetoothLEScanFilter instance
    pub fn new0() -> BluetoothLEScanFilter {
        Self {
            inner: Any::global("BluetoothLEScanFilter").new(&[]).as_::<Any>(),
        }
    }

    /// The `new BluetoothLEScanFilter(..)` constructor, creating a new BluetoothLEScanFilter instance
    pub fn new1(init: &Any) -> BluetoothLEScanFilter {
        Self {
            inner: Any::global("BluetoothLEScanFilter")
                .new(&[init.into()])
                .as_::<Any>(),
        }
    }
}
impl BluetoothLEScanFilter {
    /// Getter of the `name` attribute.
    /// [`BluetoothLEScanFilter.name`](https://developer.mozilla.org/en-US/docs/Web/API/BluetoothLEScanFilter/name)
    pub fn name(&self) -> DOMString {
        self.inner.get("name").as_::<DOMString>()
    }
}
impl BluetoothLEScanFilter {
    /// Getter of the `namePrefix` attribute.
    /// [`BluetoothLEScanFilter.namePrefix`](https://developer.mozilla.org/en-US/docs/Web/API/BluetoothLEScanFilter/namePrefix)
    pub fn name_prefix(&self) -> DOMString {
        self.inner.get("namePrefix").as_::<DOMString>()
    }
}
impl BluetoothLEScanFilter {
    /// Getter of the `services` attribute.
    /// [`BluetoothLEScanFilter.services`](https://developer.mozilla.org/en-US/docs/Web/API/BluetoothLEScanFilter/services)
    pub fn services(&self) -> FrozenArray<Any> {
        self.inner.get("services").as_::<FrozenArray<Any>>()
    }
}
impl BluetoothLEScanFilter {
    /// Getter of the `manufacturerData` attribute.
    /// [`BluetoothLEScanFilter.manufacturerData`](https://developer.mozilla.org/en-US/docs/Web/API/BluetoothLEScanFilter/manufacturerData)
    pub fn manufacturer_data(&self) -> BluetoothManufacturerDataFilter {
        self.inner
            .get("manufacturerData")
            .as_::<BluetoothManufacturerDataFilter>()
    }
}
impl BluetoothLEScanFilter {
    /// Getter of the `serviceData` attribute.
    /// [`BluetoothLEScanFilter.serviceData`](https://developer.mozilla.org/en-US/docs/Web/API/BluetoothLEScanFilter/serviceData)
    pub fn service_data(&self) -> BluetoothServiceDataFilter {
        self.inner
            .get("serviceData")
            .as_::<BluetoothServiceDataFilter>()
    }
}
