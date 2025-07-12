use super::*;

#[derive(Clone, Debug)]
pub struct XRMesh {
    inner: emlite::Val,
}
impl FromVal for XRMesh {
    fn from_val(v: &emlite::Val) -> Self {
        XRMesh {
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
impl std::ops::Deref for XRMesh {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for XRMesh {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<XRMesh> for emlite::Val {
    fn from(s: XRMesh) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl XRMesh {
    pub fn mesh_space(&self) -> XRSpace {
        self.inner.get("meshSpace").as_::<XRSpace>()
    }
}
impl XRMesh {
    pub fn vertices(&self) -> jsbind::FrozenArray<jsbind::Float32Array> {
        self.inner
            .get("vertices")
            .as_::<jsbind::FrozenArray<jsbind::Float32Array>>()
    }
}
impl XRMesh {
    pub fn indices(&self) -> jsbind::Uint32Array {
        self.inner.get("indices").as_::<jsbind::Uint32Array>()
    }
}
impl XRMesh {
    pub fn last_changed_time(&self) -> jsbind::Any {
        self.inner.get("lastChangedTime").as_::<jsbind::Any>()
    }
}
impl XRMesh {
    pub fn semantic_label(&self) -> jsbind::DOMString {
        self.inner.get("semanticLabel").as_::<jsbind::DOMString>()
    }
}
