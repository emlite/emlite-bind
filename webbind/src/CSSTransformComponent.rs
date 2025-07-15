use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct CSSTransformComponent {
    inner: emlite::Val,
}
impl FromVal for CSSTransformComponent {
    fn from_val(v: &emlite::Val) -> Self {
        CSSTransformComponent {
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
impl core::ops::Deref for CSSTransformComponent {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for CSSTransformComponent {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for CSSTransformComponent {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for CSSTransformComponent {
    fn as_mut(&mut self) -> &mut emlite::Val {
        &mut self.inner
    }
}
impl From<CSSTransformComponent> for emlite::Val {
    fn from(s: CSSTransformComponent) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}
jsbind::utils::impl_dyn_cast!(CSSTransformComponent);

impl CSSTransformComponent {
    pub fn is2_d(&self) -> bool {
        self.inner.get("is2D").as_::<bool>()
    }

    pub fn set_is2_d(&mut self, value: bool) {
        self.inner.set("is2D", value);
    }
}
impl CSSTransformComponent {
    pub fn to_matrix(&self) -> DOMMatrix {
        self.inner.call("toMatrix", &[]).as_::<DOMMatrix>()
    }
}
