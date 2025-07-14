use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct LaunchParams {
    inner: emlite::Val,
}
impl FromVal for LaunchParams {
    fn from_val(v: &emlite::Val) -> Self {
        LaunchParams {
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
impl core::ops::Deref for LaunchParams {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for LaunchParams {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<LaunchParams> for emlite::Val {
    fn from(s: LaunchParams) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl LaunchParams {
    pub fn target_url(&self) -> jsbind::DOMString {
        self.inner.get("targetURL").as_::<jsbind::DOMString>()
    }
}
impl LaunchParams {
    pub fn files(&self) -> jsbind::FrozenArray<FileSystemHandle> {
        self.inner
            .get("files")
            .as_::<jsbind::FrozenArray<FileSystemHandle>>()
    }
}
