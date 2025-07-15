use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
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
impl core::ops::Deref for DeviceMotionEvent {
    type Target = Event;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for DeviceMotionEvent {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for DeviceMotionEvent {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for DeviceMotionEvent {
    fn as_mut(&mut self) -> &mut emlite::Val {
        &mut self.inner
    }
}
impl From<DeviceMotionEvent> for emlite::Val {
    fn from(s: DeviceMotionEvent) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}
impl From<&DeviceMotionEvent> for emlite::Val {
    fn from(s: &DeviceMotionEvent) -> emlite::Val {
        s.inner.clone().into()
    }
}
jsbind::utils::impl_dyn_cast!(DeviceMotionEvent);

impl DeviceMotionEvent {
    pub fn new0(type_: &str) -> DeviceMotionEvent {
        Self {
            inner: emlite::Val::global("DeviceMotionEvent")
                .new(&[type_.into()])
                .as_::<Event>(),
        }
    }

    pub fn new1(type_: &str, event_init_dict: &Any) -> DeviceMotionEvent {
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
    pub fn request_permission() -> Promise {
        emlite::Val::global("DeviceMotionEvent")
            .call("requestPermission", &[])
            .as_::<Promise>()
    }
}
