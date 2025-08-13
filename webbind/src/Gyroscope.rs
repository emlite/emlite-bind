use super::*;




/// The Gyroscope class.
/// [`Gyroscope`](https://developer.mozilla.org/en-US/docs/Web/API/Gyroscope)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct Gyroscope {
    inner: Sensor,
}

impl FromVal for Gyroscope {
    fn from_val(v: &Any) -> Self {
        Gyroscope { inner: Sensor::from_val(v) }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for Gyroscope {
    type Target = Sensor;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for Gyroscope {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for Gyroscope {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for Gyroscope {
    fn as_mut(&mut self) -> &mut Any {
      &mut self.inner
  }
}

impl From<Gyroscope> for Any {
    fn from(s: Gyroscope) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&Gyroscope> for Any {
    fn from(s: &Gyroscope) -> Any {
        s.inner.clone().into()
    }
}

jsbind::utils::impl_dyn_cast!(Gyroscope);



impl Gyroscope {
    /// The `new Gyroscope(..)` constructor, creating a new Gyroscope instance
    pub fn new0() -> Gyroscope {
        Self {
            inner: Any::global("Gyroscope").new(&[]).as_::<Sensor>(),
        }
    }

    /// The `new Gyroscope(..)` constructor, creating a new Gyroscope instance
    pub fn new1(sensor_options: &GyroscopeSensorOptions) -> Gyroscope {
        Self {
            inner: Any::global("Gyroscope").new(&[sensor_options.into()]).as_::<Sensor>(),
        }
    }

}
impl Gyroscope {
    /// Getter of the `x` attribute.
    /// [`Gyroscope.x`](https://developer.mozilla.org/en-US/docs/Web/API/Gyroscope/x)
    pub fn x(&self) -> f64 {
        self.inner.get("x").as_::<f64>()
    }

}
impl Gyroscope {
    /// Getter of the `y` attribute.
    /// [`Gyroscope.y`](https://developer.mozilla.org/en-US/docs/Web/API/Gyroscope/y)
    pub fn y(&self) -> f64 {
        self.inner.get("y").as_::<f64>()
    }

}
impl Gyroscope {
    /// Getter of the `z` attribute.
    /// [`Gyroscope.z`](https://developer.mozilla.org/en-US/docs/Web/API/Gyroscope/z)
    pub fn z(&self) -> f64 {
        self.inner.get("z").as_::<f64>()
    }

}
