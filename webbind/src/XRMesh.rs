use super::*;




#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct XRMesh {
    inner: emlite::Val,
}
impl FromVal for XRMesh {
    fn from_val(v: &emlite::Val) -> Self {
        XRMesh { inner: emlite::Val::from_val(v) }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for XRMesh {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for XRMesh {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for XRMesh {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for XRMesh {
    fn as_mut(&mut self) -> &mut emlite::Val {
      &mut self.inner
  }
}
impl From<XRMesh> for emlite::Val {
    fn from(s: XRMesh) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}
jsbind::utils::impl_dyn_cast!(XRMesh);


impl XRMesh {
    pub fn mesh_space(&self) -> XRSpace {
        self.inner.get("meshSpace").as_::<XRSpace>()
    }

}
impl XRMesh {
    pub fn vertices(&self) -> FrozenArray<Float32Array> {
        self.inner.get("vertices").as_::<FrozenArray<Float32Array>>()
    }

}
impl XRMesh {
    pub fn indices(&self) -> Uint32Array {
        self.inner.get("indices").as_::<Uint32Array>()
    }

}
impl XRMesh {
    pub fn last_changed_time(&self) -> Any {
        self.inner.get("lastChangedTime").as_::<Any>()
    }

}
impl XRMesh {
    pub fn semantic_label(&self) -> DOMString {
        self.inner.get("semanticLabel").as_::<DOMString>()
    }

}
