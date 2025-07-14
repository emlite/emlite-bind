use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct IDBVersionChangeEvent {
    inner: Event,
}
impl FromVal for IDBVersionChangeEvent {
    fn from_val(v: &emlite::Val) -> Self {
        IDBVersionChangeEvent {
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
impl core::ops::Deref for IDBVersionChangeEvent {
    type Target = Event;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for IDBVersionChangeEvent {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<IDBVersionChangeEvent> for emlite::Val {
    fn from(s: IDBVersionChangeEvent) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl IDBVersionChangeEvent {
    pub fn new0(type_: jsbind::DOMString) -> IDBVersionChangeEvent {
        Self {
            inner: emlite::Val::global("IDBVersionChangeEvent")
                .new(&[type_.into()])
                .as_::<Event>(),
        }
    }

    pub fn new1(type_: jsbind::DOMString, event_init_dict: jsbind::Any) -> IDBVersionChangeEvent {
        Self {
            inner: emlite::Val::global("IDBVersionChangeEvent")
                .new(&[type_.into(), event_init_dict.into()])
                .as_::<Event>(),
        }
    }
}
impl IDBVersionChangeEvent {
    pub fn old_version(&self) -> u64 {
        self.inner.get("oldVersion").as_::<u64>()
    }
}
impl IDBVersionChangeEvent {
    pub fn new_version(&self) -> u64 {
        self.inner.get("newVersion").as_::<u64>()
    }
}
