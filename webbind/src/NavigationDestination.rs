use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct NavigationDestination {
    inner: emlite::Val,
}
impl FromVal for NavigationDestination {
    fn from_val(v: &emlite::Val) -> Self {
        NavigationDestination {
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
impl core::ops::Deref for NavigationDestination {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for NavigationDestination {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for NavigationDestination {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for NavigationDestination {
    fn as_mut(&mut self) -> &mut emlite::Val {
        &mut self.inner
    }
}
impl From<NavigationDestination> for emlite::Val {
    fn from(s: NavigationDestination) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}
impl From<&NavigationDestination> for emlite::Val {
    fn from(s: &NavigationDestination) -> emlite::Val {
        s.inner.clone().into()
    }
}
jsbind::utils::impl_dyn_cast!(NavigationDestination);

impl NavigationDestination {
    pub fn url(&self) -> String {
        self.inner.get("url").as_::<String>()
    }
}
impl NavigationDestination {
    pub fn key(&self) -> String {
        self.inner.get("key").as_::<String>()
    }
}
impl NavigationDestination {
    pub fn id(&self) -> String {
        self.inner.get("id").as_::<String>()
    }
}
impl NavigationDestination {
    pub fn index(&self) -> i64 {
        self.inner.get("index").as_::<i64>()
    }
}
impl NavigationDestination {
    pub fn same_document(&self) -> bool {
        self.inner.get("sameDocument").as_::<bool>()
    }
}
impl NavigationDestination {
    pub fn get_state(&self) -> Any {
        self.inner.call("getState", &[]).as_::<Any>()
    }
}
