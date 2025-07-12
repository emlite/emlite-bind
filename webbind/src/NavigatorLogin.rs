use super::*;

#[derive(Clone, Debug)]
pub struct NavigatorLogin {
    inner: emlite::Val,
}
impl FromVal for NavigatorLogin {
    fn from_val(v: &emlite::Val) -> Self {
        NavigatorLogin {
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
impl std::ops::Deref for NavigatorLogin {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for NavigatorLogin {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<NavigatorLogin> for emlite::Val {
    fn from(s: NavigatorLogin) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl NavigatorLogin {
    pub fn set_status(&self, status: LoginStatus) -> jsbind::Promise {
        self.inner
            .call("setStatus", &[status.into()])
            .as_::<jsbind::Promise>()
    }
}
