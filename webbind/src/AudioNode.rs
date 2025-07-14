use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct AudioNode {
    inner: EventTarget,
}
impl FromVal for AudioNode {
    fn from_val(v: &emlite::Val) -> Self {
        AudioNode {
            inner: EventTarget::from_val(v),
        }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for AudioNode {
    type Target = EventTarget;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for AudioNode {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for AudioNode {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for AudioNode {
    fn as_mut(&mut self) -> &mut emlite::Val {
        &mut self.inner
    }
}
impl From<AudioNode> for emlite::Val {
    fn from(s: AudioNode) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}
jsbind::utils::impl_dyn_cast!(AudioNode);

impl AudioNode {
    pub fn connect0(&self, destination_param: AudioParam) -> jsbind::Undefined {
        self.inner
            .call("connect", &[destination_param.into()])
            .as_::<jsbind::Undefined>()
    }

    pub fn connect1(&self, destination_param: AudioParam, output: u32) -> jsbind::Undefined {
        self.inner
            .call("connect", &[destination_param.into(), output.into()])
            .as_::<jsbind::Undefined>()
    }
}
impl AudioNode {
    pub fn disconnect(&self, destination_param: AudioParam, output: u32) -> jsbind::Undefined {
        self.inner
            .call("disconnect", &[destination_param.into(), output.into()])
            .as_::<jsbind::Undefined>()
    }
}
impl AudioNode {
    pub fn context(&self) -> BaseAudioContext {
        self.inner.get("context").as_::<BaseAudioContext>()
    }
}
impl AudioNode {
    pub fn number_of_inputs(&self) -> u32 {
        self.inner.get("numberOfInputs").as_::<u32>()
    }
}
impl AudioNode {
    pub fn number_of_outputs(&self) -> u32 {
        self.inner.get("numberOfOutputs").as_::<u32>()
    }
}
impl AudioNode {
    pub fn channel_count(&self) -> u32 {
        self.inner.get("channelCount").as_::<u32>()
    }

    pub fn set_channel_count(&mut self, value: u32) {
        self.inner.set("channelCount", value);
    }
}
impl AudioNode {
    pub fn channel_count_mode(&self) -> ChannelCountMode {
        self.inner.get("channelCountMode").as_::<ChannelCountMode>()
    }

    pub fn set_channel_count_mode(&mut self, value: ChannelCountMode) {
        self.inner.set("channelCountMode", value);
    }
}
impl AudioNode {
    pub fn channel_interpretation(&self) -> ChannelInterpretation {
        self.inner
            .get("channelInterpretation")
            .as_::<ChannelInterpretation>()
    }

    pub fn set_channel_interpretation(&mut self, value: ChannelInterpretation) {
        self.inner.set("channelInterpretation", value);
    }
}
