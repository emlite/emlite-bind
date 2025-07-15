use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct BackgroundFetchRecord {
    inner: emlite::Val,
}
impl FromVal for BackgroundFetchRecord {
    fn from_val(v: &emlite::Val) -> Self {
        BackgroundFetchRecord {
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
impl core::ops::Deref for BackgroundFetchRecord {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for BackgroundFetchRecord {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for BackgroundFetchRecord {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for BackgroundFetchRecord {
    fn as_mut(&mut self) -> &mut emlite::Val {
        &mut self.inner
    }
}
impl From<BackgroundFetchRecord> for emlite::Val {
    fn from(s: BackgroundFetchRecord) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}
jsbind::utils::impl_dyn_cast!(BackgroundFetchRecord);

impl BackgroundFetchRecord {
    pub fn request(&self) -> Request {
        self.inner.get("request").as_::<Request>()
    }
}
impl BackgroundFetchRecord {
    pub fn response_ready(&self) -> Promise {
        self.inner.get("responseReady").as_::<Promise>()
    }
}
