use super::*;




/// The InputDeviceInfo class.
/// [`InputDeviceInfo`](https://developer.mozilla.org/en-US/docs/Web/API/InputDeviceInfo)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct InputDeviceInfo {
    inner: MediaDeviceInfo,
}

impl FromVal for InputDeviceInfo {
    fn from_val(v: &Any) -> Self {
        InputDeviceInfo { inner: MediaDeviceInfo::from_val(v) }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for InputDeviceInfo {
    type Target = MediaDeviceInfo;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for InputDeviceInfo {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for InputDeviceInfo {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for InputDeviceInfo {
    fn as_mut(&mut self) -> &mut Any {
      &mut self.inner
  }
}

impl From<InputDeviceInfo> for Any {
    fn from(s: InputDeviceInfo) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&InputDeviceInfo> for Any {
    fn from(s: &InputDeviceInfo) -> Any {
        s.inner.clone().into()
    }
}

jsbind::utils::impl_dyn_cast!(InputDeviceInfo);


impl InputDeviceInfo {
    /// The getCapabilities method.
    /// [`InputDeviceInfo.getCapabilities`](https://developer.mozilla.org/en-US/docs/Web/API/InputDeviceInfo/getCapabilities)
    pub fn get_capabilities(&self, ) -> MediaTrackCapabilities {
        self.inner.call("getCapabilities", &[]).as_::<MediaTrackCapabilities>()
    }
}
