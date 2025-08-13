use super::*;




/// The AudioSinkInfo class.
/// [`AudioSinkInfo`](https://developer.mozilla.org/en-US/docs/Web/API/AudioSinkInfo)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct AudioSinkInfo {
    inner: Any,
}

impl FromVal for AudioSinkInfo {
    fn from_val(v: &Any) -> Self {
        AudioSinkInfo { inner: Any::from_val(v) }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for AudioSinkInfo {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for AudioSinkInfo {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for AudioSinkInfo {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for AudioSinkInfo {
    fn as_mut(&mut self) -> &mut Any {
      &mut self.inner
  }
}

impl From<AudioSinkInfo> for Any {
    fn from(s: AudioSinkInfo) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&AudioSinkInfo> for Any {
    fn from(s: &AudioSinkInfo) -> Any {
        s.inner.clone().into()
    }
}

jsbind::utils::impl_dyn_cast!(AudioSinkInfo);


impl AudioSinkInfo {
    /// Getter of the `type` attribute.
    /// [`AudioSinkInfo.type`](https://developer.mozilla.org/en-US/docs/Web/API/AudioSinkInfo/type)
    pub fn type_(&self) -> AudioSinkType {
        self.inner.get("type").as_::<AudioSinkType>()
    }

}
