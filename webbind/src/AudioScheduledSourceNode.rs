use super::*;

/// The AudioScheduledSourceNode class.
/// [`AudioScheduledSourceNode`](https://developer.mozilla.org/en-US/docs/Web/API/AudioScheduledSourceNode)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct AudioScheduledSourceNode {
    inner: AudioNode,
}
impl FromVal for AudioScheduledSourceNode {
    fn from_val(v: &Any) -> Self {
        AudioScheduledSourceNode {
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
impl core::ops::Deref for AudioScheduledSourceNode {
    type Target = AudioNode;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for AudioScheduledSourceNode {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for AudioScheduledSourceNode {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for AudioScheduledSourceNode {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<AudioScheduledSourceNode> for Any {
    fn from(s: AudioScheduledSourceNode) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&AudioScheduledSourceNode> for Any {
    fn from(s: &AudioScheduledSourceNode) -> Any {
        s.inner.clone().into()
    }
}
jsbind::utils::impl_dyn_cast!(AudioScheduledSourceNode);

impl AudioScheduledSourceNode {
    /// Getter of the `onended` attribute.
    /// [`AudioScheduledSourceNode.onended`](https://developer.mozilla.org/en-US/docs/Web/API/AudioScheduledSourceNode/onended)
    pub fn onended(&self) -> Any {
        self.inner.get("onended").as_::<Any>()
    }

    /// Setter of the `onended` attribute.
    /// [`AudioScheduledSourceNode.onended`](https://developer.mozilla.org/en-US/docs/Web/API/AudioScheduledSourceNode/onended)
    pub fn set_onended(&mut self, value: &Any) {
        self.inner.set("onended", value);
    }
}
impl AudioScheduledSourceNode {
    /// The start method.
    /// [`AudioScheduledSourceNode.start`](https://developer.mozilla.org/en-US/docs/Web/API/AudioScheduledSourceNode/start)
    pub fn start0(&self) -> Undefined {
        self.inner.call("start", &[]).as_::<Undefined>()
    }
    /// The start method.
    /// [`AudioScheduledSourceNode.start`](https://developer.mozilla.org/en-US/docs/Web/API/AudioScheduledSourceNode/start)
    pub fn start1(&self, when: f64) -> Undefined {
        self.inner.call("start", &[when.into()]).as_::<Undefined>()
    }
}
impl AudioScheduledSourceNode {
    /// The stop method.
    /// [`AudioScheduledSourceNode.stop`](https://developer.mozilla.org/en-US/docs/Web/API/AudioScheduledSourceNode/stop)
    pub fn stop0(&self) -> Undefined {
        self.inner.call("stop", &[]).as_::<Undefined>()
    }
    /// The stop method.
    /// [`AudioScheduledSourceNode.stop`](https://developer.mozilla.org/en-US/docs/Web/API/AudioScheduledSourceNode/stop)
    pub fn stop1(&self, when: f64) -> Undefined {
        self.inner.call("stop", &[when.into()]).as_::<Undefined>()
    }
}
