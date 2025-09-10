use super::*;

/// The GainNode class.
/// [`GainNode`](https://developer.mozilla.org/en-US/docs/Web/API/GainNode)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct GainNode {
    inner: AudioNode,
}

impl FromVal for GainNode {
    fn from_val(v: &Any) -> Self {
        GainNode {
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

impl core::ops::Deref for GainNode {
    type Target = AudioNode;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for GainNode {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for GainNode {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for GainNode {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}

impl From<GainNode> for Any {
    fn from(s: GainNode) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&GainNode> for Any {
    fn from(s: &GainNode) -> Any {
        s.inner.clone().into()
    }
}

jsbind::utils::impl_dyn_cast!(GainNode);

impl GainNode {
    /// The `new GainNode(..)` constructor, creating a new GainNode instance
    pub fn new0(context: &BaseAudioContext) -> GainNode {
        Self {
            inner: Any::global("GainNode")
                .new(&[context.into()])
                .as_::<AudioNode>(),
        }
    }

    /// The `new GainNode(..)` constructor, creating a new GainNode instance
    pub fn new1(context: &BaseAudioContext, options: &GainOptions) -> GainNode {
        Self {
            inner: Any::global("GainNode")
                .new(&[context.into(), options.into()])
                .as_::<AudioNode>(),
        }
    }
}
impl GainNode {
    /// Getter of the `gain` attribute.
    /// [`GainNode.gain`](https://developer.mozilla.org/en-US/docs/Web/API/GainNode/gain)
    pub fn gain(&self) -> AudioParam {
        self.inner.get("gain").as_::<AudioParam>()
    }
}
