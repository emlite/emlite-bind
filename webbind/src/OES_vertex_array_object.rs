use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct OES_vertex_array_object {
    inner: emlite::Val,
}
impl FromVal for OES_vertex_array_object {
    fn from_val(v: &emlite::Val) -> Self {
        OES_vertex_array_object {
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
impl core::ops::Deref for OES_vertex_array_object {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for OES_vertex_array_object {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for OES_vertex_array_object {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for OES_vertex_array_object {
    fn as_mut(&mut self) -> &mut emlite::Val {
        &mut self.inner
    }
}
impl From<OES_vertex_array_object> for emlite::Val {
    fn from(s: OES_vertex_array_object) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}
impl From<&OES_vertex_array_object> for emlite::Val {
    fn from(s: &OES_vertex_array_object) -> emlite::Val {
        s.inner.clone().into()
    }
}
jsbind::utils::impl_dyn_cast!(OES_vertex_array_object);

impl OES_vertex_array_object {
    pub fn create_vertex_array_oes(&self) -> WebGLVertexArrayObjectOES {
        self.inner
            .call("createVertexArrayOES", &[])
            .as_::<WebGLVertexArrayObjectOES>()
    }
}
impl OES_vertex_array_object {
    pub fn delete_vertex_array_oes(&self, array_object: &WebGLVertexArrayObjectOES) -> Undefined {
        self.inner
            .call("deleteVertexArrayOES", &[array_object.into()])
            .as_::<Undefined>()
    }
}
impl OES_vertex_array_object {
    pub fn is_vertex_array_oes(&self, array_object: &WebGLVertexArrayObjectOES) -> Any {
        self.inner
            .call("isVertexArrayOES", &[array_object.into()])
            .as_::<Any>()
    }
}
impl OES_vertex_array_object {
    pub fn bind_vertex_array_oes(&self, array_object: &WebGLVertexArrayObjectOES) -> Undefined {
        self.inner
            .call("bindVertexArrayOES", &[array_object.into()])
            .as_::<Undefined>()
    }
}
