use super::*;

/// The ConvolverNode class.
/// [`ConvolverNode`](https://developer.mozilla.org/en-US/docs/Web/API/ConvolverNode)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct ConvolverNode {
    inner: AudioNode,
}

impl FromVal for ConvolverNode {
    fn from_val(v: &Any) -> Self {
        ConvolverNode {
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

impl core::ops::Deref for ConvolverNode {
    type Target = AudioNode;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for ConvolverNode {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for ConvolverNode {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for ConvolverNode {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}

impl From<ConvolverNode> for Any {
    fn from(s: ConvolverNode) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&ConvolverNode> for Any {
    fn from(s: &ConvolverNode) -> Any {
        s.inner.clone().into()
    }
}

jsbind::utils::impl_dyn_cast!(ConvolverNode);

impl ConvolverNode {
    /// Getter of the `buffer` attribute.
    /// [`ConvolverNode.buffer`](https://developer.mozilla.org/en-US/docs/Web/API/ConvolverNode/buffer)
    pub fn buffer(&self) -> AudioBuffer {
        self.inner.get("buffer").as_::<AudioBuffer>()
    }

    /// Setter of the `buffer` attribute.
    /// [`ConvolverNode.buffer`](https://developer.mozilla.org/en-US/docs/Web/API/ConvolverNode/buffer)
    pub fn set_buffer(&mut self, value: &AudioBuffer) {
        self.inner.set("buffer", value);
    }
}
impl ConvolverNode {
    /// Getter of the `normalize` attribute.
    /// [`ConvolverNode.normalize`](https://developer.mozilla.org/en-US/docs/Web/API/ConvolverNode/normalize)
    pub fn normalize(&self) -> bool {
        self.inner.get("normalize").as_::<bool>()
    }

    /// Setter of the `normalize` attribute.
    /// [`ConvolverNode.normalize`](https://developer.mozilla.org/en-US/docs/Web/API/ConvolverNode/normalize)
    pub fn set_normalize(&mut self, value: bool) {
        self.inner.set("normalize", value);
    }
}

impl ConvolverNode {
    /// The `new ConvolverNode(..)` constructor, creating a new ConvolverNode instance
    pub fn new(context: &BaseAudioContext) -> ConvolverNode {
        Self {
            inner: Any::global("ConvolverNode")
                .new(&[context.into()])
                .as_::<AudioNode>(),
        }
    }
}

impl ConvolverNode {
    /// The `new ConvolverNode(..)` constructor, creating a new ConvolverNode instance
    pub fn new_with_options(
        context: &BaseAudioContext,
        options: &ConvolverOptions,
    ) -> ConvolverNode {
        Self {
            inner: Any::global("ConvolverNode")
                .new(&[context.into(), options.into()])
                .as_::<AudioNode>(),
        }
    }
}
