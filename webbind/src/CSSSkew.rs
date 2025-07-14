use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct CSSSkew {
    inner: CSSTransformComponent,
}
impl FromVal for CSSSkew {
    fn from_val(v: &emlite::Val) -> Self {
        CSSSkew {
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
impl core::ops::Deref for CSSSkew {
    type Target = CSSTransformComponent;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for CSSSkew {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<CSSSkew> for emlite::Val {
    fn from(s: CSSSkew) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl CSSSkew {
    pub fn new(ax: CSSNumericValue, ay: CSSNumericValue) -> CSSSkew {
        Self {
            inner: emlite::Val::global("CSSSkew")
                .new(&[ax.into(), ay.into()])
                .as_::<CSSTransformComponent>(),
        }
    }
}
impl CSSSkew {
    pub fn ax(&self) -> CSSNumericValue {
        self.inner.get("ax").as_::<CSSNumericValue>()
    }

    pub fn set_ax(&mut self, value: CSSNumericValue) {
        self.inner.set("ax", value);
    }
}
impl CSSSkew {
    pub fn ay(&self) -> CSSNumericValue {
        self.inner.get("ay").as_::<CSSNumericValue>()
    }

    pub fn set_ay(&mut self, value: CSSNumericValue) {
        self.inner.set("ay", value);
    }
}
