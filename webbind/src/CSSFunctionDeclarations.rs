use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct CSSFunctionDeclarations {
    inner: CSSRule,
}
impl FromVal for CSSFunctionDeclarations {
    fn from_val(v: &emlite::Val) -> Self {
        CSSFunctionDeclarations {
            inner: CSSRule::from_val(v),
        }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for CSSFunctionDeclarations {
    type Target = CSSRule;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for CSSFunctionDeclarations {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for CSSFunctionDeclarations {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for CSSFunctionDeclarations {
    fn as_mut(&mut self) -> &mut emlite::Val {
        &mut self.inner
    }
}
impl From<CSSFunctionDeclarations> for emlite::Val {
    fn from(s: CSSFunctionDeclarations) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}
impl From<&CSSFunctionDeclarations> for emlite::Val {
    fn from(s: &CSSFunctionDeclarations) -> emlite::Val {
        s.inner.clone().into()
    }
}
jsbind::utils::impl_dyn_cast!(CSSFunctionDeclarations);

impl CSSFunctionDeclarations {
    pub fn style(&self) -> CSSFunctionDescriptors {
        self.inner.get("style").as_::<CSSFunctionDescriptors>()
    }
}
