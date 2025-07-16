use super::*;

/// The AudioParamMap class.
/// [`AudioParamMap`](https://developer.mozilla.org/en-US/docs/Web/API/AudioParamMap)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct AudioParamMap {
    inner: Any,
}
impl FromVal for AudioParamMap {
    fn from_val(v: &Any) -> Self {
        AudioParamMap {
            inner: Any::from_val(v),
        }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for AudioParamMap {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for AudioParamMap {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for AudioParamMap {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for AudioParamMap {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<AudioParamMap> for Any {
    fn from(s: AudioParamMap) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&AudioParamMap> for Any {
    fn from(s: &AudioParamMap) -> Any {
        s.inner.clone().into()
    }
}
jsbind::utils::impl_dyn_cast!(AudioParamMap);
