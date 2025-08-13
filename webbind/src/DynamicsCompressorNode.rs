use super::*;




/// The DynamicsCompressorNode class.
/// [`DynamicsCompressorNode`](https://developer.mozilla.org/en-US/docs/Web/API/DynamicsCompressorNode)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct DynamicsCompressorNode {
    inner: AudioNode,
}

impl FromVal for DynamicsCompressorNode {
    fn from_val(v: &Any) -> Self {
        DynamicsCompressorNode { inner: AudioNode::from_val(v) }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for DynamicsCompressorNode {
    type Target = AudioNode;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for DynamicsCompressorNode {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for DynamicsCompressorNode {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for DynamicsCompressorNode {
    fn as_mut(&mut self) -> &mut Any {
      &mut self.inner
  }
}

impl From<DynamicsCompressorNode> for Any {
    fn from(s: DynamicsCompressorNode) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&DynamicsCompressorNode> for Any {
    fn from(s: &DynamicsCompressorNode) -> Any {
        s.inner.clone().into()
    }
}

jsbind::utils::impl_dyn_cast!(DynamicsCompressorNode);



impl DynamicsCompressorNode {
    /// The `new DynamicsCompressorNode(..)` constructor, creating a new DynamicsCompressorNode instance
    pub fn new0(context: &BaseAudioContext) -> DynamicsCompressorNode {
        Self {
            inner: Any::global("DynamicsCompressorNode").new(&[context.into()]).as_::<AudioNode>(),
        }
    }

    /// The `new DynamicsCompressorNode(..)` constructor, creating a new DynamicsCompressorNode instance
    pub fn new1(context: &BaseAudioContext, options: &DynamicsCompressorOptions) -> DynamicsCompressorNode {
        Self {
            inner: Any::global("DynamicsCompressorNode").new(&[context.into(), options.into()]).as_::<AudioNode>(),
        }
    }

}
impl DynamicsCompressorNode {
    /// Getter of the `threshold` attribute.
    /// [`DynamicsCompressorNode.threshold`](https://developer.mozilla.org/en-US/docs/Web/API/DynamicsCompressorNode/threshold)
    pub fn threshold(&self) -> AudioParam {
        self.inner.get("threshold").as_::<AudioParam>()
    }

}
impl DynamicsCompressorNode {
    /// Getter of the `knee` attribute.
    /// [`DynamicsCompressorNode.knee`](https://developer.mozilla.org/en-US/docs/Web/API/DynamicsCompressorNode/knee)
    pub fn knee(&self) -> AudioParam {
        self.inner.get("knee").as_::<AudioParam>()
    }

}
impl DynamicsCompressorNode {
    /// Getter of the `ratio` attribute.
    /// [`DynamicsCompressorNode.ratio`](https://developer.mozilla.org/en-US/docs/Web/API/DynamicsCompressorNode/ratio)
    pub fn ratio(&self) -> AudioParam {
        self.inner.get("ratio").as_::<AudioParam>()
    }

}
impl DynamicsCompressorNode {
    /// Getter of the `reduction` attribute.
    /// [`DynamicsCompressorNode.reduction`](https://developer.mozilla.org/en-US/docs/Web/API/DynamicsCompressorNode/reduction)
    pub fn reduction(&self) -> f32 {
        self.inner.get("reduction").as_::<f32>()
    }

}
impl DynamicsCompressorNode {
    /// Getter of the `attack` attribute.
    /// [`DynamicsCompressorNode.attack`](https://developer.mozilla.org/en-US/docs/Web/API/DynamicsCompressorNode/attack)
    pub fn attack(&self) -> AudioParam {
        self.inner.get("attack").as_::<AudioParam>()
    }

}
impl DynamicsCompressorNode {
    /// Getter of the `release` attribute.
    /// [`DynamicsCompressorNode.release`](https://developer.mozilla.org/en-US/docs/Web/API/DynamicsCompressorNode/release)
    pub fn release(&self) -> AudioParam {
        self.inner.get("release").as_::<AudioParam>()
    }

}
