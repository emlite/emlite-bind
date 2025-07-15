use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct GeolocationPositionError {
    inner: emlite::Val,
}
impl FromVal for GeolocationPositionError {
    fn from_val(v: &emlite::Val) -> Self {
        GeolocationPositionError {
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
impl core::ops::Deref for GeolocationPositionError {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for GeolocationPositionError {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for GeolocationPositionError {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for GeolocationPositionError {
    fn as_mut(&mut self) -> &mut emlite::Val {
        &mut self.inner
    }
}
impl From<GeolocationPositionError> for emlite::Val {
    fn from(s: GeolocationPositionError) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}
impl From<&GeolocationPositionError> for emlite::Val {
    fn from(s: &GeolocationPositionError) -> emlite::Val {
        s.inner.clone().into()
    }
}
jsbind::utils::impl_dyn_cast!(GeolocationPositionError);

impl GeolocationPositionError {
    pub fn code(&self) -> u16 {
        self.inner.get("code").as_::<u16>()
    }
}
impl GeolocationPositionError {
    pub fn message(&self) -> String {
        self.inner.get("message").as_::<String>()
    }
}
