use super::*;

/// The CloseEvent class.
/// [`CloseEvent`](https://developer.mozilla.org/en-US/docs/Web/API/CloseEvent)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct CloseEvent {
    inner: Event,
}
impl FromVal for CloseEvent {
    fn from_val(v: &Any) -> Self {
        CloseEvent {
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
impl core::ops::Deref for CloseEvent {
    type Target = Event;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for CloseEvent {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for CloseEvent {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for CloseEvent {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<CloseEvent> for Any {
    fn from(s: CloseEvent) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&CloseEvent> for Any {
    fn from(s: &CloseEvent) -> Any {
        s.inner.clone().into()
    }
}
jsbind::utils::impl_dyn_cast!(CloseEvent);

impl CloseEvent {
    /// The `new CloseEvent(..)` constructor, creating a new CloseEvent instance
    pub fn new0(type_: &str) -> CloseEvent {
        Self {
            inner: Any::global("CloseEvent")
                .new(&[type_.into()])
                .as_::<Event>(),
        }
    }

    /// The `new CloseEvent(..)` constructor, creating a new CloseEvent instance
    pub fn new1(type_: &str, event_init_dict: &Any) -> CloseEvent {
        Self {
            inner: Any::global("CloseEvent")
                .new(&[type_.into(), event_init_dict.into()])
                .as_::<Event>(),
        }
    }
}
impl CloseEvent {
    /// Getter of the `wasClean` attribute.
    /// [`CloseEvent.wasClean`](https://developer.mozilla.org/en-US/docs/Web/API/CloseEvent/wasClean)
    pub fn was_clean(&self) -> bool {
        self.inner.get("wasClean").as_::<bool>()
    }
}
impl CloseEvent {
    /// Getter of the `code` attribute.
    /// [`CloseEvent.code`](https://developer.mozilla.org/en-US/docs/Web/API/CloseEvent/code)
    pub fn code(&self) -> u16 {
        self.inner.get("code").as_::<u16>()
    }
}
impl CloseEvent {
    /// Getter of the `reason` attribute.
    /// [`CloseEvent.reason`](https://developer.mozilla.org/en-US/docs/Web/API/CloseEvent/reason)
    pub fn reason(&self) -> String {
        self.inner.get("reason").as_::<String>()
    }
}
