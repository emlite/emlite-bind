use super::*;

/// The CapturedMouseEvent class.
/// [`CapturedMouseEvent`](https://developer.mozilla.org/en-US/docs/Web/API/CapturedMouseEvent)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct CapturedMouseEvent {
    inner: Event,
}

impl FromVal for CapturedMouseEvent {
    fn from_val(v: &Any) -> Self {
        CapturedMouseEvent {
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

impl core::ops::Deref for CapturedMouseEvent {
    type Target = Event;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for CapturedMouseEvent {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for CapturedMouseEvent {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for CapturedMouseEvent {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}

impl From<CapturedMouseEvent> for Any {
    fn from(s: CapturedMouseEvent) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&CapturedMouseEvent> for Any {
    fn from(s: &CapturedMouseEvent) -> Any {
        s.inner.clone().into()
    }
}

jsbind::utils::impl_dyn_cast!(CapturedMouseEvent);

impl CapturedMouseEvent {
    /// Getter of the `surfaceX` attribute.
    /// [`CapturedMouseEvent.surfaceX`](https://developer.mozilla.org/en-US/docs/Web/API/CapturedMouseEvent/surfaceX)
    pub fn surface_x(&self) -> i32 {
        self.inner.get("surfaceX").as_::<i32>()
    }
}
impl CapturedMouseEvent {
    /// Getter of the `surfaceY` attribute.
    /// [`CapturedMouseEvent.surfaceY`](https://developer.mozilla.org/en-US/docs/Web/API/CapturedMouseEvent/surfaceY)
    pub fn surface_y(&self) -> i32 {
        self.inner.get("surfaceY").as_::<i32>()
    }
}

impl CapturedMouseEvent {
    /// The `new CapturedMouseEvent(..)` constructor, creating a new CapturedMouseEvent instance
    pub fn new0(type_: &JsString) -> CapturedMouseEvent {
        Self {
            inner: Any::global("CapturedMouseEvent")
                .new(&[type_.into()])
                .as_::<Event>(),
        }
    }

    /// The `new CapturedMouseEvent(..)` constructor, creating a new CapturedMouseEvent instance
    pub fn new1(type_: &JsString, event_init_dict: &CapturedMouseEventInit) -> CapturedMouseEvent {
        Self {
            inner: Any::global("CapturedMouseEvent")
                .new(&[type_.into(), event_init_dict.into()])
                .as_::<Event>(),
        }
    }
}
