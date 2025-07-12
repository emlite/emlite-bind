use super::*;

#[derive(Clone, Debug)]
pub struct External {
    inner: emlite::Val,
}
impl FromVal for External {
    fn from_val(v: &emlite::Val) -> Self {
        External {
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
impl std::ops::Deref for External {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for External {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<External> for emlite::Val {
    fn from(s: External) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl External {
    pub fn add_search_provider(&self) -> jsbind::Undefined {
        self.inner
            .call("AddSearchProvider", &[])
            .as_::<jsbind::Undefined>()
    }
}
impl External {
    pub fn is_search_provider_installed(&self) -> jsbind::Undefined {
        self.inner
            .call("IsSearchProviderInstalled", &[])
            .as_::<jsbind::Undefined>()
    }
}
