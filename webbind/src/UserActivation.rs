use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
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
impl core::ops::Deref for UserActivation {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for UserActivation {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for UserActivation {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for UserActivation {
    fn as_mut(&mut self) -> &mut emlite::Val {
        &mut self.inner
    }
}
impl From<UserActivation> for emlite::Val {
    fn from(s: UserActivation) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}
impl From<&UserActivation> for emlite::Val {
    fn from(s: &UserActivation) -> emlite::Val {
        s.inner.clone().into()
    }
}
jsbind::utils::impl_dyn_cast!(UserActivation);

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
