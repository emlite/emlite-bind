use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
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
impl core::ops::Deref for CSSScale {
    type Target = CSSTransformComponent;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for CSSScale {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for CSSScale {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for CSSScale {
    fn as_mut(&mut self) -> &mut emlite::Val {
        &mut self.inner
    }
}
impl From<CSSScale> for emlite::Val {
    fn from(s: CSSScale) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}
impl From<&CSSScale> for emlite::Val {
    fn from(s: &CSSScale) -> emlite::Val {
        s.inner.clone().into()
    }
}
jsbind::utils::impl_dyn_cast!(CSSScale);

impl CSSScale {
    pub fn new0(x: &Any, y: &Any) -> CSSScale {
        Self {
            inner: emlite::Val::global("CSSScale")
                .new(&[x.into(), y.into()])
                .as_::<CSSTransformComponent>(),
        }
    }

    pub fn new1(x: &Any, y: &Any, z: &Any) -> CSSScale {
        Self {
            inner: emlite::Val::global("CSSScale")
                .new(&[x.into(), y.into(), z.into()])
                .as_::<CSSTransformComponent>(),
        }
    }
}
impl CSSScale {
    pub fn x(&self) -> Any {
        self.inner.get("x").as_::<Any>()
    }

    pub fn set_x(&mut self, value: &Any) {
        self.inner.set("x", value);
    }
}
impl CSSScale {
    pub fn y(&self) -> Any {
        self.inner.get("y").as_::<Any>()
    }

    pub fn set_y(&mut self, value: &Any) {
        self.inner.set("y", value);
    }
}
impl CSSScale {
    pub fn z(&self) -> Any {
        self.inner.get("z").as_::<Any>()
    }

    pub fn set_z(&mut self, value: &Any) {
        self.inner.set("z", value);
    }
}
