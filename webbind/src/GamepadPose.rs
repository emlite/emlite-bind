use super::*;

/// The GamepadPose class.
/// [`GamepadPose`](https://developer.mozilla.org/en-US/docs/Web/API/GamepadPose)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct GamepadPose {
    inner: Any,
}

impl FromVal for GamepadPose {
    fn from_val(v: &Any) -> Self {
        GamepadPose {
            inner: Any::from_val(v),
        }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for GamepadPose {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for GamepadPose {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for GamepadPose {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for GamepadPose {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}

impl From<GamepadPose> for Any {
    fn from(s: GamepadPose) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&GamepadPose> for Any {
    fn from(s: &GamepadPose) -> Any {
        s.inner.clone().into()
    }
}

jsbind::utils::impl_dyn_cast!(GamepadPose);

impl GamepadPose {
    /// Getter of the `hasOrientation` attribute.
    /// [`GamepadPose.hasOrientation`](https://developer.mozilla.org/en-US/docs/Web/API/GamepadPose/hasOrientation)
    pub fn has_orientation(&self) -> bool {
        self.inner.get("hasOrientation").as_::<bool>()
    }
}
impl GamepadPose {
    /// Getter of the `hasPosition` attribute.
    /// [`GamepadPose.hasPosition`](https://developer.mozilla.org/en-US/docs/Web/API/GamepadPose/hasPosition)
    pub fn has_position(&self) -> bool {
        self.inner.get("hasPosition").as_::<bool>()
    }
}
impl GamepadPose {
    /// Getter of the `position` attribute.
    /// [`GamepadPose.position`](https://developer.mozilla.org/en-US/docs/Web/API/GamepadPose/position)
    pub fn position(&self) -> Float32Array {
        self.inner.get("position").as_::<Float32Array>()
    }
}
impl GamepadPose {
    /// Getter of the `linearVelocity` attribute.
    /// [`GamepadPose.linearVelocity`](https://developer.mozilla.org/en-US/docs/Web/API/GamepadPose/linearVelocity)
    pub fn linear_velocity(&self) -> Float32Array {
        self.inner.get("linearVelocity").as_::<Float32Array>()
    }
}
impl GamepadPose {
    /// Getter of the `linearAcceleration` attribute.
    /// [`GamepadPose.linearAcceleration`](https://developer.mozilla.org/en-US/docs/Web/API/GamepadPose/linearAcceleration)
    pub fn linear_acceleration(&self) -> Float32Array {
        self.inner.get("linearAcceleration").as_::<Float32Array>()
    }
}
impl GamepadPose {
    /// Getter of the `orientation` attribute.
    /// [`GamepadPose.orientation`](https://developer.mozilla.org/en-US/docs/Web/API/GamepadPose/orientation)
    pub fn orientation(&self) -> Float32Array {
        self.inner.get("orientation").as_::<Float32Array>()
    }
}
impl GamepadPose {
    /// Getter of the `angularVelocity` attribute.
    /// [`GamepadPose.angularVelocity`](https://developer.mozilla.org/en-US/docs/Web/API/GamepadPose/angularVelocity)
    pub fn angular_velocity(&self) -> Float32Array {
        self.inner.get("angularVelocity").as_::<Float32Array>()
    }
}
impl GamepadPose {
    /// Getter of the `angularAcceleration` attribute.
    /// [`GamepadPose.angularAcceleration`](https://developer.mozilla.org/en-US/docs/Web/API/GamepadPose/angularAcceleration)
    pub fn angular_acceleration(&self) -> Float32Array {
        self.inner.get("angularAcceleration").as_::<Float32Array>()
    }
}
