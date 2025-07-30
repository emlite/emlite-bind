use super::*;

/// The PopStateEvent class.
/// [`PopStateEvent`](https://developer.mozilla.org/en-US/docs/Web/API/PopStateEvent)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct PopStateEvent {
    inner: Event,
}
impl FromVal for PopStateEvent {
    fn from_val(v: &Any) -> Self {
        PopStateEvent {
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
impl core::ops::Deref for PopStateEvent {
    type Target = Event;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for PopStateEvent {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for PopStateEvent {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for PopStateEvent {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<PopStateEvent> for Any {
    fn from(s: PopStateEvent) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&PopStateEvent> for Any {
    fn from(s: &PopStateEvent) -> Any {
        s.inner.clone().into()
    }
}
jsbind::utils::impl_dyn_cast!(PopStateEvent);

impl PopStateEvent {
    /// The `new PopStateEvent(..)` constructor, creating a new PopStateEvent instance
    pub fn new0(type_: &JsString) -> PopStateEvent {
        Self {
            inner: Any::global("PopStateEvent")
                .new(&[type_.into()])
                .as_::<Event>(),
        }
    }

    /// The `new PopStateEvent(..)` constructor, creating a new PopStateEvent instance
    pub fn new1(type_: &JsString, event_init_dict: &Any) -> PopStateEvent {
        Self {
            inner: Any::global("PopStateEvent")
                .new(&[type_.into(), event_init_dict.into()])
                .as_::<Event>(),
        }
    }
}
impl PopStateEvent {
    /// Getter of the `state` attribute.
    /// [`PopStateEvent.state`](https://developer.mozilla.org/en-US/docs/Web/API/PopStateEvent/state)
    pub fn state(&self) -> Any {
        self.inner.get("state").as_::<Any>()
    }
}
impl PopStateEvent {
    /// Getter of the `hasUAVisualTransition` attribute.
    /// [`PopStateEvent.hasUAVisualTransition`](https://developer.mozilla.org/en-US/docs/Web/API/PopStateEvent/hasUAVisualTransition)
    pub fn has_ua_visual_transition(&self) -> bool {
        self.inner.get("hasUAVisualTransition").as_::<bool>()
    }
}
