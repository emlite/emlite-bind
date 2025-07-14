use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct MediaSourceHandle {
    inner: emlite::Val,
}
impl FromVal for MediaSourceHandle {
    fn from_val(v: &emlite::Val) -> Self {
        MediaSourceHandle {
            inner: emlite::Val::from_val(v),
        }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for MediaSourceHandle {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for MediaSourceHandle {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<MediaSourceHandle> for emlite::Val {
    fn from(s: MediaSourceHandle) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}
