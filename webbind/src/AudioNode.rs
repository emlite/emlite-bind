use super::*;

/// The AudioNode class.
/// [`AudioNode`](https://developer.mozilla.org/en-US/docs/Web/API/AudioNode)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct AudioNode {
    inner: EventTarget,
}

impl FromVal for AudioNode {
    fn from_val(v: &Any) -> Self {
        AudioNode {
            inner: EventTarget::from_val(v),
        }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
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

impl AsRef<Any> for AudioNode {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for AudioNode {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}

impl From<AudioNode> for Any {
    fn from(s: AudioNode) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&AudioNode> for Any {
    fn from(s: &AudioNode) -> Any {
        s.inner.clone().into()
    }
}

jsbind::utils::impl_dyn_cast!(AudioNode);

impl AudioNode {
    /// Getter of the `context` attribute.
    /// [`AudioNode.context`](https://developer.mozilla.org/en-US/docs/Web/API/AudioNode/context)
    pub fn context(&self) -> BaseAudioContext {
        self.inner.get("context").as_::<BaseAudioContext>()
    }
}
impl AudioNode {
    /// Getter of the `numberOfInputs` attribute.
    /// [`AudioNode.numberOfInputs`](https://developer.mozilla.org/en-US/docs/Web/API/AudioNode/numberOfInputs)
    pub fn number_of_inputs(&self) -> u32 {
        self.inner.get("numberOfInputs").as_::<u32>()
    }
}
impl AudioNode {
    /// Getter of the `numberOfOutputs` attribute.
    /// [`AudioNode.numberOfOutputs`](https://developer.mozilla.org/en-US/docs/Web/API/AudioNode/numberOfOutputs)
    pub fn number_of_outputs(&self) -> u32 {
        self.inner.get("numberOfOutputs").as_::<u32>()
    }
}
impl AudioNode {
    /// Getter of the `channelCount` attribute.
    /// [`AudioNode.channelCount`](https://developer.mozilla.org/en-US/docs/Web/API/AudioNode/channelCount)
    pub fn channel_count(&self) -> u32 {
        self.inner.get("channelCount").as_::<u32>()
    }

    /// Setter of the `channelCount` attribute.
    /// [`AudioNode.channelCount`](https://developer.mozilla.org/en-US/docs/Web/API/AudioNode/channelCount)
    pub fn set_channel_count(&mut self, value: u32) {
        self.inner.set("channelCount", value);
    }
}
impl AudioNode {
    /// Getter of the `channelCountMode` attribute.
    /// [`AudioNode.channelCountMode`](https://developer.mozilla.org/en-US/docs/Web/API/AudioNode/channelCountMode)
    pub fn channel_count_mode(&self) -> ChannelCountMode {
        self.inner.get("channelCountMode").as_::<ChannelCountMode>()
    }

    /// Setter of the `channelCountMode` attribute.
    /// [`AudioNode.channelCountMode`](https://developer.mozilla.org/en-US/docs/Web/API/AudioNode/channelCountMode)
    pub fn set_channel_count_mode(&mut self, value: &ChannelCountMode) {
        self.inner.set("channelCountMode", value);
    }
}
impl AudioNode {
    /// Getter of the `channelInterpretation` attribute.
    /// [`AudioNode.channelInterpretation`](https://developer.mozilla.org/en-US/docs/Web/API/AudioNode/channelInterpretation)
    pub fn channel_interpretation(&self) -> ChannelInterpretation {
        self.inner
            .get("channelInterpretation")
            .as_::<ChannelInterpretation>()
    }

    /// Setter of the `channelInterpretation` attribute.
    /// [`AudioNode.channelInterpretation`](https://developer.mozilla.org/en-US/docs/Web/API/AudioNode/channelInterpretation)
    pub fn set_channel_interpretation(&mut self, value: &ChannelInterpretation) {
        self.inner.set("channelInterpretation", value);
    }
}
impl AudioNode {
    /// The connect method.
    /// [`AudioNode.connect`](https://developer.mozilla.org/en-US/docs/Web/API/AudioNode/connect)
    pub fn connect(&self, destination_node: &AudioNode) -> AudioNode {
        self.inner
            .call("connect", &[destination_node.into()])
            .as_::<AudioNode>()
    }
}
impl AudioNode {
    /// The connect method.
    /// [`AudioNode.connect`](https://developer.mozilla.org/en-US/docs/Web/API/AudioNode/connect)
    pub fn connect_with_output(&self, destination_node: &AudioNode, output: u32) -> AudioNode {
        self.inner
            .call("connect", &[destination_node.into(), output.into()])
            .as_::<AudioNode>()
    }
}
impl AudioNode {
    /// The connect method.
    /// [`AudioNode.connect`](https://developer.mozilla.org/en-US/docs/Web/API/AudioNode/connect)
    pub fn connect_with_output_and_input(
        &self,
        destination_node: &AudioNode,
        output: u32,
        input: u32,
    ) -> AudioNode {
        self.inner
            .call(
                "connect",
                &[destination_node.into(), output.into(), input.into()],
            )
            .as_::<AudioNode>()
    }
}
impl AudioNode {
    /// The connect method.
    /// [`AudioNode.connect`](https://developer.mozilla.org/en-US/docs/Web/API/AudioNode/connect)
    pub fn connect_with_destination_param(&self, destination_param: &AudioParam) -> Undefined {
        self.inner
            .call("connect", &[destination_param.into()])
            .as_::<Undefined>()
    }
}
impl AudioNode {
    /// The connect method.
    /// [`AudioNode.connect`](https://developer.mozilla.org/en-US/docs/Web/API/AudioNode/connect)
    pub fn connect_with_output_2(&self, destination_param: &AudioParam, output: u32) -> Undefined {
        self.inner
            .call("connect", &[destination_param.into(), output.into()])
            .as_::<Undefined>()
    }
}
impl AudioNode {
    /// The disconnect method.
    /// [`AudioNode.disconnect`](https://developer.mozilla.org/en-US/docs/Web/API/AudioNode/disconnect)
    pub fn disconnect(&self) -> Undefined {
        self.inner.call("disconnect", &[]).as_::<Undefined>()
    }
}
impl AudioNode {
    /// The disconnect method.
    /// [`AudioNode.disconnect`](https://developer.mozilla.org/en-US/docs/Web/API/AudioNode/disconnect)
    pub fn disconnect_with_output(&self, output: u32) -> Undefined {
        self.inner
            .call("disconnect", &[output.into()])
            .as_::<Undefined>()
    }
}
impl AudioNode {
    /// The disconnect method.
    /// [`AudioNode.disconnect`](https://developer.mozilla.org/en-US/docs/Web/API/AudioNode/disconnect)
    pub fn disconnect_with_destination_node(&self, destination_node: &AudioNode) -> Undefined {
        self.inner
            .call("disconnect", &[destination_node.into()])
            .as_::<Undefined>()
    }
}
impl AudioNode {
    /// The disconnect method.
    /// [`AudioNode.disconnect`](https://developer.mozilla.org/en-US/docs/Web/API/AudioNode/disconnect)
    pub fn disconnect_with_destination_node_and_output(
        &self,
        destination_node: &AudioNode,
        output: u32,
    ) -> Undefined {
        self.inner
            .call("disconnect", &[destination_node.into(), output.into()])
            .as_::<Undefined>()
    }
}
impl AudioNode {
    /// The disconnect method.
    /// [`AudioNode.disconnect`](https://developer.mozilla.org/en-US/docs/Web/API/AudioNode/disconnect)
    pub fn disconnect_with_destination_node_and_output_and_input(
        &self,
        destination_node: &AudioNode,
        output: u32,
        input: u32,
    ) -> Undefined {
        self.inner
            .call(
                "disconnect",
                &[destination_node.into(), output.into(), input.into()],
            )
            .as_::<Undefined>()
    }
}
impl AudioNode {
    /// The disconnect method.
    /// [`AudioNode.disconnect`](https://developer.mozilla.org/en-US/docs/Web/API/AudioNode/disconnect)
    pub fn disconnect_with_destination_param(&self, destination_param: &AudioParam) -> Undefined {
        self.inner
            .call("disconnect", &[destination_param.into()])
            .as_::<Undefined>()
    }
}
impl AudioNode {
    /// The disconnect method.
    /// [`AudioNode.disconnect`](https://developer.mozilla.org/en-US/docs/Web/API/AudioNode/disconnect)
    pub fn disconnect_with_destination_param_and_output(
        &self,
        destination_param: &AudioParam,
        output: u32,
    ) -> Undefined {
        self.inner
            .call("disconnect", &[destination_param.into(), output.into()])
            .as_::<Undefined>()
    }
}
