use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct ManagedMediaSource {
    inner: MediaSource,
}
impl FromVal for ManagedMediaSource {
    fn from_val(v: &emlite::Val) -> Self {
        ManagedMediaSource {
            inner: MediaSource::from_val(v),
        }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for ManagedMediaSource {
    type Target = MediaSource;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for ManagedMediaSource {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for ManagedMediaSource {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for ManagedMediaSource {
    fn as_mut(&mut self) -> &mut emlite::Val {
        &mut self.inner
    }
}
impl From<ManagedMediaSource> for emlite::Val {
    fn from(s: ManagedMediaSource) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}
jsbind::utils::impl_dyn_cast!(ManagedMediaSource);

impl ManagedMediaSource {
    pub fn new() -> ManagedMediaSource {
        Self {
            inner: emlite::Val::global("ManagedMediaSource")
                .new(&[])
                .as_::<MediaSource>(),
        }
    }
}
impl ManagedMediaSource {
    pub fn streaming(&self) -> bool {
        self.inner.get("streaming").as_::<bool>()
    }
}
impl ManagedMediaSource {
    pub fn onstartstreaming(&self) -> Any {
        self.inner.get("onstartstreaming").as_::<Any>()
    }

    pub fn set_onstartstreaming(&mut self, value: Any) {
        self.inner.set("onstartstreaming", value);
    }
}
impl ManagedMediaSource {
    pub fn onendstreaming(&self) -> Any {
        self.inner.get("onendstreaming").as_::<Any>()
    }

    pub fn set_onendstreaming(&mut self, value: Any) {
        self.inner.set("onendstreaming", value);
    }
}
