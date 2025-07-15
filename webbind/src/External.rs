use super::*;




#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct External {
    inner: emlite::Val,
}
impl FromVal for External {
    fn from_val(v: &emlite::Val) -> Self {
        External { inner: emlite::Val::from_val(v) }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for External {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for External {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for External {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for External {
    fn as_mut(&mut self) -> &mut emlite::Val {
      &mut self.inner
  }
}
impl From<External> for emlite::Val {
    fn from(s: External) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}
jsbind::utils::impl_dyn_cast!(External);


impl External {
    pub fn add_search_provider(&self, ) -> Undefined {
        self.inner.call("AddSearchProvider", &[]).as_::<Undefined>()
    }

}
impl External {
    pub fn is_search_provider_installed(&self, ) -> Undefined {
        self.inner.call("IsSearchProviderInstalled", &[]).as_::<Undefined>()
    }

}
