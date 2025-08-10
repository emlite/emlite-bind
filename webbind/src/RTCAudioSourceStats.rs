use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct RTCAudioSourceStats {
    inner: Any,
}
impl FromVal for RTCAudioSourceStats {
    fn from_val(v: &Any) -> Self {
        RTCAudioSourceStats { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for RTCAudioSourceStats {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for RTCAudioSourceStats {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for RTCAudioSourceStats {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for RTCAudioSourceStats {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<RTCAudioSourceStats> for Any {
    fn from(s: RTCAudioSourceStats) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&RTCAudioSourceStats> for Any {
    fn from(s: &RTCAudioSourceStats) -> Any {
        s.inner.clone()
    }
}

impl RTCAudioSourceStats {
    pub fn audio_level(&self) -> f64 {
        self.inner.get("audioLevel").as_::<f64>()
    }

    pub fn set_audio_level(&mut self, value: f64) {
        self.inner.set("audioLevel", value);
    }
}
impl RTCAudioSourceStats {
    pub fn total_audio_energy(&self) -> f64 {
        self.inner.get("totalAudioEnergy").as_::<f64>()
    }

    pub fn set_total_audio_energy(&mut self, value: f64) {
        self.inner.set("totalAudioEnergy", value);
    }
}
impl RTCAudioSourceStats {
    pub fn total_samples_duration(&self) -> f64 {
        self.inner.get("totalSamplesDuration").as_::<f64>()
    }

    pub fn set_total_samples_duration(&mut self, value: f64) {
        self.inner.set("totalSamplesDuration", value);
    }
}
impl RTCAudioSourceStats {
    pub fn echo_return_loss(&self) -> f64 {
        self.inner.get("echoReturnLoss").as_::<f64>()
    }

    pub fn set_echo_return_loss(&mut self, value: f64) {
        self.inner.set("echoReturnLoss", value);
    }
}
impl RTCAudioSourceStats {
    pub fn echo_return_loss_enhancement(&self) -> f64 {
        self.inner.get("echoReturnLossEnhancement").as_::<f64>()
    }

    pub fn set_echo_return_loss_enhancement(&mut self, value: f64) {
        self.inner.set("echoReturnLossEnhancement", value);
    }
}
