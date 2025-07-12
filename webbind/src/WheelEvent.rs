use super::*;

#[derive(Clone, Debug)]
pub struct WheelEvent {
    inner: MouseEvent,
}
impl FromVal for WheelEvent {
    fn from_val(v: &emlite::Val) -> Self {
        WheelEvent {
            inner: MouseEvent::from_val(v),
        }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl std::ops::Deref for WheelEvent {
    type Target = MouseEvent;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for WheelEvent {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<WheelEvent> for emlite::Val {
    fn from(s: WheelEvent) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl WheelEvent {
    pub fn new0(type_: jsbind::DOMString) -> WheelEvent {
        Self {
            inner: emlite::Val::global("WheelEvent")
                .new(&[type_.into()])
                .as_::<MouseEvent>(),
        }
    }

    pub fn new1(type_: jsbind::DOMString, event_init_dict: jsbind::Any) -> WheelEvent {
        Self {
            inner: emlite::Val::global("WheelEvent")
                .new(&[type_.into(), event_init_dict.into()])
                .as_::<MouseEvent>(),
        }
    }
}
impl WheelEvent {
    pub fn delta_x(&self) -> f64 {
        self.inner.get("deltaX").as_::<f64>()
    }
}
impl WheelEvent {
    pub fn delta_y(&self) -> f64 {
        self.inner.get("deltaY").as_::<f64>()
    }
}
impl WheelEvent {
    pub fn delta_z(&self) -> f64 {
        self.inner.get("deltaZ").as_::<f64>()
    }
}
impl WheelEvent {
    pub fn delta_mode(&self) -> u32 {
        self.inner.get("deltaMode").as_::<u32>()
    }
}
