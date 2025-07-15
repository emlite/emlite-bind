use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct AudioListener {
    inner: emlite::Val,
}
impl FromVal for AudioListener {
    fn from_val(v: &emlite::Val) -> Self {
        AudioListener {
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
impl core::ops::Deref for AudioListener {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for AudioListener {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for AudioListener {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for AudioListener {
    fn as_mut(&mut self) -> &mut emlite::Val {
        &mut self.inner
    }
}
impl From<AudioListener> for emlite::Val {
    fn from(s: AudioListener) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}
jsbind::utils::impl_dyn_cast!(AudioListener);

impl AudioListener {
    pub fn position_x(&self) -> AudioParam {
        self.inner.get("positionX").as_::<AudioParam>()
    }
}
impl AudioListener {
    pub fn position_y(&self) -> AudioParam {
        self.inner.get("positionY").as_::<AudioParam>()
    }
}
impl AudioListener {
    pub fn position_z(&self) -> AudioParam {
        self.inner.get("positionZ").as_::<AudioParam>()
    }
}
impl AudioListener {
    pub fn forward_x(&self) -> AudioParam {
        self.inner.get("forwardX").as_::<AudioParam>()
    }
}
impl AudioListener {
    pub fn forward_y(&self) -> AudioParam {
        self.inner.get("forwardY").as_::<AudioParam>()
    }
}
impl AudioListener {
    pub fn forward_z(&self) -> AudioParam {
        self.inner.get("forwardZ").as_::<AudioParam>()
    }
}
impl AudioListener {
    pub fn up_x(&self) -> AudioParam {
        self.inner.get("upX").as_::<AudioParam>()
    }
}
impl AudioListener {
    pub fn up_y(&self) -> AudioParam {
        self.inner.get("upY").as_::<AudioParam>()
    }
}
impl AudioListener {
    pub fn up_z(&self) -> AudioParam {
        self.inner.get("upZ").as_::<AudioParam>()
    }
}
impl AudioListener {
    pub fn set_position(&self, x: f32, y: f32, z: f32) -> Undefined {
        self.inner
            .call("setPosition", &[x.into(), y.into(), z.into()])
            .as_::<Undefined>()
    }
}
impl AudioListener {
    pub fn set_orientation(
        &self,
        x: f32,
        y: f32,
        z: f32,
        x_up: f32,
        y_up: f32,
        z_up: f32,
    ) -> Undefined {
        self.inner
            .call(
                "setOrientation",
                &[
                    x.into(),
                    y.into(),
                    z.into(),
                    x_up.into(),
                    y_up.into(),
                    z_up.into(),
                ],
            )
            .as_::<Undefined>()
    }
}
