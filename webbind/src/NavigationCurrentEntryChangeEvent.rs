use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct NavigationCurrentEntryChangeEvent {
    inner: Event,
}
impl FromVal for NavigationCurrentEntryChangeEvent {
    fn from_val(v: &emlite::Val) -> Self {
        NavigationCurrentEntryChangeEvent {
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
impl core::ops::Deref for NavigationCurrentEntryChangeEvent {
    type Target = Event;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for NavigationCurrentEntryChangeEvent {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for NavigationCurrentEntryChangeEvent {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for NavigationCurrentEntryChangeEvent {
    fn as_mut(&mut self) -> &mut emlite::Val {
        &mut self.inner
    }
}
impl From<NavigationCurrentEntryChangeEvent> for emlite::Val {
    fn from(s: NavigationCurrentEntryChangeEvent) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}
impl From<&NavigationCurrentEntryChangeEvent> for emlite::Val {
    fn from(s: &NavigationCurrentEntryChangeEvent) -> emlite::Val {
        s.inner.clone().into()
    }
}
jsbind::utils::impl_dyn_cast!(NavigationCurrentEntryChangeEvent);

impl NavigationCurrentEntryChangeEvent {
    pub fn new(type_: DOMString, event_init_dict: Any) -> NavigationCurrentEntryChangeEvent {
        Self {
            inner: emlite::Val::global("NavigationCurrentEntryChangeEvent")
                .new(&[type_.into(), event_init_dict.into()])
                .as_::<Event>(),
        }
    }
}
impl NavigationCurrentEntryChangeEvent {
    pub fn navigation_type(&self) -> NavigationType {
        self.inner.get("navigationType").as_::<NavigationType>()
    }
}
impl NavigationCurrentEntryChangeEvent {
    pub fn from(&self) -> NavigationHistoryEntry {
        self.inner.get("from").as_::<NavigationHistoryEntry>()
    }
}
