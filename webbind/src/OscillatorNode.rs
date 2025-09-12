use super::*;

/// The OscillatorNode class.
/// [`OscillatorNode`](https://developer.mozilla.org/en-US/docs/Web/API/OscillatorNode)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct OscillatorNode {
    inner: AudioScheduledSourceNode,
}

impl FromVal for OscillatorNode {
    fn from_val(v: &Any) -> Self {
        OscillatorNode {
            inner: AudioScheduledSourceNode::from_val(v),
        }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for OscillatorNode {
    type Target = AudioScheduledSourceNode;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for OscillatorNode {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for OscillatorNode {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for OscillatorNode {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}

impl From<OscillatorNode> for Any {
    fn from(s: OscillatorNode) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&OscillatorNode> for Any {
    fn from(s: &OscillatorNode) -> Any {
        s.inner.clone().into()
    }
}

jsbind::utils::impl_dyn_cast!(OscillatorNode);

impl OscillatorNode {
    /// Getter of the `type` attribute.
    /// [`OscillatorNode.type`](https://developer.mozilla.org/en-US/docs/Web/API/OscillatorNode/type)
    pub fn type_(&self) -> OscillatorType {
        self.inner.get("type").as_::<OscillatorType>()
    }

    /// Setter of the `type` attribute.
    /// [`OscillatorNode.type`](https://developer.mozilla.org/en-US/docs/Web/API/OscillatorNode/type)
    pub fn set_type_(&mut self, value: &OscillatorType) {
        self.inner.set("type", value);
    }
}
impl OscillatorNode {
    /// Getter of the `frequency` attribute.
    /// [`OscillatorNode.frequency`](https://developer.mozilla.org/en-US/docs/Web/API/OscillatorNode/frequency)
    pub fn frequency(&self) -> AudioParam {
        self.inner.get("frequency").as_::<AudioParam>()
    }
}
impl OscillatorNode {
    /// Getter of the `detune` attribute.
    /// [`OscillatorNode.detune`](https://developer.mozilla.org/en-US/docs/Web/API/OscillatorNode/detune)
    pub fn detune(&self) -> AudioParam {
        self.inner.get("detune").as_::<AudioParam>()
    }
}

impl OscillatorNode {
    /// The `new OscillatorNode(..)` constructor, creating a new OscillatorNode instance
    pub fn new(context: &BaseAudioContext) -> OscillatorNode {
        Self {
            inner: Any::global("OscillatorNode")
                .new(&[context.into()])
                .as_::<AudioScheduledSourceNode>(),
        }
    }
}

impl OscillatorNode {
    /// The `new OscillatorNode(..)` constructor, creating a new OscillatorNode instance
    pub fn new_with_options(
        context: &BaseAudioContext,
        options: &OscillatorOptions,
    ) -> OscillatorNode {
        Self {
            inner: Any::global("OscillatorNode")
                .new(&[context.into(), options.into()])
                .as_::<AudioScheduledSourceNode>(),
        }
    }
}

impl OscillatorNode {
    /// The setPeriodicWave method.
    /// [`OscillatorNode.setPeriodicWave`](https://developer.mozilla.org/en-US/docs/Web/API/OscillatorNode/setPeriodicWave)
    pub fn set_periodic_wave(&self, periodic_wave: &PeriodicWave) -> Undefined {
        self.inner
            .call("setPeriodicWave", &[periodic_wave.into()])
            .as_::<Undefined>()
    }
}
