use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct XRAnchor {
    inner: emlite::Val,
}
impl FromVal for XRAnchor {
    fn from_val(v: &emlite::Val) -> Self {
        XRAnchor {
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
impl core::ops::Deref for XRAnchor {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for XRAnchor {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for XRAnchor {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for XRAnchor {
    fn as_mut(&mut self) -> &mut emlite::Val {
        &mut self.inner
    }
}
impl From<XRAnchor> for emlite::Val {
    fn from(s: XRAnchor) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}
impl From<&XRAnchor> for emlite::Val {
    fn from(s: &XRAnchor) -> emlite::Val {
        s.inner.clone().into()
    }
}
jsbind::utils::impl_dyn_cast!(XRAnchor);

impl XRAnchor {
    pub fn anchor_space(&self) -> XRSpace {
        self.inner.get("anchorSpace").as_::<XRSpace>()
    }
}
impl XRAnchor {
    pub fn request_persistent_handle(&self) -> Promise {
        self.inner
            .call("requestPersistentHandle", &[])
            .as_::<Promise>()
    }
}
impl XRAnchor {
    pub fn delete(&self) -> Undefined {
        self.inner.call("delete", &[]).as_::<Undefined>()
    }
}
