use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct ExtendableCookieChangeEvent {
    inner: ExtendableEvent,
}
impl FromVal for ExtendableCookieChangeEvent {
    fn from_val(v: &emlite::Val) -> Self {
        ExtendableCookieChangeEvent {
            inner: ExtendableEvent::from_val(v),
        }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for ExtendableCookieChangeEvent {
    type Target = ExtendableEvent;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for ExtendableCookieChangeEvent {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for ExtendableCookieChangeEvent {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for ExtendableCookieChangeEvent {
    fn as_mut(&mut self) -> &mut emlite::Val {
        &mut self.inner
    }
}
impl From<ExtendableCookieChangeEvent> for emlite::Val {
    fn from(s: ExtendableCookieChangeEvent) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}
impl From<&ExtendableCookieChangeEvent> for emlite::Val {
    fn from(s: &ExtendableCookieChangeEvent) -> emlite::Val {
        s.inner.clone().into()
    }
}
jsbind::utils::impl_dyn_cast!(ExtendableCookieChangeEvent);

impl ExtendableCookieChangeEvent {
    pub fn new0(type_: &str) -> ExtendableCookieChangeEvent {
        Self {
            inner: emlite::Val::global("ExtendableCookieChangeEvent")
                .new(&[type_.into()])
                .as_::<ExtendableEvent>(),
        }
    }

    pub fn new1(type_: &str, event_init_dict: &Any) -> ExtendableCookieChangeEvent {
        Self {
            inner: emlite::Val::global("ExtendableCookieChangeEvent")
                .new(&[type_.into(), event_init_dict.into()])
                .as_::<ExtendableEvent>(),
        }
    }
}
impl ExtendableCookieChangeEvent {
    pub fn changed(&self) -> FrozenArray<CookieListItem> {
        self.inner
            .get("changed")
            .as_::<FrozenArray<CookieListItem>>()
    }
}
impl ExtendableCookieChangeEvent {
    pub fn deleted(&self) -> FrozenArray<CookieListItem> {
        self.inner
            .get("deleted")
            .as_::<FrozenArray<CookieListItem>>()
    }
}
