use super::*;

/// The BluetoothDataFilter class.
/// [`BluetoothDataFilter`](https://developer.mozilla.org/en-US/docs/Web/API/BluetoothDataFilter)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct BluetoothDataFilter {
    inner: Any,
}

impl FromVal for BluetoothDataFilter {
    fn from_val(v: &Any) -> Self {
        BluetoothDataFilter {
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

impl core::ops::Deref for BluetoothDataFilter {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for BluetoothDataFilter {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for BluetoothDataFilter {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for BluetoothDataFilter {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}

impl From<BluetoothDataFilter> for Any {
    fn from(s: BluetoothDataFilter) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&BluetoothDataFilter> for Any {
    fn from(s: &BluetoothDataFilter) -> Any {
        s.inner.clone().into()
    }
}

jsbind::utils::impl_dyn_cast!(BluetoothDataFilter);

impl BluetoothDataFilter {
    /// Getter of the `dataPrefix` attribute.
    /// [`BluetoothDataFilter.dataPrefix`](https://developer.mozilla.org/en-US/docs/Web/API/BluetoothDataFilter/dataPrefix)
    pub fn data_prefix(&self) -> ArrayBuffer {
        self.inner.get("dataPrefix").as_::<ArrayBuffer>()
    }
}
impl BluetoothDataFilter {
    /// Getter of the `mask` attribute.
    /// [`BluetoothDataFilter.mask`](https://developer.mozilla.org/en-US/docs/Web/API/BluetoothDataFilter/mask)
    pub fn mask(&self) -> ArrayBuffer {
        self.inner.get("mask").as_::<ArrayBuffer>()
    }
}

impl BluetoothDataFilter {
    /// The `new BluetoothDataFilter(..)` constructor, creating a new BluetoothDataFilter instance
    pub fn new() -> BluetoothDataFilter {
        Self {
            inner: Any::global("BluetoothDataFilter").new(&[]).as_::<Any>(),
        }
    }
}

impl BluetoothDataFilter {
    /// The `new BluetoothDataFilter(..)` constructor, creating a new BluetoothDataFilter instance
    pub fn new_with_init(init: &BluetoothDataFilterInit) -> BluetoothDataFilter {
        Self {
            inner: Any::global("BluetoothDataFilter")
                .new(&[init.into()])
                .as_::<Any>(),
        }
    }
}
