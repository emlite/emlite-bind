use super::*;




/// The WheelEvent class.
/// [`WheelEvent`](https://developer.mozilla.org/en-US/docs/Web/API/WheelEvent)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct WheelEvent {
    inner: MouseEvent,
}

impl FromVal for WheelEvent {
    fn from_val(v: &Any) -> Self {
        WheelEvent { inner: MouseEvent::from_val(v) }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for WheelEvent {
    type Target = MouseEvent;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for WheelEvent {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for WheelEvent {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for WheelEvent {
    fn as_mut(&mut self) -> &mut Any {
      &mut self.inner
  }
}

impl From<WheelEvent> for Any {
    fn from(s: WheelEvent) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&WheelEvent> for Any {
    fn from(s: &WheelEvent) -> Any {
        s.inner.clone().into()
    }
}

jsbind::utils::impl_dyn_cast!(WheelEvent);



impl WheelEvent {
    /// The `new WheelEvent(..)` constructor, creating a new WheelEvent instance
    pub fn new0(type_: &JsString) -> WheelEvent {
        Self {
            inner: Any::global("WheelEvent").new(&[type_.into()]).as_::<MouseEvent>(),
        }
    }

    /// The `new WheelEvent(..)` constructor, creating a new WheelEvent instance
    pub fn new1(type_: &JsString, event_init_dict: &WheelEventInit) -> WheelEvent {
        Self {
            inner: Any::global("WheelEvent").new(&[type_.into(), event_init_dict.into()]).as_::<MouseEvent>(),
        }
    }

}
impl WheelEvent {
    /// Getter of the `deltaX` attribute.
    /// [`WheelEvent.deltaX`](https://developer.mozilla.org/en-US/docs/Web/API/WheelEvent/deltaX)
    pub fn delta_x(&self) -> f64 {
        self.inner.get("deltaX").as_::<f64>()
    }

}
impl WheelEvent {
    /// Getter of the `deltaY` attribute.
    /// [`WheelEvent.deltaY`](https://developer.mozilla.org/en-US/docs/Web/API/WheelEvent/deltaY)
    pub fn delta_y(&self) -> f64 {
        self.inner.get("deltaY").as_::<f64>()
    }

}
impl WheelEvent {
    /// Getter of the `deltaZ` attribute.
    /// [`WheelEvent.deltaZ`](https://developer.mozilla.org/en-US/docs/Web/API/WheelEvent/deltaZ)
    pub fn delta_z(&self) -> f64 {
        self.inner.get("deltaZ").as_::<f64>()
    }

}
impl WheelEvent {
    /// Getter of the `deltaMode` attribute.
    /// [`WheelEvent.deltaMode`](https://developer.mozilla.org/en-US/docs/Web/API/WheelEvent/deltaMode)
    pub fn delta_mode(&self) -> u32 {
        self.inner.get("deltaMode").as_::<u32>()
    }

}
