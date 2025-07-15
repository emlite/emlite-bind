use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct DeviceOrientationEvent {
    inner: Event,
}
impl FromVal for DeviceOrientationEvent {
    fn from_val(v: &emlite::Val) -> Self {
        DeviceOrientationEvent {
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
impl core::ops::Deref for DeviceOrientationEvent {
    type Target = Event;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for DeviceOrientationEvent {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for DeviceOrientationEvent {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for DeviceOrientationEvent {
    fn as_mut(&mut self) -> &mut emlite::Val {
        &mut self.inner
    }
}
impl From<DeviceOrientationEvent> for emlite::Val {
    fn from(s: DeviceOrientationEvent) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}
jsbind::utils::impl_dyn_cast!(DeviceOrientationEvent);

impl DeviceOrientationEvent {
    pub fn new0(type_: DOMString) -> DeviceOrientationEvent {
        Self {
            inner: emlite::Val::global("DeviceOrientationEvent")
                .new(&[type_.into()])
                .as_::<Event>(),
        }
    }

    pub fn new1(type_: DOMString, event_init_dict: Any) -> DeviceOrientationEvent {
        Self {
            inner: emlite::Val::global("DeviceOrientationEvent")
                .new(&[type_.into(), event_init_dict.into()])
                .as_::<Event>(),
        }
    }
}
impl DeviceOrientationEvent {
    pub fn alpha(&self) -> f64 {
        self.inner.get("alpha").as_::<f64>()
    }
}
impl DeviceOrientationEvent {
    pub fn beta(&self) -> f64 {
        self.inner.get("beta").as_::<f64>()
    }
}
impl DeviceOrientationEvent {
    pub fn gamma(&self) -> f64 {
        self.inner.get("gamma").as_::<f64>()
    }
}
impl DeviceOrientationEvent {
    pub fn absolute(&self) -> bool {
        self.inner.get("absolute").as_::<bool>()
    }
}
impl DeviceOrientationEvent {
    pub fn request_permission0() -> Promise {
        emlite::Val::global("DeviceOrientationEvent")
            .call("requestPermission", &[])
            .as_::<Promise>()
    }

    pub fn request_permission1(absolute: bool) -> Promise {
        emlite::Val::global("DeviceOrientationEvent")
            .call("requestPermission", &[absolute.into()])
            .as_::<Promise>()
    }
}
