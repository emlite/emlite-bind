use super::*;

#[derive(Clone, Debug)]
pub struct AudioDestinationNode {
    inner: AudioNode,
}
impl FromVal for AudioDestinationNode {
    fn from_val(v: &emlite::Val) -> Self {
        AudioDestinationNode {
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
impl std::ops::Deref for AudioDestinationNode {
    type Target = AudioNode;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for AudioDestinationNode {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<AudioDestinationNode> for emlite::Val {
    fn from(s: AudioDestinationNode) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl AudioDestinationNode {
    pub fn max_channel_count(&self) -> u32 {
        self.inner.get("maxChannelCount").as_::<u32>()
    }
}
