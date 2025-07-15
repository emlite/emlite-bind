use super::*;




#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct AudioWorkletNode {
    inner: AudioNode,
}
impl FromVal for AudioWorkletNode {
    fn from_val(v: &emlite::Val) -> Self {
        AudioWorkletNode { inner: AudioNode::from_val(v) }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for AudioWorkletNode {
    type Target = AudioNode;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for AudioWorkletNode {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for AudioWorkletNode {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for AudioWorkletNode {
    fn as_mut(&mut self) -> &mut emlite::Val {
      &mut self.inner
  }
}
impl From<AudioWorkletNode> for emlite::Val {
    fn from(s: AudioWorkletNode) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}
jsbind::utils::impl_dyn_cast!(AudioWorkletNode);



impl AudioWorkletNode {
    pub fn new0(context: BaseAudioContext, name: DOMString) -> AudioWorkletNode {
        Self {
            inner: emlite::Val::global("AudioWorkletNode").new(&[context.into(), name.into()]).as_::<AudioNode>(),
        }
    }

    pub fn new1(context: BaseAudioContext, name: DOMString, options: Any) -> AudioWorkletNode {
        Self {
            inner: emlite::Val::global("AudioWorkletNode").new(&[context.into(), name.into(), options.into()]).as_::<AudioNode>(),
        }
    }

}
impl AudioWorkletNode {
    pub fn parameters(&self) -> AudioParamMap {
        self.inner.get("parameters").as_::<AudioParamMap>()
    }

}
impl AudioWorkletNode {
    pub fn port(&self) -> Any {
        self.inner.get("port").as_::<Any>()
    }

}
impl AudioWorkletNode {
    pub fn onprocessorerror(&self) -> Any {
        self.inner.get("onprocessorerror").as_::<Any>()
    }

    pub fn set_onprocessorerror(&mut self, value: Any) {
        self.inner.set("onprocessorerror", value);
    }

}
