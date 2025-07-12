use super::*;

#[derive(Clone, Debug)]
pub struct InputDeviceInfo {
    inner: MediaDeviceInfo,
}
impl FromVal for InputDeviceInfo {
    fn from_val(v: &emlite::Val) -> Self {
        InputDeviceInfo {
            inner: MediaDeviceInfo::from_val(v),
        }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl std::ops::Deref for InputDeviceInfo {
    type Target = MediaDeviceInfo;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for InputDeviceInfo {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<InputDeviceInfo> for emlite::Val {
    fn from(s: InputDeviceInfo) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl InputDeviceInfo {
    pub fn get_capabilities(&self) -> MediaTrackCapabilities {
        self.inner
            .call("getCapabilities", &[])
            .as_::<MediaTrackCapabilities>()
    }
}
