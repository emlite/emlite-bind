use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct PageRevealEvent {
    inner: Event,
}
impl FromVal for PageRevealEvent {
    fn from_val(v: &emlite::Val) -> Self {
        PageRevealEvent {
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
impl core::ops::Deref for PageRevealEvent {
    type Target = Event;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for PageRevealEvent {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for PageRevealEvent {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for PageRevealEvent {
    fn as_mut(&mut self) -> &mut emlite::Val {
        &mut self.inner
    }
}
impl From<PageRevealEvent> for emlite::Val {
    fn from(s: PageRevealEvent) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}
impl From<&PageRevealEvent> for emlite::Val {
    fn from(s: &PageRevealEvent) -> emlite::Val {
        s.inner.clone().into()
    }
}
jsbind::utils::impl_dyn_cast!(PageRevealEvent);

impl PageRevealEvent {
    pub fn new0(type_: DOMString) -> PageRevealEvent {
        Self {
            inner: emlite::Val::global("PageRevealEvent")
                .new(&[type_.into()])
                .as_::<Event>(),
        }
    }

    pub fn new1(type_: DOMString, event_init_dict: Any) -> PageRevealEvent {
        Self {
            inner: emlite::Val::global("PageRevealEvent")
                .new(&[type_.into(), event_init_dict.into()])
                .as_::<Event>(),
        }
    }
}
impl PageRevealEvent {
    pub fn view_transition(&self) -> ViewTransition {
        self.inner.get("viewTransition").as_::<ViewTransition>()
    }
}
