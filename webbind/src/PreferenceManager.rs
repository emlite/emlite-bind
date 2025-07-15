use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct PreferenceManager {
    inner: emlite::Val,
}
impl FromVal for PreferenceManager {
    fn from_val(v: &emlite::Val) -> Self {
        PreferenceManager {
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
impl core::ops::Deref for PreferenceManager {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for PreferenceManager {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for PreferenceManager {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for PreferenceManager {
    fn as_mut(&mut self) -> &mut emlite::Val {
        &mut self.inner
    }
}
impl From<PreferenceManager> for emlite::Val {
    fn from(s: PreferenceManager) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}
jsbind::utils::impl_dyn_cast!(PreferenceManager);

impl PreferenceManager {
    pub fn color_scheme(&self) -> PreferenceObject {
        self.inner.get("colorScheme").as_::<PreferenceObject>()
    }
}
impl PreferenceManager {
    pub fn contrast(&self) -> PreferenceObject {
        self.inner.get("contrast").as_::<PreferenceObject>()
    }
}
impl PreferenceManager {
    pub fn reduced_motion(&self) -> PreferenceObject {
        self.inner.get("reducedMotion").as_::<PreferenceObject>()
    }
}
impl PreferenceManager {
    pub fn reduced_transparency(&self) -> PreferenceObject {
        self.inner
            .get("reducedTransparency")
            .as_::<PreferenceObject>()
    }
}
impl PreferenceManager {
    pub fn reduced_data(&self) -> PreferenceObject {
        self.inner.get("reducedData").as_::<PreferenceObject>()
    }
}
