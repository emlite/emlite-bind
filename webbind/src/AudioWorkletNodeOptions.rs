use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct AudioWorkletNodeOptions {
    inner: Any,
}
impl FromVal for AudioWorkletNodeOptions {
    fn from_val(v: &Any) -> Self {
        AudioWorkletNodeOptions { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for AudioWorkletNodeOptions {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for AudioWorkletNodeOptions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for AudioWorkletNodeOptions {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for AudioWorkletNodeOptions {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<AudioWorkletNodeOptions> for Any {
    fn from(s: AudioWorkletNodeOptions) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&AudioWorkletNodeOptions> for Any {
    fn from(s: &AudioWorkletNodeOptions) -> Any {
        s.inner.clone()
    }
}

impl AudioWorkletNodeOptions {
    pub fn number_of_inputs(&self) -> u32 {
        self.inner.get("numberOfInputs").as_::<u32>()
    }

    pub fn set_number_of_inputs(&mut self, value: u32) {
        self.inner.set("numberOfInputs", value);
    }
}
impl AudioWorkletNodeOptions {
    pub fn number_of_outputs(&self) -> u32 {
        self.inner.get("numberOfOutputs").as_::<u32>()
    }

    pub fn set_number_of_outputs(&mut self, value: u32) {
        self.inner.set("numberOfOutputs", value);
    }
}
impl AudioWorkletNodeOptions {
    pub fn output_channel_count(&self) -> TypedArray<u32> {
        self.inner
            .get("outputChannelCount")
            .as_::<TypedArray<u32>>()
    }

    pub fn set_output_channel_count(&mut self, value: TypedArray<u32>) {
        self.inner.set("outputChannelCount", value);
    }
}
impl AudioWorkletNodeOptions {
    pub fn parameter_data(&self) -> Record<JsString, f64> {
        self.inner
            .get("parameterData")
            .as_::<Record<JsString, f64>>()
    }

    pub fn set_parameter_data(&mut self, value: Record<JsString, f64>) {
        self.inner.set("parameterData", value);
    }
}
impl AudioWorkletNodeOptions {
    pub fn processor_options(&self) -> Object {
        self.inner.get("processorOptions").as_::<Object>()
    }

    pub fn set_processor_options(&mut self, value: &Object) {
        self.inner.set("processorOptions", value);
    }
}
