use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct CSSAnimation {
    inner: Animation,
}
impl FromVal for CSSAnimation {
    fn from_val(v: &emlite::Val) -> Self {
        CSSAnimation {
            inner: Animation::from_val(v),
        }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for CSSAnimation {
    type Target = Animation;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for CSSAnimation {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<CSSAnimation> for emlite::Val {
    fn from(s: CSSAnimation) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl CSSAnimation {
    pub fn animation_name(&self) -> jsbind::CSSOMString {
        self.inner.get("animationName").as_::<jsbind::CSSOMString>()
    }
}
