use super::*;

/// The OrientationSensor class.
/// [`OrientationSensor`](https://developer.mozilla.org/en-US/docs/Web/API/OrientationSensor)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct OrientationSensor {
    inner: Sensor,
}
impl FromVal for OrientationSensor {
    fn from_val(v: &Any) -> Self {
        OrientationSensor {
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
impl core::ops::Deref for OrientationSensor {
    type Target = Sensor;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for OrientationSensor {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for OrientationSensor {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for OrientationSensor {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<OrientationSensor> for Any {
    fn from(s: OrientationSensor) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&OrientationSensor> for Any {
    fn from(s: &OrientationSensor) -> Any {
        s.inner.clone().into()
    }
}
jsbind::utils::impl_dyn_cast!(OrientationSensor);

impl OrientationSensor {
    /// Getter of the `quaternion` attribute.
    /// [`OrientationSensor.quaternion`](https://developer.mozilla.org/en-US/docs/Web/API/OrientationSensor/quaternion)
    pub fn quaternion(&self) -> TypedArray<f64> {
        self.inner.get("quaternion").as_::<TypedArray<f64>>()
    }
}
impl OrientationSensor {
    /// The populateMatrix method.
    /// [`OrientationSensor.populateMatrix`](https://developer.mozilla.org/en-US/docs/Web/API/OrientationSensor/populateMatrix)
    pub fn populate_matrix(&self, target_matrix: &Any) -> Undefined {
        self.inner
            .call("populateMatrix", &[target_matrix.into()])
            .as_::<Undefined>()
    }
}
