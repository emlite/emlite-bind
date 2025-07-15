use super::*;




#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct XRRay {
    inner: emlite::Val,
}
impl FromVal for XRRay {
    fn from_val(v: &emlite::Val) -> Self {
        XRRay { inner: emlite::Val::from_val(v) }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for XRRay {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for XRRay {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for XRRay {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for XRRay {
    fn as_mut(&mut self) -> &mut emlite::Val {
      &mut self.inner
  }
}
impl From<XRRay> for emlite::Val {
    fn from(s: XRRay) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}
jsbind::utils::impl_dyn_cast!(XRRay);



impl XRRay {
    pub fn new(transform: XRRigidTransform) -> XRRay {
        Self {
            inner: emlite::Val::global("XRRay").new(&[transform.into()]).as_::<emlite::Val>(),
        }
    }

}
impl XRRay {
    pub fn origin(&self) -> DOMPointReadOnly {
        self.inner.get("origin").as_::<DOMPointReadOnly>()
    }

}
impl XRRay {
    pub fn direction(&self) -> DOMPointReadOnly {
        self.inner.get("direction").as_::<DOMPointReadOnly>()
    }

}
impl XRRay {
    pub fn matrix(&self) -> Float32Array {
        self.inner.get("matrix").as_::<Float32Array>()
    }

}
