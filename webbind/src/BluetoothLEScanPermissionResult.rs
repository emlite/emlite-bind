use super::*;

/// The BluetoothLEScanPermissionResult class.
/// [`BluetoothLEScanPermissionResult`](https://developer.mozilla.org/en-US/docs/Web/API/BluetoothLEScanPermissionResult)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct BluetoothLEScanPermissionResult {
    inner: PermissionStatus,
}

impl FromVal for BluetoothLEScanPermissionResult {
    fn from_val(v: &Any) -> Self {
        BluetoothLEScanPermissionResult {
            inner: PermissionStatus::from_val(v),
        }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for BluetoothLEScanPermissionResult {
    type Target = PermissionStatus;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for BluetoothLEScanPermissionResult {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for BluetoothLEScanPermissionResult {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for BluetoothLEScanPermissionResult {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}

impl From<BluetoothLEScanPermissionResult> for Any {
    fn from(s: BluetoothLEScanPermissionResult) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&BluetoothLEScanPermissionResult> for Any {
    fn from(s: &BluetoothLEScanPermissionResult) -> Any {
        s.inner.clone().into()
    }
}

jsbind::utils::impl_dyn_cast!(BluetoothLEScanPermissionResult);

impl BluetoothLEScanPermissionResult {
    /// Getter of the `scans` attribute.
    /// [`BluetoothLEScanPermissionResult.scans`](https://developer.mozilla.org/en-US/docs/Web/API/BluetoothLEScanPermissionResult/scans)
    pub fn scans(&self) -> TypedArray<BluetoothLEScan> {
        self.inner.get("scans").as_::<TypedArray<BluetoothLEScan>>()
    }

    /// Setter of the `scans` attribute.
    /// [`BluetoothLEScanPermissionResult.scans`](https://developer.mozilla.org/en-US/docs/Web/API/BluetoothLEScanPermissionResult/scans)
    pub fn set_scans(&mut self, value: &TypedArray<BluetoothLEScan>) {
        self.inner.set("scans", value);
    }
}
