use super::*;




#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct CSSTransformValue {
    inner: CSSStyleValue,
}
impl FromVal for CSSTransformValue {
    fn from_val(v: &emlite::Val) -> Self {
        CSSTransformValue { inner: CSSStyleValue::from_val(v) }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for CSSTransformValue {
    type Target = CSSStyleValue;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for CSSTransformValue {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for CSSTransformValue {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for CSSTransformValue {
    fn as_mut(&mut self) -> &mut emlite::Val {
      &mut self.inner
  }
}
impl From<CSSTransformValue> for emlite::Val {
    fn from(s: CSSTransformValue) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}
jsbind::utils::impl_dyn_cast!(CSSTransformValue);



impl CSSTransformValue {
    pub fn new(transforms: Sequence<CSSTransformComponent>) -> CSSTransformValue {
        Self {
            inner: emlite::Val::global("CSSTransformValue").new(&[transforms.into()]).as_::<CSSStyleValue>(),
        }
    }

}
impl CSSTransformValue {
    pub fn length(&self) -> u32 {
        self.inner.get("length").as_::<u32>()
    }

}
impl CSSTransformValue {
    pub fn is2_d(&self) -> bool {
        self.inner.get("is2D").as_::<bool>()
    }

}
impl CSSTransformValue {
    pub fn to_matrix(&self, ) -> DOMMatrix {
        self.inner.call("toMatrix", &[]).as_::<DOMMatrix>()
    }

}
