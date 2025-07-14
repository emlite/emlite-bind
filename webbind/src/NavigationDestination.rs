use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
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
impl From<NavigationDestination> for emlite::Val {
    fn from(s: NavigationDestination) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl NavigationDestination {
    pub fn url(&self) -> jsbind::USVString {
        self.inner.get("url").as_::<jsbind::USVString>()
    }
}
impl NavigationDestination {
    pub fn key(&self) -> jsbind::DOMString {
        self.inner.get("key").as_::<jsbind::DOMString>()
    }
}
impl NavigationDestination {
    pub fn id(&self) -> jsbind::DOMString {
        self.inner.get("id").as_::<jsbind::DOMString>()
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
    pub fn get_state(&self) -> jsbind::Any {
        self.inner.call("getState", &[]).as_::<jsbind::Any>()
    }
}
