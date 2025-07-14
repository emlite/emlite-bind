use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct CSSSkewX {
    inner: CSSTransformComponent,
}
impl FromVal for CSSSkewX {
    fn from_val(v: &emlite::Val) -> Self {
        CSSSkewX {
            inner: CSSTransformComponent::from_val(v),
        }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for CSSSkewX {
    type Target = CSSTransformComponent;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for CSSSkewX {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<CSSSkewX> for emlite::Val {
    fn from(s: CSSSkewX) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl CSSSkewX {
    pub fn new(ax: CSSNumericValue) -> CSSSkewX {
        Self {
            inner: emlite::Val::global("CSSSkewX")
                .new(&[ax.into()])
                .as_::<CSSTransformComponent>(),
        }
    }
}
impl CSSSkewX {
    pub fn ax(&self) -> CSSNumericValue {
        self.inner.get("ax").as_::<CSSNumericValue>()
    }

    pub fn set_ax(&mut self, value: CSSNumericValue) {
        self.inner.set("ax", value);
    }
}
