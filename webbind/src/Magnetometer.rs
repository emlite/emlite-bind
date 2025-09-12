use super::*;

/// The Magnetometer class.
/// [`Magnetometer`](https://developer.mozilla.org/en-US/docs/Web/API/Magnetometer)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct Magnetometer {
    inner: Sensor,
}

impl FromVal for Magnetometer {
    fn from_val(v: &Any) -> Self {
        Magnetometer {
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

impl core::ops::Deref for Magnetometer {
    type Target = Sensor;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for Magnetometer {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for Magnetometer {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for Magnetometer {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}

impl From<Magnetometer> for Any {
    fn from(s: Magnetometer) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&Magnetometer> for Any {
    fn from(s: &Magnetometer) -> Any {
        s.inner.clone().into()
    }
}

jsbind::utils::impl_dyn_cast!(Magnetometer);

impl Magnetometer {
    /// Getter of the `x` attribute.
    /// [`Magnetometer.x`](https://developer.mozilla.org/en-US/docs/Web/API/Magnetometer/x)
    pub fn x(&self) -> f64 {
        self.inner.get("x").as_::<f64>()
    }
}
impl Magnetometer {
    /// Getter of the `y` attribute.
    /// [`Magnetometer.y`](https://developer.mozilla.org/en-US/docs/Web/API/Magnetometer/y)
    pub fn y(&self) -> f64 {
        self.inner.get("y").as_::<f64>()
    }
}
impl Magnetometer {
    /// Getter of the `z` attribute.
    /// [`Magnetometer.z`](https://developer.mozilla.org/en-US/docs/Web/API/Magnetometer/z)
    pub fn z(&self) -> f64 {
        self.inner.get("z").as_::<f64>()
    }
}

impl Magnetometer {
    /// The `new Magnetometer(..)` constructor, creating a new Magnetometer instance
    pub fn new() -> Magnetometer {
        Self {
            inner: Any::global("Magnetometer").new(&[]).as_::<Sensor>(),
        }
    }
}

impl Magnetometer {
    /// The `new Magnetometer(..)` constructor, creating a new Magnetometer instance
    pub fn new_with_sensor_options(sensor_options: &MagnetometerSensorOptions) -> Magnetometer {
        Self {
            inner: Any::global("Magnetometer")
                .new(&[sensor_options.into()])
                .as_::<Sensor>(),
        }
    }
}
