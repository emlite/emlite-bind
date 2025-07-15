use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct AudioParam {
    inner: emlite::Val,
}
impl FromVal for AudioParam {
    fn from_val(v: &emlite::Val) -> Self {
        AudioParam {
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
impl core::ops::Deref for AudioParam {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for AudioParam {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for AudioParam {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for AudioParam {
    fn as_mut(&mut self) -> &mut emlite::Val {
        &mut self.inner
    }
}
impl From<AudioParam> for emlite::Val {
    fn from(s: AudioParam) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}
impl From<&AudioParam> for emlite::Val {
    fn from(s: &AudioParam) -> emlite::Val {
        s.inner.clone().into()
    }
}
jsbind::utils::impl_dyn_cast!(AudioParam);

impl AudioParam {
    pub fn value(&self) -> f32 {
        self.inner.get("value").as_::<f32>()
    }

    pub fn set_value(&mut self, value: f32) {
        self.inner.set("value", value);
    }
}
impl AudioParam {
    pub fn automation_rate(&self) -> AutomationRate {
        self.inner.get("automationRate").as_::<AutomationRate>()
    }

    pub fn set_automation_rate(&mut self, value: AutomationRate) {
        self.inner.set("automationRate", value);
    }
}
impl AudioParam {
    pub fn default_value(&self) -> f32 {
        self.inner.get("defaultValue").as_::<f32>()
    }
}
impl AudioParam {
    pub fn min_value(&self) -> f32 {
        self.inner.get("minValue").as_::<f32>()
    }
}
impl AudioParam {
    pub fn max_value(&self) -> f32 {
        self.inner.get("maxValue").as_::<f32>()
    }
}
impl AudioParam {
    pub fn set_value_at_time(&self, value: f32, start_time: f64) -> AudioParam {
        self.inner
            .call("setValueAtTime", &[value.into(), start_time.into()])
            .as_::<AudioParam>()
    }
}
impl AudioParam {
    pub fn linear_ramp_to_value_at_time(&self, value: f32, end_time: f64) -> AudioParam {
        self.inner
            .call("linearRampToValueAtTime", &[value.into(), end_time.into()])
            .as_::<AudioParam>()
    }
}
impl AudioParam {
    pub fn exponential_ramp_to_value_at_time(&self, value: f32, end_time: f64) -> AudioParam {
        self.inner
            .call(
                "exponentialRampToValueAtTime",
                &[value.into(), end_time.into()],
            )
            .as_::<AudioParam>()
    }
}
impl AudioParam {
    pub fn set_target_at_time(
        &self,
        target: f32,
        start_time: f64,
        time_constant: f32,
    ) -> AudioParam {
        self.inner
            .call(
                "setTargetAtTime",
                &[target.into(), start_time.into(), time_constant.into()],
            )
            .as_::<AudioParam>()
    }
}
impl AudioParam {
    pub fn set_value_curve_at_time(
        &self,
        values: Sequence<f32>,
        start_time: f64,
        duration: f64,
    ) -> AudioParam {
        self.inner
            .call(
                "setValueCurveAtTime",
                &[values.into(), start_time.into(), duration.into()],
            )
            .as_::<AudioParam>()
    }
}
impl AudioParam {
    pub fn cancel_scheduled_values(&self, cancel_time: f64) -> AudioParam {
        self.inner
            .call("cancelScheduledValues", &[cancel_time.into()])
            .as_::<AudioParam>()
    }
}
impl AudioParam {
    pub fn cancel_and_hold_at_time(&self, cancel_time: f64) -> AudioParam {
        self.inner
            .call("cancelAndHoldAtTime", &[cancel_time.into()])
            .as_::<AudioParam>()
    }
}
