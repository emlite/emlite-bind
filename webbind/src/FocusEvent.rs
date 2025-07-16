use super::*;

/// The FocusEvent class.
/// [`FocusEvent`](https://developer.mozilla.org/en-US/docs/Web/API/FocusEvent)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct FocusEvent {
    inner: UIEvent,
}
impl FromVal for FocusEvent {
    fn from_val(v: &Any) -> Self {
        FocusEvent {
            inner: UIEvent::from_val(v),
        }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for FocusEvent {
    type Target = UIEvent;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for FocusEvent {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for FocusEvent {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for FocusEvent {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<FocusEvent> for Any {
    fn from(s: FocusEvent) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&FocusEvent> for Any {
    fn from(s: &FocusEvent) -> Any {
        s.inner.clone().into()
    }
}
jsbind::utils::impl_dyn_cast!(FocusEvent);

impl FocusEvent {
    /// The `new FocusEvent(..)` constructor, creating a new FocusEvent instance
    pub fn new0(type_: &str) -> FocusEvent {
        Self {
            inner: Any::global("FocusEvent")
                .new(&[type_.into()])
                .as_::<UIEvent>(),
        }
    }

    /// The `new FocusEvent(..)` constructor, creating a new FocusEvent instance
    pub fn new1(type_: &str, event_init_dict: &Any) -> FocusEvent {
        Self {
            inner: Any::global("FocusEvent")
                .new(&[type_.into(), event_init_dict.into()])
                .as_::<UIEvent>(),
        }
    }
}
impl FocusEvent {
    /// Getter of the `relatedTarget` attribute.
    /// [`FocusEvent.relatedTarget`](https://developer.mozilla.org/en-US/docs/Web/API/FocusEvent/relatedTarget)
    pub fn related_target(&self) -> EventTarget {
        self.inner.get("relatedTarget").as_::<EventTarget>()
    }
}
