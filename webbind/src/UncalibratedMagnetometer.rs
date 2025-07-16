use super::*;

/// The UncalibratedMagnetometer class.
/// [`UncalibratedMagnetometer`](https://developer.mozilla.org/en-US/docs/Web/API/UncalibratedMagnetometer)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct UncalibratedMagnetometer {
    inner: Sensor,
}
impl FromVal for UncalibratedMagnetometer {
    fn from_val(v: &Any) -> Self {
        UncalibratedMagnetometer {
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
impl core::ops::Deref for UncalibratedMagnetometer {
    type Target = Sensor;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for UncalibratedMagnetometer {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for UncalibratedMagnetometer {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for UncalibratedMagnetometer {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<UncalibratedMagnetometer> for Any {
    fn from(s: UncalibratedMagnetometer) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&UncalibratedMagnetometer> for Any {
    fn from(s: &UncalibratedMagnetometer) -> Any {
        s.inner.clone().into()
    }
}
jsbind::utils::impl_dyn_cast!(UncalibratedMagnetometer);

impl UncalibratedMagnetometer {
    /// The `new UncalibratedMagnetometer(..)` constructor, creating a new UncalibratedMagnetometer instance
    pub fn new0() -> UncalibratedMagnetometer {
        Self {
            inner: Any::global("UncalibratedMagnetometer")
                .new(&[])
                .as_::<Sensor>(),
        }
    }

    /// The `new UncalibratedMagnetometer(..)` constructor, creating a new UncalibratedMagnetometer instance
    pub fn new1(sensor_options: &Any) -> UncalibratedMagnetometer {
        Self {
            inner: Any::global("UncalibratedMagnetometer")
                .new(&[sensor_options.into()])
                .as_::<Sensor>(),
        }
    }
}
impl UncalibratedMagnetometer {
    /// Getter of the `x` attribute.
    /// [`UncalibratedMagnetometer.x`](https://developer.mozilla.org/en-US/docs/Web/API/UncalibratedMagnetometer/x)
    pub fn x(&self) -> f64 {
        self.inner.get("x").as_::<f64>()
    }
}
impl UncalibratedMagnetometer {
    /// Getter of the `y` attribute.
    /// [`UncalibratedMagnetometer.y`](https://developer.mozilla.org/en-US/docs/Web/API/UncalibratedMagnetometer/y)
    pub fn y(&self) -> f64 {
        self.inner.get("y").as_::<f64>()
    }
}
impl UncalibratedMagnetometer {
    /// Getter of the `z` attribute.
    /// [`UncalibratedMagnetometer.z`](https://developer.mozilla.org/en-US/docs/Web/API/UncalibratedMagnetometer/z)
    pub fn z(&self) -> f64 {
        self.inner.get("z").as_::<f64>()
    }
}
impl UncalibratedMagnetometer {
    /// Getter of the `xBias` attribute.
    /// [`UncalibratedMagnetometer.xBias`](https://developer.mozilla.org/en-US/docs/Web/API/UncalibratedMagnetometer/xBias)
    pub fn x_bias(&self) -> f64 {
        self.inner.get("xBias").as_::<f64>()
    }
}
impl UncalibratedMagnetometer {
    /// Getter of the `yBias` attribute.
    /// [`UncalibratedMagnetometer.yBias`](https://developer.mozilla.org/en-US/docs/Web/API/UncalibratedMagnetometer/yBias)
    pub fn y_bias(&self) -> f64 {
        self.inner.get("yBias").as_::<f64>()
    }
}
impl UncalibratedMagnetometer {
    /// Getter of the `zBias` attribute.
    /// [`UncalibratedMagnetometer.zBias`](https://developer.mozilla.org/en-US/docs/Web/API/UncalibratedMagnetometer/zBias)
    pub fn z_bias(&self) -> f64 {
        self.inner.get("zBias").as_::<f64>()
    }
}
