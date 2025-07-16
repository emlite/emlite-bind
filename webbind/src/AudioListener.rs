use super::*;

/// The AudioListener class.
/// [`AudioListener`](https://developer.mozilla.org/en-US/docs/Web/API/AudioListener)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct AudioListener {
    inner: Any,
}
impl FromVal for AudioListener {
    fn from_val(v: &Any) -> Self {
        AudioListener {
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
impl core::ops::Deref for AudioListener {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for AudioListener {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for AudioListener {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for AudioListener {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<AudioListener> for Any {
    fn from(s: AudioListener) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&AudioListener> for Any {
    fn from(s: &AudioListener) -> Any {
        s.inner.clone().into()
    }
}
jsbind::utils::impl_dyn_cast!(AudioListener);

impl AudioListener {
    /// Getter of the `positionX` attribute.
    /// [`AudioListener.positionX`](https://developer.mozilla.org/en-US/docs/Web/API/AudioListener/positionX)
    pub fn position_x(&self) -> AudioParam {
        self.inner.get("positionX").as_::<AudioParam>()
    }
}
impl AudioListener {
    /// Getter of the `positionY` attribute.
    /// [`AudioListener.positionY`](https://developer.mozilla.org/en-US/docs/Web/API/AudioListener/positionY)
    pub fn position_y(&self) -> AudioParam {
        self.inner.get("positionY").as_::<AudioParam>()
    }
}
impl AudioListener {
    /// Getter of the `positionZ` attribute.
    /// [`AudioListener.positionZ`](https://developer.mozilla.org/en-US/docs/Web/API/AudioListener/positionZ)
    pub fn position_z(&self) -> AudioParam {
        self.inner.get("positionZ").as_::<AudioParam>()
    }
}
impl AudioListener {
    /// Getter of the `forwardX` attribute.
    /// [`AudioListener.forwardX`](https://developer.mozilla.org/en-US/docs/Web/API/AudioListener/forwardX)
    pub fn forward_x(&self) -> AudioParam {
        self.inner.get("forwardX").as_::<AudioParam>()
    }
}
impl AudioListener {
    /// Getter of the `forwardY` attribute.
    /// [`AudioListener.forwardY`](https://developer.mozilla.org/en-US/docs/Web/API/AudioListener/forwardY)
    pub fn forward_y(&self) -> AudioParam {
        self.inner.get("forwardY").as_::<AudioParam>()
    }
}
impl AudioListener {
    /// Getter of the `forwardZ` attribute.
    /// [`AudioListener.forwardZ`](https://developer.mozilla.org/en-US/docs/Web/API/AudioListener/forwardZ)
    pub fn forward_z(&self) -> AudioParam {
        self.inner.get("forwardZ").as_::<AudioParam>()
    }
}
impl AudioListener {
    /// Getter of the `upX` attribute.
    /// [`AudioListener.upX`](https://developer.mozilla.org/en-US/docs/Web/API/AudioListener/upX)
    pub fn up_x(&self) -> AudioParam {
        self.inner.get("upX").as_::<AudioParam>()
    }
}
impl AudioListener {
    /// Getter of the `upY` attribute.
    /// [`AudioListener.upY`](https://developer.mozilla.org/en-US/docs/Web/API/AudioListener/upY)
    pub fn up_y(&self) -> AudioParam {
        self.inner.get("upY").as_::<AudioParam>()
    }
}
impl AudioListener {
    /// Getter of the `upZ` attribute.
    /// [`AudioListener.upZ`](https://developer.mozilla.org/en-US/docs/Web/API/AudioListener/upZ)
    pub fn up_z(&self) -> AudioParam {
        self.inner.get("upZ").as_::<AudioParam>()
    }
}
impl AudioListener {
    /// The setPosition method.
    /// [`AudioListener.setPosition`](https://developer.mozilla.org/en-US/docs/Web/API/AudioListener/setPosition)
    pub fn set_position(&self, x: f32, y: f32, z: f32) -> Undefined {
        self.inner
            .call("setPosition", &[x.into(), y.into(), z.into()])
            .as_::<Undefined>()
    }
}
impl AudioListener {
    /// The setOrientation method.
    /// [`AudioListener.setOrientation`](https://developer.mozilla.org/en-US/docs/Web/API/AudioListener/setOrientation)
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
