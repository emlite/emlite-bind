use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct XRReferenceSpace {
    inner: XRSpace,
}
impl FromVal for XRReferenceSpace {
    fn from_val(v: &emlite::Val) -> Self {
        XRReferenceSpace {
            inner: XRSpace::from_val(v),
        }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for XRReferenceSpace {
    type Target = XRSpace;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for XRReferenceSpace {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for XRReferenceSpace {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for XRReferenceSpace {
    fn as_mut(&mut self) -> &mut emlite::Val {
        &mut self.inner
    }
}
impl From<XRReferenceSpace> for emlite::Val {
    fn from(s: XRReferenceSpace) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}
impl From<&XRReferenceSpace> for emlite::Val {
    fn from(s: &XRReferenceSpace) -> emlite::Val {
        s.inner.clone().into()
    }
}
jsbind::utils::impl_dyn_cast!(XRReferenceSpace);

impl XRReferenceSpace {
    pub fn get_offset_reference_space(&self, origin_offset: XRRigidTransform) -> XRReferenceSpace {
        self.inner
            .call("getOffsetReferenceSpace", &[origin_offset.into()])
            .as_::<XRReferenceSpace>()
    }
}
impl XRReferenceSpace {
    pub fn onreset(&self) -> Any {
        self.inner.get("onreset").as_::<Any>()
    }

    pub fn set_onreset(&mut self, value: Any) {
        self.inner.set("onreset", value);
    }
}
