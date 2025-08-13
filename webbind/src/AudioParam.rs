use super::*;




/// The AudioParam class.
/// [`AudioParam`](https://developer.mozilla.org/en-US/docs/Web/API/AudioParam)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct AudioParam {
    inner: Any,
}

impl FromVal for AudioParam {
    fn from_val(v: &Any) -> Self {
        AudioParam { inner: Any::from_val(v) }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for AudioParam {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for AudioParam {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for AudioParam {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for AudioParam {
    fn as_mut(&mut self) -> &mut Any {
      &mut self.inner
  }
}

impl From<AudioParam> for Any {
    fn from(s: AudioParam) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&AudioParam> for Any {
    fn from(s: &AudioParam) -> Any {
        s.inner.clone().into()
    }
}

jsbind::utils::impl_dyn_cast!(AudioParam);


impl AudioParam {
    /// Getter of the `value` attribute.
    /// [`AudioParam.value`](https://developer.mozilla.org/en-US/docs/Web/API/AudioParam/value)
    pub fn value(&self) -> f32 {
        self.inner.get("value").as_::<f32>()
    }

    /// Setter of the `value` attribute.
    /// [`AudioParam.value`](https://developer.mozilla.org/en-US/docs/Web/API/AudioParam/value)
    pub fn set_value(&mut self, value: f32) {
        self.inner.set("value", value);
    }
}
impl AudioParam {
    /// Getter of the `automationRate` attribute.
    /// [`AudioParam.automationRate`](https://developer.mozilla.org/en-US/docs/Web/API/AudioParam/automationRate)
    pub fn automation_rate(&self) -> AutomationRate {
        self.inner.get("automationRate").as_::<AutomationRate>()
    }

    /// Setter of the `automationRate` attribute.
    /// [`AudioParam.automationRate`](https://developer.mozilla.org/en-US/docs/Web/API/AudioParam/automationRate)
    pub fn set_automation_rate(&mut self, value: &AutomationRate) {
        self.inner.set("automationRate", value);
    }
}
impl AudioParam {
    /// Getter of the `defaultValue` attribute.
    /// [`AudioParam.defaultValue`](https://developer.mozilla.org/en-US/docs/Web/API/AudioParam/defaultValue)
    pub fn default_value(&self) -> f32 {
        self.inner.get("defaultValue").as_::<f32>()
    }

}
impl AudioParam {
    /// Getter of the `minValue` attribute.
    /// [`AudioParam.minValue`](https://developer.mozilla.org/en-US/docs/Web/API/AudioParam/minValue)
    pub fn min_value(&self) -> f32 {
        self.inner.get("minValue").as_::<f32>()
    }

}
impl AudioParam {
    /// Getter of the `maxValue` attribute.
    /// [`AudioParam.maxValue`](https://developer.mozilla.org/en-US/docs/Web/API/AudioParam/maxValue)
    pub fn max_value(&self) -> f32 {
        self.inner.get("maxValue").as_::<f32>()
    }

}
impl AudioParam {
    /// The setValueAtTime method.
    /// [`AudioParam.setValueAtTime`](https://developer.mozilla.org/en-US/docs/Web/API/AudioParam/setValueAtTime)
    pub fn set_value_at_time(&self, value: f32, start_time: f64) -> AudioParam {
        self.inner.call("setValueAtTime", &[value.into(), start_time.into(), ]).as_::<AudioParam>()
    }
}
impl AudioParam {
    /// The linearRampToValueAtTime method.
    /// [`AudioParam.linearRampToValueAtTime`](https://developer.mozilla.org/en-US/docs/Web/API/AudioParam/linearRampToValueAtTime)
    pub fn linear_ramp_to_value_at_time(&self, value: f32, end_time: f64) -> AudioParam {
        self.inner.call("linearRampToValueAtTime", &[value.into(), end_time.into(), ]).as_::<AudioParam>()
    }
}
impl AudioParam {
    /// The exponentialRampToValueAtTime method.
    /// [`AudioParam.exponentialRampToValueAtTime`](https://developer.mozilla.org/en-US/docs/Web/API/AudioParam/exponentialRampToValueAtTime)
    pub fn exponential_ramp_to_value_at_time(&self, value: f32, end_time: f64) -> AudioParam {
        self.inner.call("exponentialRampToValueAtTime", &[value.into(), end_time.into(), ]).as_::<AudioParam>()
    }
}
impl AudioParam {
    /// The setTargetAtTime method.
    /// [`AudioParam.setTargetAtTime`](https://developer.mozilla.org/en-US/docs/Web/API/AudioParam/setTargetAtTime)
    pub fn set_target_at_time(&self, target: f32, start_time: f64, time_constant: f32) -> AudioParam {
        self.inner.call("setTargetAtTime", &[target.into(), start_time.into(), time_constant.into(), ]).as_::<AudioParam>()
    }
}
impl AudioParam {
    /// The setValueCurveAtTime method.
    /// [`AudioParam.setValueCurveAtTime`](https://developer.mozilla.org/en-US/docs/Web/API/AudioParam/setValueCurveAtTime)
    pub fn set_value_curve_at_time(&self, values: TypedArray<f32>, start_time: f64, duration: f64) -> AudioParam {
        self.inner.call("setValueCurveAtTime", &[values.into(), start_time.into(), duration.into(), ]).as_::<AudioParam>()
    }
}
impl AudioParam {
    /// The cancelScheduledValues method.
    /// [`AudioParam.cancelScheduledValues`](https://developer.mozilla.org/en-US/docs/Web/API/AudioParam/cancelScheduledValues)
    pub fn cancel_scheduled_values(&self, cancel_time: f64) -> AudioParam {
        self.inner.call("cancelScheduledValues", &[cancel_time.into(), ]).as_::<AudioParam>()
    }
}
impl AudioParam {
    /// The cancelAndHoldAtTime method.
    /// [`AudioParam.cancelAndHoldAtTime`](https://developer.mozilla.org/en-US/docs/Web/API/AudioParam/cancelAndHoldAtTime)
    pub fn cancel_and_hold_at_time(&self, cancel_time: f64) -> AudioParam {
        self.inner.call("cancelAndHoldAtTime", &[cancel_time.into(), ]).as_::<AudioParam>()
    }
}
