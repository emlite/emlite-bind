use super::*;

#[derive(Clone, Debug)]
pub struct CSSScale {
    inner: CSSTransformComponent,
}
impl FromVal for CSSScale {
    fn from_val(v: &emlite::Val) -> Self {
        CSSScale {
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
impl std::ops::Deref for CSSScale {
    type Target = CSSTransformComponent;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for CSSScale {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<CSSScale> for emlite::Val {
    fn from(s: CSSScale) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl CSSScale {
    pub fn new0(x: jsbind::Any, y: jsbind::Any) -> CSSScale {
        Self {
            inner: emlite::Val::global("CSSScale")
                .new(&[x.into(), y.into()])
                .as_::<CSSTransformComponent>(),
        }
    }

    pub fn new1(x: jsbind::Any, y: jsbind::Any, z: jsbind::Any) -> CSSScale {
        Self {
            inner: emlite::Val::global("CSSScale")
                .new(&[x.into(), y.into(), z.into()])
                .as_::<CSSTransformComponent>(),
        }
    }
}
impl CSSScale {
    pub fn x(&self) -> jsbind::Any {
        self.inner.get("x").as_::<jsbind::Any>()
    }

    pub fn set_x(&mut self, value: jsbind::Any) {
        self.inner.set("x", value);
    }
}
impl CSSScale {
    pub fn y(&self) -> jsbind::Any {
        self.inner.get("y").as_::<jsbind::Any>()
    }

    pub fn set_y(&mut self, value: jsbind::Any) {
        self.inner.set("y", value);
    }
}
impl CSSScale {
    pub fn z(&self) -> jsbind::Any {
        self.inner.get("z").as_::<jsbind::Any>()
    }

    pub fn set_z(&mut self, value: jsbind::Any) {
        self.inner.set("z", value);
    }
}
