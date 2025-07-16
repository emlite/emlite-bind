use super::*;

/// The OES_vertex_array_object class.
/// [`OES_vertex_array_object`](https://developer.mozilla.org/en-US/docs/Web/API/OES_vertex_array_object)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct OES_vertex_array_object {
    inner: Any,
}
impl FromVal for OES_vertex_array_object {
    fn from_val(v: &Any) -> Self {
        OES_vertex_array_object {
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
impl core::ops::Deref for OES_vertex_array_object {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for OES_vertex_array_object {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for OES_vertex_array_object {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for OES_vertex_array_object {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<OES_vertex_array_object> for Any {
    fn from(s: OES_vertex_array_object) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&OES_vertex_array_object> for Any {
    fn from(s: &OES_vertex_array_object) -> Any {
        s.inner.clone().into()
    }
}
jsbind::utils::impl_dyn_cast!(OES_vertex_array_object);

impl OES_vertex_array_object {
    /// The createVertexArrayOES method.
    /// [`OES_vertex_array_object.createVertexArrayOES`](https://developer.mozilla.org/en-US/docs/Web/API/OES_vertex_array_object/createVertexArrayOES)
    pub fn create_vertex_array_oes(&self) -> WebGLVertexArrayObjectOES {
        self.inner
            .call("createVertexArrayOES", &[])
            .as_::<WebGLVertexArrayObjectOES>()
    }
}
impl OES_vertex_array_object {
    /// The deleteVertexArrayOES method.
    /// [`OES_vertex_array_object.deleteVertexArrayOES`](https://developer.mozilla.org/en-US/docs/Web/API/OES_vertex_array_object/deleteVertexArrayOES)
    pub fn delete_vertex_array_oes(&self, array_object: &WebGLVertexArrayObjectOES) -> Undefined {
        self.inner
            .call("deleteVertexArrayOES", &[array_object.into()])
            .as_::<Undefined>()
    }
}
impl OES_vertex_array_object {
    /// The isVertexArrayOES method.
    /// [`OES_vertex_array_object.isVertexArrayOES`](https://developer.mozilla.org/en-US/docs/Web/API/OES_vertex_array_object/isVertexArrayOES)
    pub fn is_vertex_array_oes(&self, array_object: &WebGLVertexArrayObjectOES) -> Any {
        self.inner
            .call("isVertexArrayOES", &[array_object.into()])
            .as_::<Any>()
    }
}
impl OES_vertex_array_object {
    /// The bindVertexArrayOES method.
    /// [`OES_vertex_array_object.bindVertexArrayOES`](https://developer.mozilla.org/en-US/docs/Web/API/OES_vertex_array_object/bindVertexArrayOES)
    pub fn bind_vertex_array_oes(&self, array_object: &WebGLVertexArrayObjectOES) -> Undefined {
        self.inner
            .call("bindVertexArrayOES", &[array_object.into()])
            .as_::<Undefined>()
    }
}
