use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct NavigationEvent {
    inner: UIEvent,
}
impl FromVal for NavigationEvent {
    fn from_val(v: &emlite::Val) -> Self {
        NavigationEvent {
            inner: UIEvent::from_val(v),
        }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for NavigationEvent {
    type Target = UIEvent;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for NavigationEvent {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for NavigationEvent {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for NavigationEvent {
    fn as_mut(&mut self) -> &mut emlite::Val {
        &mut self.inner
    }
}
impl From<NavigationEvent> for emlite::Val {
    fn from(s: NavigationEvent) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}
jsbind::utils::impl_dyn_cast!(NavigationEvent);

impl NavigationEvent {
    pub fn new0(type_: DOMString) -> NavigationEvent {
        Self {
            inner: emlite::Val::global("NavigationEvent")
                .new(&[type_.into()])
                .as_::<UIEvent>(),
        }
    }

    pub fn new1(type_: DOMString, event_init_dict: Any) -> NavigationEvent {
        Self {
            inner: emlite::Val::global("NavigationEvent")
                .new(&[type_.into(), event_init_dict.into()])
                .as_::<UIEvent>(),
        }
    }
}
impl NavigationEvent {
    pub fn dir(&self) -> SpatialNavigationDirection {
        self.inner.get("dir").as_::<SpatialNavigationDirection>()
    }
}
impl NavigationEvent {
    pub fn related_target(&self) -> EventTarget {
        self.inner.get("relatedTarget").as_::<EventTarget>()
    }
}
