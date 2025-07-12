use super::*;

#[derive(Clone, Debug)]
pub struct Permissions {
    inner: emlite::Val,
}
impl FromVal for Permissions {
    fn from_val(v: &emlite::Val) -> Self {
        Permissions {
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
impl std::ops::Deref for Permissions {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for Permissions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<Permissions> for emlite::Val {
    fn from(s: Permissions) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl Permissions {
    pub fn query(&self, permission_desc: jsbind::Object) -> jsbind::Promise {
        self.inner
            .call("query", &[permission_desc.into()])
            .as_::<jsbind::Promise>()
    }
}
impl Permissions {
    pub fn request(&self, permission_desc: jsbind::Object) -> jsbind::Promise {
        self.inner
            .call("request", &[permission_desc.into()])
            .as_::<jsbind::Promise>()
    }
}
impl Permissions {
    pub fn revoke(&self, permission_desc: jsbind::Object) -> jsbind::Promise {
        self.inner
            .call("revoke", &[permission_desc.into()])
            .as_::<jsbind::Promise>()
    }
}
