use super::*;




#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct CSSOKLab {
    inner: CSSColorValue,
}
impl FromVal for CSSOKLab {
    fn from_val(v: &emlite::Val) -> Self {
        CSSOKLab { inner: CSSColorValue::from_val(v) }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for CSSOKLab {
    type Target = CSSColorValue;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for CSSOKLab {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for CSSOKLab {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for CSSOKLab {
    fn as_mut(&mut self) -> &mut emlite::Val {
      &mut self.inner
  }
}
impl From<CSSOKLab> for emlite::Val {
    fn from(s: CSSOKLab) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}
jsbind::utils::impl_dyn_cast!(CSSOKLab);



impl CSSOKLab {
    pub fn new0(l: Any, a: Any, b: Any) -> CSSOKLab {
        Self {
            inner: emlite::Val::global("CSSOKLab").new(&[l.into(), a.into(), b.into()]).as_::<CSSColorValue>(),
        }
    }

    pub fn new1(l: Any, a: Any, b: Any, alpha: Any) -> CSSOKLab {
        Self {
            inner: emlite::Val::global("CSSOKLab").new(&[l.into(), a.into(), b.into(), alpha.into()]).as_::<CSSColorValue>(),
        }
    }

}
impl CSSOKLab {
    pub fn l(&self) -> Any {
        self.inner.get("l").as_::<Any>()
    }

    pub fn set_l(&mut self, value: Any) {
        self.inner.set("l", value);
    }

}
impl CSSOKLab {
    pub fn a(&self) -> Any {
        self.inner.get("a").as_::<Any>()
    }

    pub fn set_a(&mut self, value: Any) {
        self.inner.set("a", value);
    }

}
impl CSSOKLab {
    pub fn b(&self) -> Any {
        self.inner.get("b").as_::<Any>()
    }

    pub fn set_b(&mut self, value: Any) {
        self.inner.set("b", value);
    }

}
impl CSSOKLab {
    pub fn alpha(&self) -> Any {
        self.inner.get("alpha").as_::<Any>()
    }

    pub fn set_alpha(&mut self, value: Any) {
        self.inner.set("alpha", value);
    }

}
