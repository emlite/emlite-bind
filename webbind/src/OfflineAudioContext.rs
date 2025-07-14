use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct OfflineAudioContext {
    inner: BaseAudioContext,
}
impl FromVal for OfflineAudioContext {
    fn from_val(v: &emlite::Val) -> Self {
        OfflineAudioContext {
            inner: BaseAudioContext::from_val(v),
        }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for OfflineAudioContext {
    type Target = BaseAudioContext;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for OfflineAudioContext {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for OfflineAudioContext {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for OfflineAudioContext {
    fn as_mut(&mut self) -> &mut emlite::Val {
        &mut self.inner
    }
}
impl From<OfflineAudioContext> for emlite::Val {
    fn from(s: OfflineAudioContext) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}
jsbind::utils::impl_dyn_cast!(OfflineAudioContext);

impl OfflineAudioContext {
    pub fn new(number_of_channels: u32, length: u32, sample_rate: f32) -> OfflineAudioContext {
        Self {
            inner: emlite::Val::global("OfflineAudioContext")
                .new(&[number_of_channels.into(), length.into(), sample_rate.into()])
                .as_::<BaseAudioContext>(),
        }
    }
}
impl OfflineAudioContext {
    pub fn start_rendering(&self) -> jsbind::Promise {
        self.inner
            .call("startRendering", &[])
            .as_::<jsbind::Promise>()
    }
}
impl OfflineAudioContext {
    pub fn resume(&self) -> jsbind::Promise {
        self.inner.call("resume", &[]).as_::<jsbind::Promise>()
    }
}
impl OfflineAudioContext {
    pub fn suspend(&self, suspend_time: f64) -> jsbind::Promise {
        self.inner
            .call("suspend", &[suspend_time.into()])
            .as_::<jsbind::Promise>()
    }
}
impl OfflineAudioContext {
    pub fn length(&self) -> u32 {
        self.inner.get("length").as_::<u32>()
    }
}
impl OfflineAudioContext {
    pub fn oncomplete(&self) -> jsbind::Any {
        self.inner.get("oncomplete").as_::<jsbind::Any>()
    }

    pub fn set_oncomplete(&mut self, value: jsbind::Any) {
        self.inner.set("oncomplete", value);
    }
}
