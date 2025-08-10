use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct RTCAudioPlayoutStats {
    inner: Any,
}
impl FromVal for RTCAudioPlayoutStats {
    fn from_val(v: &Any) -> Self {
        RTCAudioPlayoutStats { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for RTCAudioPlayoutStats {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for RTCAudioPlayoutStats {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for RTCAudioPlayoutStats {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for RTCAudioPlayoutStats {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<RTCAudioPlayoutStats> for Any {
    fn from(s: RTCAudioPlayoutStats) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&RTCAudioPlayoutStats> for Any {
    fn from(s: &RTCAudioPlayoutStats) -> Any {
        s.inner.clone()
    }
}

impl RTCAudioPlayoutStats {
    pub fn kind(&self) -> JsString {
        self.inner.get("kind").as_::<JsString>()
    }

    pub fn set_kind(&mut self, value: &JsString) {
        self.inner.set("kind", value);
    }
}
impl RTCAudioPlayoutStats {
    pub fn synthesized_samples_duration(&self) -> f64 {
        self.inner.get("synthesizedSamplesDuration").as_::<f64>()
    }

    pub fn set_synthesized_samples_duration(&mut self, value: f64) {
        self.inner.set("synthesizedSamplesDuration", value);
    }
}
impl RTCAudioPlayoutStats {
    pub fn synthesized_samples_events(&self) -> u32 {
        self.inner.get("synthesizedSamplesEvents").as_::<u32>()
    }

    pub fn set_synthesized_samples_events(&mut self, value: u32) {
        self.inner.set("synthesizedSamplesEvents", value);
    }
}
impl RTCAudioPlayoutStats {
    pub fn total_samples_duration(&self) -> f64 {
        self.inner.get("totalSamplesDuration").as_::<f64>()
    }

    pub fn set_total_samples_duration(&mut self, value: f64) {
        self.inner.set("totalSamplesDuration", value);
    }
}
impl RTCAudioPlayoutStats {
    pub fn total_playout_delay(&self) -> f64 {
        self.inner.get("totalPlayoutDelay").as_::<f64>()
    }

    pub fn set_total_playout_delay(&mut self, value: f64) {
        self.inner.set("totalPlayoutDelay", value);
    }
}
impl RTCAudioPlayoutStats {
    pub fn total_samples_count(&self) -> u64 {
        self.inner.get("totalSamplesCount").as_::<u64>()
    }

    pub fn set_total_samples_count(&mut self, value: u64) {
        self.inner.set("totalSamplesCount", value);
    }
}
