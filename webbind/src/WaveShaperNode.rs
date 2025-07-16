use super::*;

/// The WaveShaperNode class.
/// [`WaveShaperNode`](https://developer.mozilla.org/en-US/docs/Web/API/WaveShaperNode)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct WaveShaperNode {
    inner: AudioNode,
}
impl FromVal for WaveShaperNode {
    fn from_val(v: &Any) -> Self {
        WaveShaperNode {
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
impl core::ops::Deref for WaveShaperNode {
    type Target = AudioNode;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for WaveShaperNode {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for WaveShaperNode {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for WaveShaperNode {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<WaveShaperNode> for Any {
    fn from(s: WaveShaperNode) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&WaveShaperNode> for Any {
    fn from(s: &WaveShaperNode) -> Any {
        s.inner.clone().into()
    }
}
jsbind::utils::impl_dyn_cast!(WaveShaperNode);

impl WaveShaperNode {
    /// The `new WaveShaperNode(..)` constructor, creating a new WaveShaperNode instance
    pub fn new0(context: &BaseAudioContext) -> WaveShaperNode {
        Self {
            inner: Any::global("WaveShaperNode")
                .new(&[context.into()])
                .as_::<AudioNode>(),
        }
    }

    /// The `new WaveShaperNode(..)` constructor, creating a new WaveShaperNode instance
    pub fn new1(context: &BaseAudioContext, options: &Any) -> WaveShaperNode {
        Self {
            inner: Any::global("WaveShaperNode")
                .new(&[context.into(), options.into()])
                .as_::<AudioNode>(),
        }
    }
}
impl WaveShaperNode {
    /// Getter of the `curve` attribute.
    /// [`WaveShaperNode.curve`](https://developer.mozilla.org/en-US/docs/Web/API/WaveShaperNode/curve)
    pub fn curve(&self) -> Float32Array {
        self.inner.get("curve").as_::<Float32Array>()
    }

    /// Setter of the `curve` attribute.
    /// [`WaveShaperNode.curve`](https://developer.mozilla.org/en-US/docs/Web/API/WaveShaperNode/curve)
    pub fn set_curve(&mut self, value: &Float32Array) {
        self.inner.set("curve", value);
    }
}
impl WaveShaperNode {
    /// Getter of the `oversample` attribute.
    /// [`WaveShaperNode.oversample`](https://developer.mozilla.org/en-US/docs/Web/API/WaveShaperNode/oversample)
    pub fn oversample(&self) -> OverSampleType {
        self.inner.get("oversample").as_::<OverSampleType>()
    }

    /// Setter of the `oversample` attribute.
    /// [`WaveShaperNode.oversample`](https://developer.mozilla.org/en-US/docs/Web/API/WaveShaperNode/oversample)
    pub fn set_oversample(&mut self, value: &OverSampleType) {
        self.inner.set("oversample", value);
    }
}
