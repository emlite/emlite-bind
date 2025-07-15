use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct CSSSkewY {
    inner: CSSTransformComponent,
}
impl FromVal for CSSSkewY {
    fn from_val(v: &emlite::Val) -> Self {
        CSSSkewY {
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
impl core::ops::Deref for CSSSkewY {
    type Target = CSSTransformComponent;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for CSSSkewY {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for CSSSkewY {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for CSSSkewY {
    fn as_mut(&mut self) -> &mut emlite::Val {
        &mut self.inner
    }
}
impl From<CSSSkewY> for emlite::Val {
    fn from(s: CSSSkewY) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}
impl From<&CSSSkewY> for emlite::Val {
    fn from(s: &CSSSkewY) -> emlite::Val {
        s.inner.clone().into()
    }
}
jsbind::utils::impl_dyn_cast!(CSSSkewY);

impl CSSSkewY {
    pub fn new(ay: &CSSNumericValue) -> CSSSkewY {
        Self {
            inner: emlite::Val::global("CSSSkewY")
                .new(&[ay.into()])
                .as_::<CSSTransformComponent>(),
        }
    }
}
impl CSSSkewY {
    pub fn ay(&self) -> CSSNumericValue {
        self.inner.get("ay").as_::<CSSNumericValue>()
    }

    pub fn set_ay(&mut self, value: &CSSNumericValue) {
        self.inner.set("ay", value);
    }
}
