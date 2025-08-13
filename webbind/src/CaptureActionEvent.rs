use super::*;




/// The CaptureActionEvent class.
/// [`CaptureActionEvent`](https://developer.mozilla.org/en-US/docs/Web/API/CaptureActionEvent)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct CaptureActionEvent {
    inner: Event,
}

impl FromVal for CaptureActionEvent {
    fn from_val(v: &Any) -> Self {
        CaptureActionEvent { inner: Event::from_val(v) }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for CaptureActionEvent {
    type Target = Event;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for CaptureActionEvent {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for CaptureActionEvent {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for CaptureActionEvent {
    fn as_mut(&mut self) -> &mut Any {
      &mut self.inner
  }
}

impl From<CaptureActionEvent> for Any {
    fn from(s: CaptureActionEvent) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&CaptureActionEvent> for Any {
    fn from(s: &CaptureActionEvent) -> Any {
        s.inner.clone().into()
    }
}

jsbind::utils::impl_dyn_cast!(CaptureActionEvent);



impl CaptureActionEvent {
    /// The `new CaptureActionEvent(..)` constructor, creating a new CaptureActionEvent instance
    pub fn new0() -> CaptureActionEvent {
        Self {
            inner: Any::global("CaptureActionEvent").new(&[]).as_::<Event>(),
        }
    }

    /// The `new CaptureActionEvent(..)` constructor, creating a new CaptureActionEvent instance
    pub fn new1(init: &CaptureActionEventInit) -> CaptureActionEvent {
        Self {
            inner: Any::global("CaptureActionEvent").new(&[init.into()]).as_::<Event>(),
        }
    }

}
impl CaptureActionEvent {
    /// Getter of the `action` attribute.
    /// [`CaptureActionEvent.action`](https://developer.mozilla.org/en-US/docs/Web/API/CaptureActionEvent/action)
    pub fn action(&self) -> CaptureAction {
        self.inner.get("action").as_::<CaptureAction>()
    }

}
