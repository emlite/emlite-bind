use super::*;

#[derive(Clone, Debug)]
pub struct CSSPerspective {
    inner: CSSTransformComponent,
}
impl FromVal for CSSPerspective {
    fn from_val(v: &emlite::Val) -> Self {
        CSSPerspective {
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
impl std::ops::Deref for CSSPerspective {
    type Target = CSSTransformComponent;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for CSSPerspective {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<CSSPerspective> for emlite::Val {
    fn from(s: CSSPerspective) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl CSSPerspective {
    pub fn new(length: jsbind::Any) -> CSSPerspective {
        Self {
            inner: emlite::Val::global("CSSPerspective")
                .new(&[length.into()])
                .as_::<CSSTransformComponent>(),
        }
    }
}
impl CSSPerspective {
    pub fn length(&self) -> jsbind::Any {
        self.inner.get("length").as_::<jsbind::Any>()
    }

    pub fn set_length(&mut self, value: jsbind::Any) {
        self.inner.set("length", value);
    }
}
