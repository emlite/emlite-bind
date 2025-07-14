use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct CapturedMouseEvent {
    inner: Event,
}
impl FromVal for CapturedMouseEvent {
    fn from_val(v: &emlite::Val) -> Self {
        CapturedMouseEvent {
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
impl core::ops::Deref for CapturedMouseEvent {
    type Target = Event;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for CapturedMouseEvent {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for CapturedMouseEvent {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for CapturedMouseEvent {
    fn as_mut(&mut self) -> &mut emlite::Val {
        &mut self.inner
    }
}
impl From<CapturedMouseEvent> for emlite::Val {
    fn from(s: CapturedMouseEvent) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}
jsbind::utils::impl_dyn_cast!(CapturedMouseEvent);

impl CapturedMouseEvent {
    pub fn new0(type_: jsbind::DOMString) -> CapturedMouseEvent {
        Self {
            inner: emlite::Val::global("CapturedMouseEvent")
                .new(&[type_.into()])
                .as_::<Event>(),
        }
    }

    pub fn new1(type_: jsbind::DOMString, event_init_dict: jsbind::Any) -> CapturedMouseEvent {
        Self {
            inner: emlite::Val::global("CapturedMouseEvent")
                .new(&[type_.into(), event_init_dict.into()])
                .as_::<Event>(),
        }
    }
}
impl CapturedMouseEvent {
    pub fn surface_x(&self) -> i32 {
        self.inner.get("surfaceX").as_::<i32>()
    }
}
impl CapturedMouseEvent {
    pub fn surface_y(&self) -> i32 {
        self.inner.get("surfaceY").as_::<i32>()
    }
}
