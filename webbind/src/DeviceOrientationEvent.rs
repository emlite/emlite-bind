use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
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
impl From<DeviceOrientationEvent> for emlite::Val {
    fn from(s: DeviceOrientationEvent) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl DeviceOrientationEvent {
    pub fn new0(type_: jsbind::DOMString) -> DeviceOrientationEvent {
        Self {
            inner: emlite::Val::global("DeviceOrientationEvent")
                .new(&[type_.into()])
                .as_::<Event>(),
        }
    }

    pub fn new1(type_: jsbind::DOMString, event_init_dict: jsbind::Any) -> DeviceOrientationEvent {
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
    pub fn request_permission0() -> jsbind::Promise {
        emlite::Val::global("deviceorientationevent")
            .call("requestPermission", &[])
            .as_::<jsbind::Promise>()
    }

    pub fn request_permission1(absolute: bool) -> jsbind::Promise {
        emlite::Val::global("deviceorientationevent")
            .call("requestPermission", &[absolute.into()])
            .as_::<jsbind::Promise>()
    }
}
