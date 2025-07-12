use super::*;

#[derive(Clone, Debug)]
pub struct DeviceMotionEvent {
    inner: Event,
}
impl FromVal for DeviceMotionEvent {
    fn from_val(v: &emlite::Val) -> Self {
        DeviceMotionEvent {
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
impl std::ops::Deref for DeviceMotionEvent {
    type Target = Event;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for DeviceMotionEvent {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<DeviceMotionEvent> for emlite::Val {
    fn from(s: DeviceMotionEvent) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl DeviceMotionEvent {
    pub fn new0(type_: jsbind::DOMString) -> DeviceMotionEvent {
        Self {
            inner: emlite::Val::global("DeviceMotionEvent")
                .new(&[type_.into()])
                .as_::<Event>(),
        }
    }

    pub fn new1(type_: jsbind::DOMString, event_init_dict: jsbind::Any) -> DeviceMotionEvent {
        Self {
            inner: emlite::Val::global("DeviceMotionEvent")
                .new(&[type_.into(), event_init_dict.into()])
                .as_::<Event>(),
        }
    }
}
impl DeviceMotionEvent {
    pub fn acceleration(&self) -> DeviceMotionEventAcceleration {
        self.inner
            .get("acceleration")
            .as_::<DeviceMotionEventAcceleration>()
    }
}
impl DeviceMotionEvent {
    pub fn acceleration_including_gravity(&self) -> DeviceMotionEventAcceleration {
        self.inner
            .get("accelerationIncludingGravity")
            .as_::<DeviceMotionEventAcceleration>()
    }
}
impl DeviceMotionEvent {
    pub fn rotation_rate(&self) -> DeviceMotionEventRotationRate {
        self.inner
            .get("rotationRate")
            .as_::<DeviceMotionEventRotationRate>()
    }
}
impl DeviceMotionEvent {
    pub fn interval(&self) -> f64 {
        self.inner.get("interval").as_::<f64>()
    }
}
impl DeviceMotionEvent {
    pub fn request_permission() -> jsbind::Promise {
        emlite::Val::global("devicemotionevent")
            .call("requestPermission", &[])
            .as_::<jsbind::Promise>()
    }
}
