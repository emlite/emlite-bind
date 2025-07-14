use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
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
impl From<NavigatorManagedData> for emlite::Val {
    fn from(s: NavigatorManagedData) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl NavigatorManagedData {
    pub fn get_managed_configuration(
        &self,
        keys: jsbind::Sequence<jsbind::DOMString>,
    ) -> jsbind::Promise {
        self.inner
            .call("getManagedConfiguration", &[keys.into()])
            .as_::<jsbind::Promise>()
    }
}
impl NavigatorManagedData {
    pub fn onmanagedconfigurationchange(&self) -> jsbind::Any {
        self.inner
            .get("onmanagedconfigurationchange")
            .as_::<jsbind::Any>()
    }

    pub fn set_onmanagedconfigurationchange(&mut self, value: jsbind::Any) {
        self.inner.set("onmanagedconfigurationchange", value);
    }
}
impl NavigatorManagedData {
    pub fn get_annotated_asset_id(&self) -> jsbind::Promise {
        self.inner
            .call("getAnnotatedAssetId", &[])
            .as_::<jsbind::Promise>()
    }
}
impl NavigatorManagedData {
    pub fn get_annotated_location(&self) -> jsbind::Promise {
        self.inner
            .call("getAnnotatedLocation", &[])
            .as_::<jsbind::Promise>()
    }
}
impl NavigatorManagedData {
    pub fn get_directory_id(&self) -> jsbind::Promise {
        self.inner
            .call("getDirectoryId", &[])
            .as_::<jsbind::Promise>()
    }
}
impl NavigatorManagedData {
    pub fn get_hostname(&self) -> jsbind::Promise {
        self.inner.call("getHostname", &[]).as_::<jsbind::Promise>()
    }
}
impl NavigatorManagedData {
    pub fn get_serial_number(&self) -> jsbind::Promise {
        self.inner
            .call("getSerialNumber", &[])
            .as_::<jsbind::Promise>()
    }
}
