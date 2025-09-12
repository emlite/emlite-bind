use super::*;

/// The DelayNode class.
/// [`DelayNode`](https://developer.mozilla.org/en-US/docs/Web/API/DelayNode)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct DelayNode {
    inner: AudioNode,
}

impl FromVal for DelayNode {
    fn from_val(v: &Any) -> Self {
        DelayNode {
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

impl core::ops::Deref for DelayNode {
    type Target = AudioNode;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for DelayNode {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for DelayNode {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for DelayNode {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}

impl From<DelayNode> for Any {
    fn from(s: DelayNode) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&DelayNode> for Any {
    fn from(s: &DelayNode) -> Any {
        s.inner.clone().into()
    }
}

jsbind::utils::impl_dyn_cast!(DelayNode);

impl DelayNode {
    /// Getter of the `delayTime` attribute.
    /// [`DelayNode.delayTime`](https://developer.mozilla.org/en-US/docs/Web/API/DelayNode/delayTime)
    pub fn delay_time(&self) -> AudioParam {
        self.inner.get("delayTime").as_::<AudioParam>()
    }
}

impl DelayNode {
    /// The `new DelayNode(..)` constructor, creating a new DelayNode instance
    pub fn new0(context: &BaseAudioContext) -> DelayNode {
        Self {
            inner: Any::global("DelayNode")
                .new(&[context.into()])
                .as_::<AudioNode>(),
        }
    }

    /// The `new DelayNode(..)` constructor, creating a new DelayNode instance
    pub fn new1(context: &BaseAudioContext, options: &DelayOptions) -> DelayNode {
        Self {
            inner: Any::global("DelayNode")
                .new(&[context.into(), options.into()])
                .as_::<AudioNode>(),
        }
    }
}
