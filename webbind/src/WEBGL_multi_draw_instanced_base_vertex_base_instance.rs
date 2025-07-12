use super::*;

#[derive(Clone, Debug)]
pub struct WEBGL_multi_draw_instanced_base_vertex_base_instance {
    inner: emlite::Val,
}
impl FromVal for WEBGL_multi_draw_instanced_base_vertex_base_instance {
    fn from_val(v: &emlite::Val) -> Self {
        WEBGL_multi_draw_instanced_base_vertex_base_instance {
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
impl std::ops::Deref for WEBGL_multi_draw_instanced_base_vertex_base_instance {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for WEBGL_multi_draw_instanced_base_vertex_base_instance {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<WEBGL_multi_draw_instanced_base_vertex_base_instance> for emlite::Val {
    fn from(s: WEBGL_multi_draw_instanced_base_vertex_base_instance) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl WEBGL_multi_draw_instanced_base_vertex_base_instance {
    pub fn multi_draw_arrays_instanced_base_instance_webgl(
        &self,
        mode: jsbind::Any,
        firsts_list: jsbind::Any,
        firsts_offset: u64,
        counts_list: jsbind::Any,
        counts_offset: u64,
        instance_counts_list: jsbind::Any,
        instance_counts_offset: u64,
        base_instances_list: jsbind::Any,
        base_instances_offset: u64,
        drawcount: jsbind::Any,
    ) -> jsbind::Undefined {
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
            .as_::<jsbind::Undefined>()
    }
}
impl WEBGL_multi_draw_instanced_base_vertex_base_instance {
    pub fn multi_draw_elements_instanced_base_vertex_base_instance_webgl(
        &self,
        mode: jsbind::Any,
        counts_list: jsbind::Any,
        counts_offset: u64,
        type_: jsbind::Any,
        offsets_list: jsbind::Any,
        offsets_offset: u64,
        instance_counts_list: jsbind::Any,
        instance_counts_offset: u64,
        base_vertices_list: jsbind::Any,
        base_vertices_offset: u64,
        base_instances_list: jsbind::Any,
        base_instances_offset: u64,
        drawcount: jsbind::Any,
    ) -> jsbind::Undefined {
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
            .as_::<jsbind::Undefined>()
    }
}
