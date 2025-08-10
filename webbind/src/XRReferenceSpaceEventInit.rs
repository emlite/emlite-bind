use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct XRReferenceSpaceEventInit {
    inner: Any,
}
impl FromVal for XRReferenceSpaceEventInit {
    fn from_val(v: &Any) -> Self {
        XRReferenceSpaceEventInit { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for XRReferenceSpaceEventInit {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for XRReferenceSpaceEventInit {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for XRReferenceSpaceEventInit {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for XRReferenceSpaceEventInit {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<XRReferenceSpaceEventInit> for Any {
    fn from(s: XRReferenceSpaceEventInit) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&XRReferenceSpaceEventInit> for Any {
    fn from(s: &XRReferenceSpaceEventInit) -> Any {
        s.inner.clone()
    }
}

impl XRReferenceSpaceEventInit {
    pub fn reference_space(&self) -> XRReferenceSpace {
        self.inner.get("referenceSpace").as_::<XRReferenceSpace>()
    }

    pub fn set_reference_space(&mut self, value: &XRReferenceSpace) {
        self.inner.set("referenceSpace", value);
    }
}
impl XRReferenceSpaceEventInit {
    pub fn transform(&self) -> XRRigidTransform {
        self.inner.get("transform").as_::<XRRigidTransform>()
    }

    pub fn set_transform(&mut self, value: &XRRigidTransform) {
        self.inner.set("transform", value);
    }
}
