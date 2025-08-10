use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct AudioParamDescriptor {
    inner: Any,
}
impl FromVal for AudioParamDescriptor {
    fn from_val(v: &Any) -> Self {
        AudioParamDescriptor { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for AudioParamDescriptor {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for AudioParamDescriptor {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for AudioParamDescriptor {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for AudioParamDescriptor {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<AudioParamDescriptor> for Any {
    fn from(s: AudioParamDescriptor) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&AudioParamDescriptor> for Any {
    fn from(s: &AudioParamDescriptor) -> Any {
        s.inner.clone()
    }
}

impl AudioParamDescriptor {
    pub fn name(&self) -> JsString {
        self.inner.get("name").as_::<JsString>()
    }

    pub fn set_name(&mut self, value: &JsString) {
        self.inner.set("name", value);
    }
}
impl AudioParamDescriptor {
    pub fn default_value(&self) -> f32 {
        self.inner.get("defaultValue").as_::<f32>()
    }

    pub fn set_default_value(&mut self, value: f32) {
        self.inner.set("defaultValue", value);
    }
}
impl AudioParamDescriptor {
    pub fn min_value(&self) -> f32 {
        self.inner.get("minValue").as_::<f32>()
    }

    pub fn set_min_value(&mut self, value: f32) {
        self.inner.set("minValue", value);
    }
}
impl AudioParamDescriptor {
    pub fn max_value(&self) -> f32 {
        self.inner.get("maxValue").as_::<f32>()
    }

    pub fn set_max_value(&mut self, value: f32) {
        self.inner.set("maxValue", value);
    }
}
impl AudioParamDescriptor {
    pub fn automation_rate(&self) -> AutomationRate {
        self.inner.get("automationRate").as_::<AutomationRate>()
    }

    pub fn set_automation_rate(&mut self, value: &AutomationRate) {
        self.inner.set("automationRate", value);
    }
}
