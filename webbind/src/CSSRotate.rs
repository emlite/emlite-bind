use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct CSSRotate {
    inner: CSSTransformComponent,
}
impl FromVal for CSSRotate {
    fn from_val(v: &emlite::Val) -> Self {
        CSSRotate {
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
impl core::ops::Deref for CSSRotate {
    type Target = CSSTransformComponent;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for CSSRotate {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for CSSRotate {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for CSSRotate {
    fn as_mut(&mut self) -> &mut emlite::Val {
        &mut self.inner
    }
}
impl From<CSSRotate> for emlite::Val {
    fn from(s: CSSRotate) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}
impl From<&CSSRotate> for emlite::Val {
    fn from(s: &CSSRotate) -> emlite::Val {
        s.inner.clone().into()
    }
}
jsbind::utils::impl_dyn_cast!(CSSRotate);

impl CSSRotate {
    pub fn new(x: Any, y: Any, z: Any, angle: CSSNumericValue) -> CSSRotate {
        Self {
            inner: emlite::Val::global("CSSRotate")
                .new(&[x.into(), y.into(), z.into(), angle.into()])
                .as_::<CSSTransformComponent>(),
        }
    }
}
impl CSSRotate {
    pub fn x(&self) -> Any {
        self.inner.get("x").as_::<Any>()
    }

    pub fn set_x(&mut self, value: Any) {
        self.inner.set("x", value);
    }
}
impl CSSRotate {
    pub fn y(&self) -> Any {
        self.inner.get("y").as_::<Any>()
    }

    pub fn set_y(&mut self, value: Any) {
        self.inner.set("y", value);
    }
}
impl CSSRotate {
    pub fn z(&self) -> Any {
        self.inner.get("z").as_::<Any>()
    }

    pub fn set_z(&mut self, value: Any) {
        self.inner.set("z", value);
    }
}
impl CSSRotate {
    pub fn angle(&self) -> CSSNumericValue {
        self.inner.get("angle").as_::<CSSNumericValue>()
    }

    pub fn set_angle(&mut self, value: CSSNumericValue) {
        self.inner.set("angle", value);
    }
}
