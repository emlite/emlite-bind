use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct FontData {
    inner: emlite::Val,
}
impl FromVal for FontData {
    fn from_val(v: &emlite::Val) -> Self {
        FontData {
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
impl core::ops::Deref for FontData {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for FontData {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for FontData {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for FontData {
    fn as_mut(&mut self) -> &mut emlite::Val {
        &mut self.inner
    }
}
impl From<FontData> for emlite::Val {
    fn from(s: FontData) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}
impl From<&FontData> for emlite::Val {
    fn from(s: &FontData) -> emlite::Val {
        s.inner.clone().into()
    }
}
jsbind::utils::impl_dyn_cast!(FontData);

impl FontData {
    pub fn blob(&self) -> Promise {
        self.inner.call("blob", &[]).as_::<Promise>()
    }
}
impl FontData {
    pub fn postscript_name(&self) -> String {
        self.inner.get("postscriptName").as_::<String>()
    }
}
impl FontData {
    pub fn full_name(&self) -> String {
        self.inner.get("fullName").as_::<String>()
    }
}
impl FontData {
    pub fn family(&self) -> String {
        self.inner.get("family").as_::<String>()
    }
}
impl FontData {
    pub fn style(&self) -> String {
        self.inner.get("style").as_::<String>()
    }
}
