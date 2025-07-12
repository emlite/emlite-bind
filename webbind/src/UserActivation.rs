use super::*;

#[derive(Clone, Debug)]
pub struct UserActivation {
    inner: emlite::Val,
}
impl FromVal for UserActivation {
    fn from_val(v: &emlite::Val) -> Self {
        UserActivation {
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
impl std::ops::Deref for UserActivation {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for UserActivation {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<UserActivation> for emlite::Val {
    fn from(s: UserActivation) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl UserActivation {
    pub fn has_been_active(&self) -> bool {
        self.inner.get("hasBeenActive").as_::<bool>()
    }
}
impl UserActivation {
    pub fn is_active(&self) -> bool {
        self.inner.get("isActive").as_::<bool>()
    }
}
