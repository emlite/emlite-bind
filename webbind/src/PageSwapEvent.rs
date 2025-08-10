use super::*;

/// The PageSwapEvent class.
/// [`PageSwapEvent`](https://developer.mozilla.org/en-US/docs/Web/API/PageSwapEvent)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct PageSwapEvent {
    inner: Event,
}
impl FromVal for PageSwapEvent {
    fn from_val(v: &Any) -> Self {
        PageSwapEvent {
            inner: Event::from_val(v),
        }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
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
impl AsRef<Any> for PageSwapEvent {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for PageSwapEvent {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<PageSwapEvent> for Any {
    fn from(s: PageSwapEvent) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&PageSwapEvent> for Any {
    fn from(s: &PageSwapEvent) -> Any {
        s.inner.clone().into()
    }
}
jsbind::utils::impl_dyn_cast!(PageSwapEvent);

impl PageSwapEvent {
    /// The `new PageSwapEvent(..)` constructor, creating a new PageSwapEvent instance
    pub fn new0(type_: &JsString) -> PageSwapEvent {
        Self {
            inner: Any::global("PageSwapEvent")
                .new(&[type_.into()])
                .as_::<Event>(),
        }
    }

    /// The `new PageSwapEvent(..)` constructor, creating a new PageSwapEvent instance
    pub fn new1(type_: &JsString, event_init_dict: &PageSwapEventInit) -> PageSwapEvent {
        Self {
            inner: Any::global("PageSwapEvent")
                .new(&[type_.into(), event_init_dict.into()])
                .as_::<Event>(),
        }
    }
}
impl PageSwapEvent {
    /// Getter of the `activation` attribute.
    /// [`PageSwapEvent.activation`](https://developer.mozilla.org/en-US/docs/Web/API/PageSwapEvent/activation)
    pub fn activation(&self) -> NavigationActivation {
        self.inner.get("activation").as_::<NavigationActivation>()
    }
}
impl PageSwapEvent {
    /// Getter of the `viewTransition` attribute.
    /// [`PageSwapEvent.viewTransition`](https://developer.mozilla.org/en-US/docs/Web/API/PageSwapEvent/viewTransition)
    pub fn view_transition(&self) -> ViewTransition {
        self.inner.get("viewTransition").as_::<ViewTransition>()
    }
}
