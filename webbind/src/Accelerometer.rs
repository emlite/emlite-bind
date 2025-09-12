use super::*;

/// The Accelerometer class.
/// [`Accelerometer`](https://developer.mozilla.org/en-US/docs/Web/API/Accelerometer)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct Accelerometer {
    inner: Sensor,
}

impl FromVal for Accelerometer {
    fn from_val(v: &Any) -> Self {
        Accelerometer {
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

impl core::ops::Deref for Accelerometer {
    type Target = Sensor;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for Accelerometer {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for Accelerometer {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for Accelerometer {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}

impl From<Accelerometer> for Any {
    fn from(s: Accelerometer) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&Accelerometer> for Any {
    fn from(s: &Accelerometer) -> Any {
        s.inner.clone().into()
    }
}

jsbind::utils::impl_dyn_cast!(Accelerometer);

impl Accelerometer {
    /// Getter of the `x` attribute.
    /// [`Accelerometer.x`](https://developer.mozilla.org/en-US/docs/Web/API/Accelerometer/x)
    pub fn x(&self) -> f64 {
        self.inner.get("x").as_::<f64>()
    }
}
impl Accelerometer {
    /// Getter of the `y` attribute.
    /// [`Accelerometer.y`](https://developer.mozilla.org/en-US/docs/Web/API/Accelerometer/y)
    pub fn y(&self) -> f64 {
        self.inner.get("y").as_::<f64>()
    }
}
impl Accelerometer {
    /// Getter of the `z` attribute.
    /// [`Accelerometer.z`](https://developer.mozilla.org/en-US/docs/Web/API/Accelerometer/z)
    pub fn z(&self) -> f64 {
        self.inner.get("z").as_::<f64>()
    }
}

impl Accelerometer {
    /// The `new Accelerometer(..)` constructor, creating a new Accelerometer instance
    pub fn new0() -> Accelerometer {
        Self {
            inner: Any::global("Accelerometer").new(&[]).as_::<Sensor>(),
        }
    }

    /// The `new Accelerometer(..)` constructor, creating a new Accelerometer instance
    pub fn new1(options: &AccelerometerSensorOptions) -> Accelerometer {
        Self {
            inner: Any::global("Accelerometer")
                .new(&[options.into()])
                .as_::<Sensor>(),
        }
    }
}
