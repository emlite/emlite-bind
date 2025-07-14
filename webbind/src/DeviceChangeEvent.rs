use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
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
impl From<DeviceChangeEvent> for emlite::Val {
    fn from(s: DeviceChangeEvent) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl DeviceChangeEvent {
    pub fn new0(type_: jsbind::DOMString) -> DeviceChangeEvent {
        Self {
            inner: emlite::Val::global("DeviceChangeEvent")
                .new(&[type_.into()])
                .as_::<Event>(),
        }
    }

    pub fn new1(type_: jsbind::DOMString, event_init_dict: jsbind::Any) -> DeviceChangeEvent {
        Self {
            inner: emlite::Val::global("DeviceChangeEvent")
                .new(&[type_.into(), event_init_dict.into()])
                .as_::<Event>(),
        }
    }
}
impl DeviceChangeEvent {
    pub fn devices(&self) -> jsbind::FrozenArray<MediaDeviceInfo> {
        self.inner
            .get("devices")
            .as_::<jsbind::FrozenArray<MediaDeviceInfo>>()
    }
}
impl DeviceChangeEvent {
    pub fn user_inserted_devices(&self) -> jsbind::FrozenArray<MediaDeviceInfo> {
        self.inner
            .get("userInsertedDevices")
            .as_::<jsbind::FrozenArray<MediaDeviceInfo>>()
    }
}
