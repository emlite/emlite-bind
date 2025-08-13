use super::*;




/// The WEBGL_draw_instanced_base_vertex_base_instance class.
/// [`WEBGL_draw_instanced_base_vertex_base_instance`](https://developer.mozilla.org/en-US/docs/Web/API/WEBGL_draw_instanced_base_vertex_base_instance)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct WEBGL_draw_instanced_base_vertex_base_instance {
    inner: Any,
}

impl FromVal for WEBGL_draw_instanced_base_vertex_base_instance {
    fn from_val(v: &Any) -> Self {
        WEBGL_draw_instanced_base_vertex_base_instance { inner: Any::from_val(v) }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for WEBGL_draw_instanced_base_vertex_base_instance {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for WEBGL_draw_instanced_base_vertex_base_instance {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for WEBGL_draw_instanced_base_vertex_base_instance {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for WEBGL_draw_instanced_base_vertex_base_instance {
    fn as_mut(&mut self) -> &mut Any {
      &mut self.inner
  }
}

impl From<WEBGL_draw_instanced_base_vertex_base_instance> for Any {
    fn from(s: WEBGL_draw_instanced_base_vertex_base_instance) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&WEBGL_draw_instanced_base_vertex_base_instance> for Any {
    fn from(s: &WEBGL_draw_instanced_base_vertex_base_instance) -> Any {
        s.inner.clone().into()
    }
}

jsbind::utils::impl_dyn_cast!(WEBGL_draw_instanced_base_vertex_base_instance);


impl WEBGL_draw_instanced_base_vertex_base_instance {
    /// The drawArraysInstancedBaseInstanceWEBGL method.
    /// [`WEBGL_draw_instanced_base_vertex_base_instance.drawArraysInstancedBaseInstanceWEBGL`](https://developer.mozilla.org/en-US/docs/Web/API/WEBGL_draw_instanced_base_vertex_base_instance/drawArraysInstancedBaseInstanceWEBGL)
    pub fn draw_arrays_instanced_base_instance_webgl(&self, mode: &Any, first: &Any, count: &Any, instance_count: &Any, base_instance: &Any) -> Undefined {
        self.inner.call("drawArraysInstancedBaseInstanceWEBGL", &[mode.into(), first.into(), count.into(), instance_count.into(), base_instance.into(), ]).as_::<Undefined>()
    }
}
impl WEBGL_draw_instanced_base_vertex_base_instance {
    /// The drawElementsInstancedBaseVertexBaseInstanceWEBGL method.
    /// [`WEBGL_draw_instanced_base_vertex_base_instance.drawElementsInstancedBaseVertexBaseInstanceWEBGL`](https://developer.mozilla.org/en-US/docs/Web/API/WEBGL_draw_instanced_base_vertex_base_instance/drawElementsInstancedBaseVertexBaseInstanceWEBGL)
    pub fn draw_elements_instanced_base_vertex_base_instance_webgl(&self, mode: &Any, count: &Any, type_: &Any, offset: &Any, instance_count: &Any, base_vertex: &Any, base_instance: &Any) -> Undefined {
        self.inner.call("drawElementsInstancedBaseVertexBaseInstanceWEBGL", &[mode.into(), count.into(), type_.into(), offset.into(), instance_count.into(), base_vertex.into(), base_instance.into(), ]).as_::<Undefined>()
    }
}
