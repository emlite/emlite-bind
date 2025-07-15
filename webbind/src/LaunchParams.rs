use super::*;




#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct LaunchParams {
    inner: emlite::Val,
}
impl FromVal for LaunchParams {
    fn from_val(v: &emlite::Val) -> Self {
        LaunchParams { inner: emlite::Val::from_val(v) }
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
impl AsRef<emlite::Val> for LaunchParams {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for LaunchParams {
    fn as_mut(&mut self) -> &mut emlite::Val {
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
jsbind::utils::impl_dyn_cast!(LaunchParams);


impl LaunchParams {
    pub fn target_url(&self) -> DOMString {
        self.inner.get("targetURL").as_::<DOMString>()
    }

}
impl LaunchParams {
    pub fn files(&self) -> FrozenArray<FileSystemHandle> {
        self.inner.get("files").as_::<FrozenArray<FileSystemHandle>>()
    }

}
