use super::*;

/// The NavigatorManagedData class.
/// [`NavigatorManagedData`](https://developer.mozilla.org/en-US/docs/Web/API/NavigatorManagedData)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct NavigatorManagedData {
    inner: EventTarget,
}
impl FromVal for NavigatorManagedData {
    fn from_val(v: &Any) -> Self {
        NavigatorManagedData {
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
impl core::ops::Deref for NavigatorManagedData {
    type Target = EventTarget;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for NavigatorManagedData {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for NavigatorManagedData {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for NavigatorManagedData {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<NavigatorManagedData> for Any {
    fn from(s: NavigatorManagedData) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&NavigatorManagedData> for Any {
    fn from(s: &NavigatorManagedData) -> Any {
        s.inner.clone().into()
    }
}
jsbind::utils::impl_dyn_cast!(NavigatorManagedData);

impl NavigatorManagedData {
    /// The getManagedConfiguration method.
    /// [`NavigatorManagedData.getManagedConfiguration`](https://developer.mozilla.org/en-US/docs/Web/API/NavigatorManagedData/getManagedConfiguration)
    pub fn get_managed_configuration(&self, keys: &Sequence<String>) -> Promise {
        self.inner
            .call("getManagedConfiguration", &[keys.into()])
            .as_::<Promise>()
    }
}
impl NavigatorManagedData {
    /// Getter of the `onmanagedconfigurationchange` attribute.
    /// [`NavigatorManagedData.onmanagedconfigurationchange`](https://developer.mozilla.org/en-US/docs/Web/API/NavigatorManagedData/onmanagedconfigurationchange)
    pub fn onmanagedconfigurationchange(&self) -> Any {
        self.inner.get("onmanagedconfigurationchange").as_::<Any>()
    }

    /// Setter of the `onmanagedconfigurationchange` attribute.
    /// [`NavigatorManagedData.onmanagedconfigurationchange`](https://developer.mozilla.org/en-US/docs/Web/API/NavigatorManagedData/onmanagedconfigurationchange)
    pub fn set_onmanagedconfigurationchange(&mut self, value: &Any) {
        self.inner.set("onmanagedconfigurationchange", value);
    }
}
impl NavigatorManagedData {
    /// The getAnnotatedAssetId method.
    /// [`NavigatorManagedData.getAnnotatedAssetId`](https://developer.mozilla.org/en-US/docs/Web/API/NavigatorManagedData/getAnnotatedAssetId)
    pub fn get_annotated_asset_id(&self) -> Promise {
        self.inner.call("getAnnotatedAssetId", &[]).as_::<Promise>()
    }
}
impl NavigatorManagedData {
    /// The getAnnotatedLocation method.
    /// [`NavigatorManagedData.getAnnotatedLocation`](https://developer.mozilla.org/en-US/docs/Web/API/NavigatorManagedData/getAnnotatedLocation)
    pub fn get_annotated_location(&self) -> Promise {
        self.inner
            .call("getAnnotatedLocation", &[])
            .as_::<Promise>()
    }
}
impl NavigatorManagedData {
    /// The getDirectoryId method.
    /// [`NavigatorManagedData.getDirectoryId`](https://developer.mozilla.org/en-US/docs/Web/API/NavigatorManagedData/getDirectoryId)
    pub fn get_directory_id(&self) -> Promise {
        self.inner.call("getDirectoryId", &[]).as_::<Promise>()
    }
}
impl NavigatorManagedData {
    /// The getHostname method.
    /// [`NavigatorManagedData.getHostname`](https://developer.mozilla.org/en-US/docs/Web/API/NavigatorManagedData/getHostname)
    pub fn get_hostname(&self) -> Promise {
        self.inner.call("getHostname", &[]).as_::<Promise>()
    }
}
impl NavigatorManagedData {
    /// The getSerialNumber method.
    /// [`NavigatorManagedData.getSerialNumber`](https://developer.mozilla.org/en-US/docs/Web/API/NavigatorManagedData/getSerialNumber)
    pub fn get_serial_number(&self) -> Promise {
        self.inner.call("getSerialNumber", &[]).as_::<Promise>()
    }
}
