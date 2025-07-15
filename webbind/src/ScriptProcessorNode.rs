use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct ScriptProcessorNode {
    inner: AudioNode,
}
impl FromVal for ScriptProcessorNode {
    fn from_val(v: &emlite::Val) -> Self {
        ScriptProcessorNode {
            inner: AudioNode::from_val(v),
        }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
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
impl AsRef<emlite::Val> for ScriptProcessorNode {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for ScriptProcessorNode {
    fn as_mut(&mut self) -> &mut emlite::Val {
        &mut self.inner
    }
}
impl From<ScriptProcessorNode> for emlite::Val {
    fn from(s: ScriptProcessorNode) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}
jsbind::utils::impl_dyn_cast!(ScriptProcessorNode);

impl ScriptProcessorNode {
    pub fn onaudioprocess(&self) -> Any {
        self.inner.get("onaudioprocess").as_::<Any>()
    }

    pub fn set_onaudioprocess(&mut self, value: Any) {
        self.inner.set("onaudioprocess", value);
    }
}
impl ScriptProcessorNode {
    pub fn buffer_size(&self) -> i32 {
        self.inner.get("bufferSize").as_::<i32>()
    }
}
