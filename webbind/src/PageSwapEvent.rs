use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct PageSwapEvent {
    inner: Event,
}
impl FromVal for PageSwapEvent {
    fn from_val(v: &emlite::Val) -> Self {
        PageSwapEvent {
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
impl core::ops::Deref for PageSwapEvent {
    type Target = Event;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for PageSwapEvent {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for PageSwapEvent {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for PageSwapEvent {
    fn as_mut(&mut self) -> &mut emlite::Val {
        &mut self.inner
    }
}
impl From<PageSwapEvent> for emlite::Val {
    fn from(s: PageSwapEvent) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}
impl From<&PageSwapEvent> for emlite::Val {
    fn from(s: &PageSwapEvent) -> emlite::Val {
        s.inner.clone().into()
    }
}
jsbind::utils::impl_dyn_cast!(PageSwapEvent);

impl PageSwapEvent {
    pub fn new0(type_: DOMString) -> PageSwapEvent {
        Self {
            inner: emlite::Val::global("PageSwapEvent")
                .new(&[type_.into()])
                .as_::<Event>(),
        }
    }

    pub fn new1(type_: DOMString, event_init_dict: Any) -> PageSwapEvent {
        Self {
            inner: emlite::Val::global("PageSwapEvent")
                .new(&[type_.into(), event_init_dict.into()])
                .as_::<Event>(),
        }
    }
}
impl PageSwapEvent {
    pub fn activation(&self) -> NavigationActivation {
        self.inner.get("activation").as_::<NavigationActivation>()
    }
}
impl PageSwapEvent {
    pub fn view_transition(&self) -> ViewTransition {
        self.inner.get("viewTransition").as_::<ViewTransition>()
    }
}
