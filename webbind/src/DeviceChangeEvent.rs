use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct DeviceChangeEvent {
    inner: Event,
}
impl FromVal for DeviceChangeEvent {
    fn from_val(v: &emlite::Val) -> Self {
        DeviceChangeEvent {
            inner: Event::from_val(v),
        }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for DeviceChangeEvent {
    type Target = Event;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for DeviceChangeEvent {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for DeviceChangeEvent {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for DeviceChangeEvent {
    fn as_mut(&mut self) -> &mut emlite::Val {
        &mut self.inner
    }
}
impl From<DeviceChangeEvent> for emlite::Val {
    fn from(s: DeviceChangeEvent) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}
jsbind::utils::impl_dyn_cast!(DeviceChangeEvent);

impl DeviceChangeEvent {
    pub fn new0(type_: DOMString) -> DeviceChangeEvent {
        Self {
            inner: emlite::Val::global("DeviceChangeEvent")
                .new(&[type_.into()])
                .as_::<Event>(),
        }
    }

    pub fn new1(type_: DOMString, event_init_dict: Any) -> DeviceChangeEvent {
        Self {
            inner: emlite::Val::global("DeviceChangeEvent")
                .new(&[type_.into(), event_init_dict.into()])
                .as_::<Event>(),
        }
    }
}
impl DeviceChangeEvent {
    pub fn devices(&self) -> FrozenArray<MediaDeviceInfo> {
        self.inner
            .get("devices")
            .as_::<FrozenArray<MediaDeviceInfo>>()
    }
}
impl DeviceChangeEvent {
    pub fn user_inserted_devices(&self) -> FrozenArray<MediaDeviceInfo> {
        self.inner
            .get("userInsertedDevices")
            .as_::<FrozenArray<MediaDeviceInfo>>()
    }
}
