use super::*;

/// The WEBGL_multi_draw_instanced_base_vertex_base_instance class.
/// [`WEBGL_multi_draw_instanced_base_vertex_base_instance`](https://developer.mozilla.org/en-US/docs/Web/API/WEBGL_multi_draw_instanced_base_vertex_base_instance)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct WEBGL_multi_draw_instanced_base_vertex_base_instance {
    inner: Any,
}

impl FromVal for WEBGL_multi_draw_instanced_base_vertex_base_instance {
    fn from_val(v: &Any) -> Self {
        WEBGL_multi_draw_instanced_base_vertex_base_instance {
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

impl core::ops::Deref for WEBGL_multi_draw_instanced_base_vertex_base_instance {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for WEBGL_multi_draw_instanced_base_vertex_base_instance {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for WEBGL_multi_draw_instanced_base_vertex_base_instance {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for WEBGL_multi_draw_instanced_base_vertex_base_instance {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}

impl From<WEBGL_multi_draw_instanced_base_vertex_base_instance> for Any {
    fn from(s: WEBGL_multi_draw_instanced_base_vertex_base_instance) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&WEBGL_multi_draw_instanced_base_vertex_base_instance> for Any {
    fn from(s: &WEBGL_multi_draw_instanced_base_vertex_base_instance) -> Any {
        s.inner.clone().into()
    }
}

jsbind::utils::impl_dyn_cast!(WEBGL_multi_draw_instanced_base_vertex_base_instance);

impl WEBGL_multi_draw_instanced_base_vertex_base_instance {
    /// The multiDrawArraysInstancedBaseInstanceWEBGL method.
    /// [`WEBGL_multi_draw_instanced_base_vertex_base_instance.multiDrawArraysInstancedBaseInstanceWEBGL`](https://developer.mozilla.org/en-US/docs/Web/API/WEBGL_multi_draw_instanced_base_vertex_base_instance/multiDrawArraysInstancedBaseInstanceWEBGL)
    pub fn multi_draw_arrays_instanced_base_instance_webgl(
        &self,
        mode: &Any,
        firsts_list: &Any,
        firsts_offset: u64,
        counts_list: &Any,
        counts_offset: u64,
        instance_counts_list: &Any,
        instance_counts_offset: u64,
        base_instances_list: &Any,
        base_instances_offset: u64,
        drawcount: &Any,
    ) -> Undefined {
        self.inner
            .call(
                "multiDrawArraysInstancedBaseInstanceWEBGL",
                &[
                    mode.into(),
                    firsts_list.into(),
                    firsts_offset.into(),
                    counts_list.into(),
                    counts_offset.into(),
                    instance_counts_list.into(),
                    instance_counts_offset.into(),
                    base_instances_list.into(),
                    base_instances_offset.into(),
                    drawcount.into(),
                ],
            )
            .as_::<Undefined>()
    }
}
impl WEBGL_multi_draw_instanced_base_vertex_base_instance {
    /// The multiDrawElementsInstancedBaseVertexBaseInstanceWEBGL method.
    /// [`WEBGL_multi_draw_instanced_base_vertex_base_instance.multiDrawElementsInstancedBaseVertexBaseInstanceWEBGL`](https://developer.mozilla.org/en-US/docs/Web/API/WEBGL_multi_draw_instanced_base_vertex_base_instance/multiDrawElementsInstancedBaseVertexBaseInstanceWEBGL)
    pub fn multi_draw_elements_instanced_base_vertex_base_instance_webgl(
        &self,
        mode: &Any,
        counts_list: &Any,
        counts_offset: u64,
        type_: &Any,
        offsets_list: &Any,
        offsets_offset: u64,
        instance_counts_list: &Any,
        instance_counts_offset: u64,
        base_vertices_list: &Any,
        base_vertices_offset: u64,
        base_instances_list: &Any,
        base_instances_offset: u64,
        drawcount: &Any,
    ) -> Undefined {
        self.inner
            .call(
                "multiDrawElementsInstancedBaseVertexBaseInstanceWEBGL",
                &[
                    mode.into(),
                    counts_list.into(),
                    counts_offset.into(),
                    type_.into(),
                    offsets_list.into(),
                    offsets_offset.into(),
                    instance_counts_list.into(),
                    instance_counts_offset.into(),
                    base_vertices_list.into(),
                    base_vertices_offset.into(),
                    base_instances_list.into(),
                    base_instances_offset.into(),
                    drawcount.into(),
                ],
            )
            .as_::<Undefined>()
    }
}
