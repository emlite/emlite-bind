use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct GamepadPose {
    inner: emlite::Val,
}
impl FromVal for GamepadPose {
    fn from_val(v: &emlite::Val) -> Self {
        GamepadPose {
            inner: emlite::Val::from_val(v),
        }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for GamepadPose {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for GamepadPose {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for GamepadPose {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for GamepadPose {
    fn as_mut(&mut self) -> &mut emlite::Val {
        &mut self.inner
    }
}
impl From<GamepadPose> for emlite::Val {
    fn from(s: GamepadPose) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}
impl From<&GamepadPose> for emlite::Val {
    fn from(s: &GamepadPose) -> emlite::Val {
        s.inner.clone().into()
    }
}
jsbind::utils::impl_dyn_cast!(GamepadPose);

impl GamepadPose {
    pub fn has_orientation(&self) -> bool {
        self.inner.get("hasOrientation").as_::<bool>()
    }
}
impl GamepadPose {
    pub fn has_position(&self) -> bool {
        self.inner.get("hasPosition").as_::<bool>()
    }
}
impl GamepadPose {
    pub fn position(&self) -> Float32Array {
        self.inner.get("position").as_::<Float32Array>()
    }
}
impl GamepadPose {
    pub fn linear_velocity(&self) -> Float32Array {
        self.inner.get("linearVelocity").as_::<Float32Array>()
    }
}
impl GamepadPose {
    pub fn linear_acceleration(&self) -> Float32Array {
        self.inner.get("linearAcceleration").as_::<Float32Array>()
    }
}
impl GamepadPose {
    pub fn orientation(&self) -> Float32Array {
        self.inner.get("orientation").as_::<Float32Array>()
    }
}
impl GamepadPose {
    pub fn angular_velocity(&self) -> Float32Array {
        self.inner.get("angularVelocity").as_::<Float32Array>()
    }
}
impl GamepadPose {
    pub fn angular_acceleration(&self) -> Float32Array {
        self.inner.get("angularAcceleration").as_::<Float32Array>()
    }
}
