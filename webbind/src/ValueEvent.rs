use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct ValueEvent {
    inner: Event,
}
impl FromVal for ValueEvent {
    fn from_val(v: &emlite::Val) -> Self {
        ValueEvent {
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
impl core::ops::Deref for ValueEvent {
    type Target = Event;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for ValueEvent {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<ValueEvent> for emlite::Val {
    fn from(s: ValueEvent) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl ValueEvent {
    pub fn new0(type_: jsbind::DOMString) -> ValueEvent {
        Self {
            inner: emlite::Val::global("ValueEvent")
                .new(&[type_.into()])
                .as_::<Event>(),
        }
    }

    pub fn new1(type_: jsbind::DOMString, init_dict: jsbind::Any) -> ValueEvent {
        Self {
            inner: emlite::Val::global("ValueEvent")
                .new(&[type_.into(), init_dict.into()])
                .as_::<Event>(),
        }
    }
}
impl ValueEvent {
    pub fn value(&self) -> jsbind::Any {
        self.inner.get("value").as_::<jsbind::Any>()
    }
}
