use super::*;

/// The StereoPannerNode class.
/// [`StereoPannerNode`](https://developer.mozilla.org/en-US/docs/Web/API/StereoPannerNode)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct StereoPannerNode {
    inner: AudioNode,
}

impl FromVal for StereoPannerNode {
    fn from_val(v: &Any) -> Self {
        StereoPannerNode {
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

impl core::ops::Deref for StereoPannerNode {
    type Target = AudioNode;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for StereoPannerNode {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for StereoPannerNode {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for StereoPannerNode {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}

impl From<StereoPannerNode> for Any {
    fn from(s: StereoPannerNode) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&StereoPannerNode> for Any {
    fn from(s: &StereoPannerNode) -> Any {
        s.inner.clone().into()
    }
}

jsbind::utils::impl_dyn_cast!(StereoPannerNode);

impl StereoPannerNode {
    /// The `new StereoPannerNode(..)` constructor, creating a new StereoPannerNode instance
    pub fn new0(context: &BaseAudioContext) -> StereoPannerNode {
        Self {
            inner: Any::global("StereoPannerNode")
                .new(&[context.into()])
                .as_::<AudioNode>(),
        }
    }

    /// The `new StereoPannerNode(..)` constructor, creating a new StereoPannerNode instance
    pub fn new1(context: &BaseAudioContext, options: &StereoPannerOptions) -> StereoPannerNode {
        Self {
            inner: Any::global("StereoPannerNode")
                .new(&[context.into(), options.into()])
                .as_::<AudioNode>(),
        }
    }
}
impl StereoPannerNode {
    /// Getter of the `pan` attribute.
    /// [`StereoPannerNode.pan`](https://developer.mozilla.org/en-US/docs/Web/API/StereoPannerNode/pan)
    pub fn pan(&self) -> AudioParam {
        self.inner.get("pan").as_::<AudioParam>()
    }
}
