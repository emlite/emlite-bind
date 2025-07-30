use super::*;

/// The AudioWorkletNode class.
/// [`AudioWorkletNode`](https://developer.mozilla.org/en-US/docs/Web/API/AudioWorkletNode)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct AudioWorkletNode {
    inner: AudioNode,
}
impl FromVal for AudioWorkletNode {
    fn from_val(v: &Any) -> Self {
        AudioWorkletNode {
            inner: AudioNode::from_val(v),
        }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for AudioWorkletNode {
    type Target = AudioNode;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for AudioWorkletNode {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for AudioWorkletNode {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for AudioWorkletNode {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<AudioWorkletNode> for Any {
    fn from(s: AudioWorkletNode) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&AudioWorkletNode> for Any {
    fn from(s: &AudioWorkletNode) -> Any {
        s.inner.clone().into()
    }
}
jsbind::utils::impl_dyn_cast!(AudioWorkletNode);

impl AudioWorkletNode {
    /// The `new AudioWorkletNode(..)` constructor, creating a new AudioWorkletNode instance
    pub fn new0(context: &BaseAudioContext, name: &JsString) -> AudioWorkletNode {
        Self {
            inner: Any::global("AudioWorkletNode")
                .new(&[context.into(), name.into()])
                .as_::<AudioNode>(),
        }
    }

    /// The `new AudioWorkletNode(..)` constructor, creating a new AudioWorkletNode instance
    pub fn new1(context: &BaseAudioContext, name: &JsString, options: &Any) -> AudioWorkletNode {
        Self {
            inner: Any::global("AudioWorkletNode")
                .new(&[context.into(), name.into(), options.into()])
                .as_::<AudioNode>(),
        }
    }
}
impl AudioWorkletNode {
    /// Getter of the `parameters` attribute.
    /// [`AudioWorkletNode.parameters`](https://developer.mozilla.org/en-US/docs/Web/API/AudioWorkletNode/parameters)
    pub fn parameters(&self) -> AudioParamMap {
        self.inner.get("parameters").as_::<AudioParamMap>()
    }
}
impl AudioWorkletNode {
    /// Getter of the `port` attribute.
    /// [`AudioWorkletNode.port`](https://developer.mozilla.org/en-US/docs/Web/API/AudioWorkletNode/port)
    pub fn port(&self) -> Any {
        self.inner.get("port").as_::<Any>()
    }
}
impl AudioWorkletNode {
    /// Getter of the `onprocessorerror` attribute.
    /// [`AudioWorkletNode.onprocessorerror`](https://developer.mozilla.org/en-US/docs/Web/API/AudioWorkletNode/onprocessorerror)
    pub fn onprocessorerror(&self) -> Any {
        self.inner.get("onprocessorerror").as_::<Any>()
    }

    /// Setter of the `onprocessorerror` attribute.
    /// [`AudioWorkletNode.onprocessorerror`](https://developer.mozilla.org/en-US/docs/Web/API/AudioWorkletNode/onprocessorerror)
    pub fn set_onprocessorerror(&mut self, value: &Any) {
        self.inner.set("onprocessorerror", value);
    }
}
