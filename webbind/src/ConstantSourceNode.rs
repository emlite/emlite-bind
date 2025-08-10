use super::*;

/// The ConstantSourceNode class.
/// [`ConstantSourceNode`](https://developer.mozilla.org/en-US/docs/Web/API/ConstantSourceNode)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct ConstantSourceNode {
    inner: AudioScheduledSourceNode,
}
impl FromVal for ConstantSourceNode {
    fn from_val(v: &Any) -> Self {
        ConstantSourceNode {
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
impl core::ops::Deref for ConstantSourceNode {
    type Target = AudioScheduledSourceNode;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for ConstantSourceNode {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for ConstantSourceNode {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for ConstantSourceNode {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<ConstantSourceNode> for Any {
    fn from(s: ConstantSourceNode) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&ConstantSourceNode> for Any {
    fn from(s: &ConstantSourceNode) -> Any {
        s.inner.clone().into()
    }
}
jsbind::utils::impl_dyn_cast!(ConstantSourceNode);

impl ConstantSourceNode {
    /// The `new ConstantSourceNode(..)` constructor, creating a new ConstantSourceNode instance
    pub fn new0(context: &BaseAudioContext) -> ConstantSourceNode {
        Self {
            inner: Any::global("ConstantSourceNode")
                .new(&[context.into()])
                .as_::<AudioScheduledSourceNode>(),
        }
    }

    /// The `new ConstantSourceNode(..)` constructor, creating a new ConstantSourceNode instance
    pub fn new1(context: &BaseAudioContext, options: &ConstantSourceOptions) -> ConstantSourceNode {
        Self {
            inner: Any::global("ConstantSourceNode")
                .new(&[context.into(), options.into()])
                .as_::<AudioScheduledSourceNode>(),
        }
    }
}
impl ConstantSourceNode {
    /// Getter of the `offset` attribute.
    /// [`ConstantSourceNode.offset`](https://developer.mozilla.org/en-US/docs/Web/API/ConstantSourceNode/offset)
    pub fn offset(&self) -> AudioParam {
        self.inner.get("offset").as_::<AudioParam>()
    }
}
