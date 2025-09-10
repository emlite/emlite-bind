use super::*;

/// The XRSessionEvent class.
/// [`XRSessionEvent`](https://developer.mozilla.org/en-US/docs/Web/API/XRSessionEvent)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct XRSessionEvent {
    inner: Event,
}

impl FromVal for XRSessionEvent {
    fn from_val(v: &Any) -> Self {
        XRSessionEvent {
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

impl core::ops::Deref for XRSessionEvent {
    type Target = Event;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for XRSessionEvent {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for XRSessionEvent {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for XRSessionEvent {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}

impl From<XRSessionEvent> for Any {
    fn from(s: XRSessionEvent) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&XRSessionEvent> for Any {
    fn from(s: &XRSessionEvent) -> Any {
        s.inner.clone().into()
    }
}

jsbind::utils::impl_dyn_cast!(XRSessionEvent);

impl XRSessionEvent {
    /// The `new XRSessionEvent(..)` constructor, creating a new XRSessionEvent instance
    pub fn new(type_: &JsString, event_init_dict: &XRSessionEventInit) -> XRSessionEvent {
        Self {
            inner: Any::global("XRSessionEvent")
                .new(&[type_.into(), event_init_dict.into()])
                .as_::<Event>(),
        }
    }
}
impl XRSessionEvent {
    /// Getter of the `session` attribute.
    /// [`XRSessionEvent.session`](https://developer.mozilla.org/en-US/docs/Web/API/XRSessionEvent/session)
    pub fn session(&self) -> XRSession {
        self.inner.get("session").as_::<XRSession>()
    }
}
