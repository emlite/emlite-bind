use super::*;

#[derive(Clone, Debug)]
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
impl std::ops::Deref for CSSFunctionDescriptors {
    type Target = CSSStyleDeclaration;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for CSSFunctionDescriptors {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<CSSFunctionDescriptors> for emlite::Val {
    fn from(s: CSSFunctionDescriptors) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl CSSFunctionDescriptors {
    pub fn result(&self) -> jsbind::CSSOMString {
        self.inner.get("result").as_::<jsbind::CSSOMString>()
    }

    pub fn set_result(&mut self, value: jsbind::CSSOMString) {
        self.inner.set("result", value);
    }
}
