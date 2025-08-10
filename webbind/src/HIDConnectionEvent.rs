use super::*;

/// The HIDConnectionEvent class.
/// [`HIDConnectionEvent`](https://developer.mozilla.org/en-US/docs/Web/API/HIDConnectionEvent)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct HIDConnectionEvent {
    inner: Event,
}
impl FromVal for HIDConnectionEvent {
    fn from_val(v: &Any) -> Self {
        HIDConnectionEvent {
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
impl core::ops::Deref for HIDConnectionEvent {
    type Target = Event;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for HIDConnectionEvent {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for HIDConnectionEvent {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for HIDConnectionEvent {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<HIDConnectionEvent> for Any {
    fn from(s: HIDConnectionEvent) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&HIDConnectionEvent> for Any {
    fn from(s: &HIDConnectionEvent) -> Any {
        s.inner.clone().into()
    }
}
jsbind::utils::impl_dyn_cast!(HIDConnectionEvent);

impl HIDConnectionEvent {
    /// The `new HIDConnectionEvent(..)` constructor, creating a new HIDConnectionEvent instance
    pub fn new(type_: &JsString, event_init_dict: &HIDConnectionEventInit) -> HIDConnectionEvent {
        Self {
            inner: Any::global("HIDConnectionEvent")
                .new(&[type_.into(), event_init_dict.into()])
                .as_::<Event>(),
        }
    }
}
impl HIDConnectionEvent {
    /// Getter of the `device` attribute.
    /// [`HIDConnectionEvent.device`](https://developer.mozilla.org/en-US/docs/Web/API/HIDConnectionEvent/device)
    pub fn device(&self) -> HIDDevice {
        self.inner.get("device").as_::<HIDDevice>()
    }
}
