use super::*;

/// The XRMesh class.
/// [`XRMesh`](https://developer.mozilla.org/en-US/docs/Web/API/XRMesh)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct XRMesh {
    inner: Any,
}
impl FromVal for XRMesh {
    fn from_val(v: &Any) -> Self {
        XRMesh {
            inner: Any::from_val(v),
        }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for XRMesh {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for XRMesh {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for XRMesh {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for XRMesh {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<XRMesh> for Any {
    fn from(s: XRMesh) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&XRMesh> for Any {
    fn from(s: &XRMesh) -> Any {
        s.inner.clone().into()
    }
}
jsbind::utils::impl_dyn_cast!(XRMesh);

impl XRMesh {
    /// Getter of the `meshSpace` attribute.
    /// [`XRMesh.meshSpace`](https://developer.mozilla.org/en-US/docs/Web/API/XRMesh/meshSpace)
    pub fn mesh_space(&self) -> XRSpace {
        self.inner.get("meshSpace").as_::<XRSpace>()
    }
}
impl XRMesh {
    /// Getter of the `vertices` attribute.
    /// [`XRMesh.vertices`](https://developer.mozilla.org/en-US/docs/Web/API/XRMesh/vertices)
    pub fn vertices(&self) -> FrozenArray<Float32Array> {
        self.inner
            .get("vertices")
            .as_::<FrozenArray<Float32Array>>()
    }
}
impl XRMesh {
    /// Getter of the `indices` attribute.
    /// [`XRMesh.indices`](https://developer.mozilla.org/en-US/docs/Web/API/XRMesh/indices)
    pub fn indices(&self) -> Uint32Array {
        self.inner.get("indices").as_::<Uint32Array>()
    }
}
impl XRMesh {
    /// Getter of the `lastChangedTime` attribute.
    /// [`XRMesh.lastChangedTime`](https://developer.mozilla.org/en-US/docs/Web/API/XRMesh/lastChangedTime)
    pub fn last_changed_time(&self) -> Any {
        self.inner.get("lastChangedTime").as_::<Any>()
    }
}
impl XRMesh {
    /// Getter of the `semanticLabel` attribute.
    /// [`XRMesh.semanticLabel`](https://developer.mozilla.org/en-US/docs/Web/API/XRMesh/semanticLabel)
    pub fn semantic_label(&self) -> DOMString {
        self.inner.get("semanticLabel").as_::<DOMString>()
    }
}
