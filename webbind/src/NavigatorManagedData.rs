use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct NavigatorManagedData {
    inner: EventTarget,
}
impl FromVal for NavigatorManagedData {
    fn from_val(v: &emlite::Val) -> Self {
        NavigatorManagedData {
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
impl AsRef<emlite::Val> for NavigatorManagedData {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for NavigatorManagedData {
    fn as_mut(&mut self) -> &mut emlite::Val {
        &mut self.inner
    }
}
impl From<NavigatorManagedData> for emlite::Val {
    fn from(s: NavigatorManagedData) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}
jsbind::utils::impl_dyn_cast!(NavigatorManagedData);

impl NavigatorManagedData {
    pub fn get_managed_configuration(&self, keys: Sequence<DOMString>) -> Promise {
        self.inner
            .call("getManagedConfiguration", &[keys.into()])
            .as_::<Promise>()
    }
}
impl NavigatorManagedData {
    pub fn onmanagedconfigurationchange(&self) -> Any {
        self.inner.get("onmanagedconfigurationchange").as_::<Any>()
    }

    pub fn set_onmanagedconfigurationchange(&mut self, value: Any) {
        self.inner.set("onmanagedconfigurationchange", value);
    }
}
impl NavigatorManagedData {
    pub fn get_annotated_asset_id(&self) -> Promise {
        self.inner.call("getAnnotatedAssetId", &[]).as_::<Promise>()
    }
}
impl NavigatorManagedData {
    pub fn get_annotated_location(&self) -> Promise {
        self.inner
            .call("getAnnotatedLocation", &[])
            .as_::<Promise>()
    }
}
impl NavigatorManagedData {
    pub fn get_directory_id(&self) -> Promise {
        self.inner.call("getDirectoryId", &[]).as_::<Promise>()
    }
}
impl NavigatorManagedData {
    pub fn get_hostname(&self) -> Promise {
        self.inner.call("getHostname", &[]).as_::<Promise>()
    }
}
impl NavigatorManagedData {
    pub fn get_serial_number(&self) -> Promise {
        self.inner.call("getSerialNumber", &[]).as_::<Promise>()
    }
}
