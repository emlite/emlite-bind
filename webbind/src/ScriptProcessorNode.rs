use super::*;

/// The ScriptProcessorNode class.
/// [`ScriptProcessorNode`](https://developer.mozilla.org/en-US/docs/Web/API/ScriptProcessorNode)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct ScriptProcessorNode {
    inner: AudioNode,
}

impl FromVal for ScriptProcessorNode {
    fn from_val(v: &Any) -> Self {
        ScriptProcessorNode {
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

impl core::ops::Deref for ScriptProcessorNode {
    type Target = AudioNode;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for ScriptProcessorNode {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for ScriptProcessorNode {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for ScriptProcessorNode {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}

impl From<ScriptProcessorNode> for Any {
    fn from(s: ScriptProcessorNode) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&ScriptProcessorNode> for Any {
    fn from(s: &ScriptProcessorNode) -> Any {
        s.inner.clone().into()
    }
}

jsbind::utils::impl_dyn_cast!(ScriptProcessorNode);

impl ScriptProcessorNode {
    /// Getter of the `onaudioprocess` attribute.
    /// [`ScriptProcessorNode.onaudioprocess`](https://developer.mozilla.org/en-US/docs/Web/API/ScriptProcessorNode/onaudioprocess)
    pub fn onaudioprocess(&self) -> Any {
        self.inner.get("onaudioprocess").as_::<Any>()
    }

    /// Setter of the `onaudioprocess` attribute.
    /// [`ScriptProcessorNode.onaudioprocess`](https://developer.mozilla.org/en-US/docs/Web/API/ScriptProcessorNode/onaudioprocess)
    pub fn set_onaudioprocess(&mut self, value: &Any) {
        self.inner.set("onaudioprocess", value);
    }
}
impl ScriptProcessorNode {
    /// Getter of the `bufferSize` attribute.
    /// [`ScriptProcessorNode.bufferSize`](https://developer.mozilla.org/en-US/docs/Web/API/ScriptProcessorNode/bufferSize)
    pub fn buffer_size(&self) -> i32 {
        self.inner.get("bufferSize").as_::<i32>()
    }
}
