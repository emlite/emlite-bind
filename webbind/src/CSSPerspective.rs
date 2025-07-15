use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
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
impl core::ops::Deref for CSSPerspective {
    type Target = CSSTransformComponent;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for CSSPerspective {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for CSSPerspective {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for CSSPerspective {
    fn as_mut(&mut self) -> &mut emlite::Val {
        &mut self.inner
    }
}
impl From<CSSPerspective> for emlite::Val {
    fn from(s: CSSPerspective) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}
impl From<&CSSPerspective> for emlite::Val {
    fn from(s: &CSSPerspective) -> emlite::Val {
        s.inner.clone().into()
    }
}
jsbind::utils::impl_dyn_cast!(CSSPerspective);

impl CSSPerspective {
    pub fn new(length: Any) -> CSSPerspective {
        Self {
            inner: emlite::Val::global("CSSPerspective")
                .new(&[length.into()])
                .as_::<CSSTransformComponent>(),
        }
    }
}
impl CSSPerspective {
    pub fn length(&self) -> Any {
        self.inner.get("length").as_::<Any>()
    }

    pub fn set_length(&mut self, value: Any) {
        self.inner.set("length", value);
    }
}
