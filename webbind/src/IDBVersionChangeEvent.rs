use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
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
impl AsRef<emlite::Val> for IDBVersionChangeEvent {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for IDBVersionChangeEvent {
    fn as_mut(&mut self) -> &mut emlite::Val {
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
impl From<&IDBVersionChangeEvent> for emlite::Val {
    fn from(s: &IDBVersionChangeEvent) -> emlite::Val {
        s.inner.clone().into()
    }
}
jsbind::utils::impl_dyn_cast!(IDBVersionChangeEvent);

impl IDBVersionChangeEvent {
    pub fn new0(type_: &str) -> IDBVersionChangeEvent {
        Self {
            inner: emlite::Val::global("IDBVersionChangeEvent")
                .new(&[type_.into()])
                .as_::<Event>(),
        }
    }

    pub fn new1(type_: &str, event_init_dict: &Any) -> IDBVersionChangeEvent {
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
