use super::*;

#[derive(Clone, Debug)]
pub struct GPURenderBundleDescriptor {
    inner: emlite::Val,
}
impl FromVal for GPURenderBundleDescriptor {
    fn from_val(v: &emlite::Val) -> Self {
        GPURenderBundleDescriptor { inner: v.clone() }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl std::ops::Deref for GPURenderBundleDescriptor {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for GPURenderBundleDescriptor {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<GPURenderBundleDescriptor> for emlite::Val {
    fn from(s: GPURenderBundleDescriptor) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

#[derive(Clone, Debug)]
pub struct GPURenderBundleEncoder {
    inner: emlite::Val,
}
impl FromVal for GPURenderBundleEncoder {
    fn from_val(v: &emlite::Val) -> Self {
        GPURenderBundleEncoder {
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
impl std::ops::Deref for GPURenderBundleEncoder {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for GPURenderBundleEncoder {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<GPURenderBundleEncoder> for emlite::Val {
    fn from(s: GPURenderBundleEncoder) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl GPURenderBundleEncoder {
    pub fn finish0(&self) -> GPURenderBundle {
        self.inner.call("finish", &[]).as_::<GPURenderBundle>()
    }

    pub fn finish1(&self, descriptor: GPURenderBundleDescriptor) -> GPURenderBundle {
        self.inner
            .call("finish", &[descriptor.into()])
            .as_::<GPURenderBundle>()
    }
}
impl GPURenderBundleEncoder {
    pub fn label(&self) -> jsbind::USVString {
        self.inner.get("label").as_::<jsbind::USVString>()
    }

    pub fn set_label(&mut self, value: jsbind::USVString) {
        self.inner.set("label", value);
    }
}
impl GPURenderBundleEncoder {
    pub fn push_debug_group(&self, group_label: jsbind::USVString) -> jsbind::Undefined {
        self.inner
            .call("pushDebugGroup", &[group_label.into()])
            .as_::<jsbind::Undefined>()
    }
}
impl GPURenderBundleEncoder {
    pub fn pop_debug_group(&self) -> jsbind::Undefined {
        self.inner
            .call("popDebugGroup", &[])
            .as_::<jsbind::Undefined>()
    }
}
impl GPURenderBundleEncoder {
    pub fn insert_debug_marker(&self, marker_label: jsbind::USVString) -> jsbind::Undefined {
        self.inner
            .call("insertDebugMarker", &[marker_label.into()])
            .as_::<jsbind::Undefined>()
    }
}
impl GPURenderBundleEncoder {
    pub fn set_bind_group(
        &self,
        index: jsbind::Any,
        bind_group: GPUBindGroup,
        dynamic_offsets_data: jsbind::Uint32Array,
        dynamic_offsets_data_start: jsbind::Any,
        dynamic_offsets_data_length: jsbind::Any,
    ) -> jsbind::Undefined {
        self.inner
            .call(
                "setBindGroup",
                &[
                    index.into(),
                    bind_group.into(),
                    dynamic_offsets_data.into(),
                    dynamic_offsets_data_start.into(),
                    dynamic_offsets_data_length.into(),
                ],
            )
            .as_::<jsbind::Undefined>()
    }
}
impl GPURenderBundleEncoder {
    pub fn set_pipeline(&self, pipeline: GPURenderPipeline) -> jsbind::Undefined {
        self.inner
            .call("setPipeline", &[pipeline.into()])
            .as_::<jsbind::Undefined>()
    }
}
impl GPURenderBundleEncoder {
    pub fn set_index_buffer0(
        &self,
        buffer: GPUBuffer,
        index_format: GPUIndexFormat,
    ) -> jsbind::Undefined {
        self.inner
            .call("setIndexBuffer", &[buffer.into(), index_format.into()])
            .as_::<jsbind::Undefined>()
    }

    pub fn set_index_buffer1(
        &self,
        buffer: GPUBuffer,
        index_format: GPUIndexFormat,
        offset: jsbind::Any,
    ) -> jsbind::Undefined {
        self.inner
            .call(
                "setIndexBuffer",
                &[buffer.into(), index_format.into(), offset.into()],
            )
            .as_::<jsbind::Undefined>()
    }

    pub fn set_index_buffer2(
        &self,
        buffer: GPUBuffer,
        index_format: GPUIndexFormat,
        offset: jsbind::Any,
        size: jsbind::Any,
    ) -> jsbind::Undefined {
        self.inner
            .call(
                "setIndexBuffer",
                &[
                    buffer.into(),
                    index_format.into(),
                    offset.into(),
                    size.into(),
                ],
            )
            .as_::<jsbind::Undefined>()
    }
}
impl GPURenderBundleEncoder {
    pub fn set_vertex_buffer0(&self, slot: jsbind::Any, buffer: GPUBuffer) -> jsbind::Undefined {
        self.inner
            .call("setVertexBuffer", &[slot.into(), buffer.into()])
            .as_::<jsbind::Undefined>()
    }

    pub fn set_vertex_buffer1(
        &self,
        slot: jsbind::Any,
        buffer: GPUBuffer,
        offset: jsbind::Any,
    ) -> jsbind::Undefined {
        self.inner
            .call(
                "setVertexBuffer",
                &[slot.into(), buffer.into(), offset.into()],
            )
            .as_::<jsbind::Undefined>()
    }

    pub fn set_vertex_buffer2(
        &self,
        slot: jsbind::Any,
        buffer: GPUBuffer,
        offset: jsbind::Any,
        size: jsbind::Any,
    ) -> jsbind::Undefined {
        self.inner
            .call(
                "setVertexBuffer",
                &[slot.into(), buffer.into(), offset.into(), size.into()],
            )
            .as_::<jsbind::Undefined>()
    }
}
impl GPURenderBundleEncoder {
    pub fn draw0(&self, vertex_count: jsbind::Any) -> jsbind::Undefined {
        self.inner
            .call("draw", &[vertex_count.into()])
            .as_::<jsbind::Undefined>()
    }

    pub fn draw1(
        &self,
        vertex_count: jsbind::Any,
        instance_count: jsbind::Any,
    ) -> jsbind::Undefined {
        self.inner
            .call("draw", &[vertex_count.into(), instance_count.into()])
            .as_::<jsbind::Undefined>()
    }

    pub fn draw2(
        &self,
        vertex_count: jsbind::Any,
        instance_count: jsbind::Any,
        first_vertex: jsbind::Any,
    ) -> jsbind::Undefined {
        self.inner
            .call(
                "draw",
                &[
                    vertex_count.into(),
                    instance_count.into(),
                    first_vertex.into(),
                ],
            )
            .as_::<jsbind::Undefined>()
    }

    pub fn draw3(
        &self,
        vertex_count: jsbind::Any,
        instance_count: jsbind::Any,
        first_vertex: jsbind::Any,
        first_instance: jsbind::Any,
    ) -> jsbind::Undefined {
        self.inner
            .call(
                "draw",
                &[
                    vertex_count.into(),
                    instance_count.into(),
                    first_vertex.into(),
                    first_instance.into(),
                ],
            )
            .as_::<jsbind::Undefined>()
    }
}
impl GPURenderBundleEncoder {
    pub fn draw_indexed0(&self, index_count: jsbind::Any) -> jsbind::Undefined {
        self.inner
            .call("drawIndexed", &[index_count.into()])
            .as_::<jsbind::Undefined>()
    }

    pub fn draw_indexed1(
        &self,
        index_count: jsbind::Any,
        instance_count: jsbind::Any,
    ) -> jsbind::Undefined {
        self.inner
            .call("drawIndexed", &[index_count.into(), instance_count.into()])
            .as_::<jsbind::Undefined>()
    }

    pub fn draw_indexed2(
        &self,
        index_count: jsbind::Any,
        instance_count: jsbind::Any,
        first_index: jsbind::Any,
    ) -> jsbind::Undefined {
        self.inner
            .call(
                "drawIndexed",
                &[
                    index_count.into(),
                    instance_count.into(),
                    first_index.into(),
                ],
            )
            .as_::<jsbind::Undefined>()
    }

    pub fn draw_indexed3(
        &self,
        index_count: jsbind::Any,
        instance_count: jsbind::Any,
        first_index: jsbind::Any,
        base_vertex: jsbind::Any,
    ) -> jsbind::Undefined {
        self.inner
            .call(
                "drawIndexed",
                &[
                    index_count.into(),
                    instance_count.into(),
                    first_index.into(),
                    base_vertex.into(),
                ],
            )
            .as_::<jsbind::Undefined>()
    }

    pub fn draw_indexed4(
        &self,
        index_count: jsbind::Any,
        instance_count: jsbind::Any,
        first_index: jsbind::Any,
        base_vertex: jsbind::Any,
        first_instance: jsbind::Any,
    ) -> jsbind::Undefined {
        self.inner
            .call(
                "drawIndexed",
                &[
                    index_count.into(),
                    instance_count.into(),
                    first_index.into(),
                    base_vertex.into(),
                    first_instance.into(),
                ],
            )
            .as_::<jsbind::Undefined>()
    }
}
impl GPURenderBundleEncoder {
    pub fn draw_indirect(
        &self,
        indirect_buffer: GPUBuffer,
        indirect_offset: jsbind::Any,
    ) -> jsbind::Undefined {
        self.inner
            .call(
                "drawIndirect",
                &[indirect_buffer.into(), indirect_offset.into()],
            )
            .as_::<jsbind::Undefined>()
    }
}
impl GPURenderBundleEncoder {
    pub fn draw_indexed_indirect(
        &self,
        indirect_buffer: GPUBuffer,
        indirect_offset: jsbind::Any,
    ) -> jsbind::Undefined {
        self.inner
            .call(
                "drawIndexedIndirect",
                &[indirect_buffer.into(), indirect_offset.into()],
            )
            .as_::<jsbind::Undefined>()
    }
}
