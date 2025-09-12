use super::*;

/// The ProximitySensor class.
/// [`ProximitySensor`](https://developer.mozilla.org/en-US/docs/Web/API/ProximitySensor)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct ProximitySensor {
    inner: Sensor,
}

impl FromVal for ProximitySensor {
    fn from_val(v: &Any) -> Self {
        ProximitySensor {
            inner: Sensor::from_val(v),
        }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for ProximitySensor {
    type Target = Sensor;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for ProximitySensor {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for ProximitySensor {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for ProximitySensor {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}

impl From<ProximitySensor> for Any {
    fn from(s: ProximitySensor) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&ProximitySensor> for Any {
    fn from(s: &ProximitySensor) -> Any {
        s.inner.clone().into()
    }
}

jsbind::utils::impl_dyn_cast!(ProximitySensor);

impl ProximitySensor {
    /// Getter of the `distance` attribute.
    /// [`ProximitySensor.distance`](https://developer.mozilla.org/en-US/docs/Web/API/ProximitySensor/distance)
    pub fn distance(&self) -> f64 {
        self.inner.get("distance").as_::<f64>()
    }
}
impl ProximitySensor {
    /// Getter of the `max` attribute.
    /// [`ProximitySensor.max`](https://developer.mozilla.org/en-US/docs/Web/API/ProximitySensor/max)
    pub fn max(&self) -> f64 {
        self.inner.get("max").as_::<f64>()
    }
}
impl ProximitySensor {
    /// Getter of the `near` attribute.
    /// [`ProximitySensor.near`](https://developer.mozilla.org/en-US/docs/Web/API/ProximitySensor/near)
    pub fn near(&self) -> bool {
        self.inner.get("near").as_::<bool>()
    }
}

impl ProximitySensor {
    /// The `new ProximitySensor(..)` constructor, creating a new ProximitySensor instance
    pub fn new0() -> ProximitySensor {
        Self {
            inner: Any::global("ProximitySensor").new(&[]).as_::<Sensor>(),
        }
    }

    /// The `new ProximitySensor(..)` constructor, creating a new ProximitySensor instance
    pub fn new1(sensor_options: &SensorOptions) -> ProximitySensor {
        Self {
            inner: Any::global("ProximitySensor")
                .new(&[sensor_options.into()])
                .as_::<Sensor>(),
        }
    }
}
