use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct CSSFunctionDescriptors {
    inner: CSSStyleDeclaration,
}
impl FromVal for CSSFunctionDescriptors {
    fn from_val(v: &emlite::Val) -> Self {
        CSSFunctionDescriptors {
            inner: CSSStyleDeclaration::from_val(v),
        }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for CSSFunctionDescriptors {
    type Target = CSSStyleDeclaration;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for CSSFunctionDescriptors {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for CSSFunctionDescriptors {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for CSSFunctionDescriptors {
    fn as_mut(&mut self) -> &mut emlite::Val {
        &mut self.inner
    }
}
impl From<CSSFunctionDescriptors> for emlite::Val {
    fn from(s: CSSFunctionDescriptors) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}
jsbind::utils::impl_dyn_cast!(CSSFunctionDescriptors);

impl CSSFunctionDescriptors {
    pub fn result(&self) -> CSSOMString {
        self.inner.get("result").as_::<CSSOMString>()
    }

    pub fn set_result(&mut self, value: CSSOMString) {
        self.inner.set("result", value);
    }
}
