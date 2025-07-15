use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct CookieChangeEvent {
    inner: Event,
}
impl FromVal for CookieChangeEvent {
    fn from_val(v: &emlite::Val) -> Self {
        CookieChangeEvent {
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
impl core::ops::Deref for CookieChangeEvent {
    type Target = Event;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for CookieChangeEvent {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for CookieChangeEvent {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for CookieChangeEvent {
    fn as_mut(&mut self) -> &mut emlite::Val {
        &mut self.inner
    }
}
impl From<CookieChangeEvent> for emlite::Val {
    fn from(s: CookieChangeEvent) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}
jsbind::utils::impl_dyn_cast!(CookieChangeEvent);

impl CookieChangeEvent {
    pub fn new0(type_: DOMString) -> CookieChangeEvent {
        Self {
            inner: emlite::Val::global("CookieChangeEvent")
                .new(&[type_.into()])
                .as_::<Event>(),
        }
    }

    pub fn new1(type_: DOMString, event_init_dict: Any) -> CookieChangeEvent {
        Self {
            inner: emlite::Val::global("CookieChangeEvent")
                .new(&[type_.into(), event_init_dict.into()])
                .as_::<Event>(),
        }
    }
}
impl CookieChangeEvent {
    pub fn changed(&self) -> FrozenArray<CookieListItem> {
        self.inner
            .get("changed")
            .as_::<FrozenArray<CookieListItem>>()
    }
}
impl CookieChangeEvent {
    pub fn deleted(&self) -> FrozenArray<CookieListItem> {
        self.inner
            .get("deleted")
            .as_::<FrozenArray<CookieListItem>>()
    }
}
