use super::*;

/// The Sensor class.
/// [`Sensor`](https://developer.mozilla.org/en-US/docs/Web/API/Sensor)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct Sensor {
    inner: EventTarget,
}

impl FromVal for Sensor {
    fn from_val(v: &Any) -> Self {
        Sensor {
            inner: EventTarget::from_val(v),
        }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for Sensor {
    type Target = EventTarget;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for Sensor {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for Sensor {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for Sensor {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}

impl From<Sensor> for Any {
    fn from(s: Sensor) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&Sensor> for Any {
    fn from(s: &Sensor) -> Any {
        s.inner.clone().into()
    }
}

jsbind::utils::impl_dyn_cast!(Sensor);

impl Sensor {
    /// Getter of the `activated` attribute.
    /// [`Sensor.activated`](https://developer.mozilla.org/en-US/docs/Web/API/Sensor/activated)
    pub fn activated(&self) -> bool {
        self.inner.get("activated").as_::<bool>()
    }
}
impl Sensor {
    /// Getter of the `hasReading` attribute.
    /// [`Sensor.hasReading`](https://developer.mozilla.org/en-US/docs/Web/API/Sensor/hasReading)
    pub fn has_reading(&self) -> bool {
        self.inner.get("hasReading").as_::<bool>()
    }
}
impl Sensor {
    /// Getter of the `timestamp` attribute.
    /// [`Sensor.timestamp`](https://developer.mozilla.org/en-US/docs/Web/API/Sensor/timestamp)
    pub fn timestamp(&self) -> Any {
        self.inner.get("timestamp").as_::<Any>()
    }
}
impl Sensor {
    /// Getter of the `onreading` attribute.
    /// [`Sensor.onreading`](https://developer.mozilla.org/en-US/docs/Web/API/Sensor/onreading)
    pub fn onreading(&self) -> Any {
        self.inner.get("onreading").as_::<Any>()
    }

    /// Setter of the `onreading` attribute.
    /// [`Sensor.onreading`](https://developer.mozilla.org/en-US/docs/Web/API/Sensor/onreading)
    pub fn set_onreading(&mut self, value: &Any) {
        self.inner.set("onreading", value);
    }
}
impl Sensor {
    /// Getter of the `onactivate` attribute.
    /// [`Sensor.onactivate`](https://developer.mozilla.org/en-US/docs/Web/API/Sensor/onactivate)
    pub fn onactivate(&self) -> Any {
        self.inner.get("onactivate").as_::<Any>()
    }

    /// Setter of the `onactivate` attribute.
    /// [`Sensor.onactivate`](https://developer.mozilla.org/en-US/docs/Web/API/Sensor/onactivate)
    pub fn set_onactivate(&mut self, value: &Any) {
        self.inner.set("onactivate", value);
    }
}
impl Sensor {
    /// Getter of the `onerror` attribute.
    /// [`Sensor.onerror`](https://developer.mozilla.org/en-US/docs/Web/API/Sensor/onerror)
    pub fn onerror(&self) -> Any {
        self.inner.get("onerror").as_::<Any>()
    }

    /// Setter of the `onerror` attribute.
    /// [`Sensor.onerror`](https://developer.mozilla.org/en-US/docs/Web/API/Sensor/onerror)
    pub fn set_onerror(&mut self, value: &Any) {
        self.inner.set("onerror", value);
    }
}
impl Sensor {
    /// The start method.
    /// [`Sensor.start`](https://developer.mozilla.org/en-US/docs/Web/API/Sensor/start)
    pub fn start(&self) -> Undefined {
        self.inner.call("start", &[]).as_::<Undefined>()
    }
}
impl Sensor {
    /// The stop method.
    /// [`Sensor.stop`](https://developer.mozilla.org/en-US/docs/Web/API/Sensor/stop)
    pub fn stop(&self) -> Undefined {
        self.inner.call("stop", &[]).as_::<Undefined>()
    }
}
