use super::*;




#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct CanvasPattern {
    inner: emlite::Val,
}
impl FromVal for CanvasPattern {
    fn from_val(v: &emlite::Val) -> Self {
        CanvasPattern { inner: emlite::Val::from_val(v) }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for CanvasPattern {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for CanvasPattern {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for CanvasPattern {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for CanvasPattern {
    fn as_mut(&mut self) -> &mut emlite::Val {
      &mut self.inner
  }
}
impl From<CanvasPattern> for emlite::Val {
    fn from(s: CanvasPattern) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}
jsbind::utils::impl_dyn_cast!(CanvasPattern);


impl CanvasPattern {
    pub fn set_transform0(&self, ) -> Undefined {
        self.inner.call("setTransform", &[]).as_::<Undefined>()
    }

    pub fn set_transform1(&self, transform: DOMMatrix2DInit) -> Undefined {
        self.inner.call("setTransform", &[transform.into(), ]).as_::<Undefined>()
    }

}
