use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct GPURenderPassEncoder {
    inner: emlite::Val,
}
impl FromVal for GPURenderPassEncoder {
    fn from_val(v: &emlite::Val) -> Self {
        GPURenderPassEncoder {
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
impl core::ops::Deref for GPURenderPassEncoder {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for GPURenderPassEncoder {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for GPURenderPassEncoder {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for GPURenderPassEncoder {
    fn as_mut(&mut self) -> &mut emlite::Val {
        &mut self.inner
    }
}
impl From<GPURenderPassEncoder> for emlite::Val {
    fn from(s: GPURenderPassEncoder) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}
jsbind::utils::impl_dyn_cast!(GPURenderPassEncoder);

impl GPURenderPassEncoder {
    pub fn set_viewport(
        &self,
        x: f32,
        y: f32,
        width: f32,
        height: f32,
        min_depth: f32,
        max_depth: f32,
    ) -> Undefined {
        self.inner
            .call(
                "setViewport",
                &[
                    x.into(),
                    y.into(),
                    width.into(),
                    height.into(),
                    min_depth.into(),
                    max_depth.into(),
                ],
            )
            .as_::<Undefined>()
    }
}
impl GPURenderPassEncoder {
    pub fn set_scissor_rect(&self, x: Any, y: Any, width: Any, height: Any) -> Undefined {
        self.inner
            .call(
                "setScissorRect",
                &[x.into(), y.into(), width.into(), height.into()],
            )
            .as_::<Undefined>()
    }
}
impl GPURenderPassEncoder {
    pub fn set_blend_constant(&self, color: Any) -> Undefined {
        self.inner
            .call("setBlendConstant", &[color.into()])
            .as_::<Undefined>()
    }
}
impl GPURenderPassEncoder {
    pub fn set_stencil_reference(&self, reference: Any) -> Undefined {
        self.inner
            .call("setStencilReference", &[reference.into()])
            .as_::<Undefined>()
    }
}
impl GPURenderPassEncoder {
    pub fn begin_occlusion_query(&self, query_index: Any) -> Undefined {
        self.inner
            .call("beginOcclusionQuery", &[query_index.into()])
            .as_::<Undefined>()
    }
}
impl GPURenderPassEncoder {
    pub fn end_occlusion_query(&self) -> Undefined {
        self.inner.call("endOcclusionQuery", &[]).as_::<Undefined>()
    }
}
impl GPURenderPassEncoder {
    pub fn execute_bundles(&self, bundles: Sequence<GPURenderBundle>) -> Undefined {
        self.inner
            .call("executeBundles", &[bundles.into()])
            .as_::<Undefined>()
    }
}
impl GPURenderPassEncoder {
    pub fn end(&self) -> Undefined {
        self.inner.call("end", &[]).as_::<Undefined>()
    }
}
impl GPURenderPassEncoder {
    pub fn label(&self) -> USVString {
        self.inner.get("label").as_::<USVString>()
    }

    pub fn set_label(&mut self, value: USVString) {
        self.inner.set("label", value);
    }
}
impl GPURenderPassEncoder {
    pub fn push_debug_group(&self, group_label: USVString) -> Undefined {
        self.inner
            .call("pushDebugGroup", &[group_label.into()])
            .as_::<Undefined>()
    }
}
impl GPURenderPassEncoder {
    pub fn pop_debug_group(&self) -> Undefined {
        self.inner.call("popDebugGroup", &[]).as_::<Undefined>()
    }
}
impl GPURenderPassEncoder {
    pub fn insert_debug_marker(&self, marker_label: USVString) -> Undefined {
        self.inner
            .call("insertDebugMarker", &[marker_label.into()])
            .as_::<Undefined>()
    }
}
impl GPURenderPassEncoder {
    pub fn set_bind_group(
        &self,
        index: Any,
        bind_group: GPUBindGroup,
        dynamic_offsets_data: Uint32Array,
        dynamic_offsets_data_start: Any,
        dynamic_offsets_data_length: Any,
    ) -> Undefined {
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
            .as_::<Undefined>()
    }
}
impl GPURenderPassEncoder {
    pub fn set_pipeline(&self, pipeline: GPURenderPipeline) -> Undefined {
        self.inner
            .call("setPipeline", &[pipeline.into()])
            .as_::<Undefined>()
    }
}
impl GPURenderPassEncoder {
    pub fn set_index_buffer0(&self, buffer: GPUBuffer, index_format: GPUIndexFormat) -> Undefined {
        self.inner
            .call("setIndexBuffer", &[buffer.into(), index_format.into()])
            .as_::<Undefined>()
    }

    pub fn set_index_buffer1(
        &self,
        buffer: GPUBuffer,
        index_format: GPUIndexFormat,
        offset: Any,
    ) -> Undefined {
        self.inner
            .call(
                "setIndexBuffer",
                &[buffer.into(), index_format.into(), offset.into()],
            )
            .as_::<Undefined>()
    }

    pub fn set_index_buffer2(
        &self,
        buffer: GPUBuffer,
        index_format: GPUIndexFormat,
        offset: Any,
        size: Any,
    ) -> Undefined {
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
            .as_::<Undefined>()
    }
}
impl GPURenderPassEncoder {
    pub fn set_vertex_buffer0(&self, slot: Any, buffer: GPUBuffer) -> Undefined {
        self.inner
            .call("setVertexBuffer", &[slot.into(), buffer.into()])
            .as_::<Undefined>()
    }

    pub fn set_vertex_buffer1(&self, slot: Any, buffer: GPUBuffer, offset: Any) -> Undefined {
        self.inner
            .call(
                "setVertexBuffer",
                &[slot.into(), buffer.into(), offset.into()],
            )
            .as_::<Undefined>()
    }

    pub fn set_vertex_buffer2(
        &self,
        slot: Any,
        buffer: GPUBuffer,
        offset: Any,
        size: Any,
    ) -> Undefined {
        self.inner
            .call(
                "setVertexBuffer",
                &[slot.into(), buffer.into(), offset.into(), size.into()],
            )
            .as_::<Undefined>()
    }
}
impl GPURenderPassEncoder {
    pub fn draw0(&self, vertex_count: Any) -> Undefined {
        self.inner
            .call("draw", &[vertex_count.into()])
            .as_::<Undefined>()
    }

    pub fn draw1(&self, vertex_count: Any, instance_count: Any) -> Undefined {
        self.inner
            .call("draw", &[vertex_count.into(), instance_count.into()])
            .as_::<Undefined>()
    }

    pub fn draw2(&self, vertex_count: Any, instance_count: Any, first_vertex: Any) -> Undefined {
        self.inner
            .call(
                "draw",
                &[
                    vertex_count.into(),
                    instance_count.into(),
                    first_vertex.into(),
                ],
            )
            .as_::<Undefined>()
    }

    pub fn draw3(
        &self,
        vertex_count: Any,
        instance_count: Any,
        first_vertex: Any,
        first_instance: Any,
    ) -> Undefined {
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
            .as_::<Undefined>()
    }
}
impl GPURenderPassEncoder {
    pub fn draw_indexed0(&self, index_count: Any) -> Undefined {
        self.inner
            .call("drawIndexed", &[index_count.into()])
            .as_::<Undefined>()
    }

    pub fn draw_indexed1(&self, index_count: Any, instance_count: Any) -> Undefined {
        self.inner
            .call("drawIndexed", &[index_count.into(), instance_count.into()])
            .as_::<Undefined>()
    }

    pub fn draw_indexed2(
        &self,
        index_count: Any,
        instance_count: Any,
        first_index: Any,
    ) -> Undefined {
        self.inner
            .call(
                "drawIndexed",
                &[
                    index_count.into(),
                    instance_count.into(),
                    first_index.into(),
                ],
            )
            .as_::<Undefined>()
    }

    pub fn draw_indexed3(
        &self,
        index_count: Any,
        instance_count: Any,
        first_index: Any,
        base_vertex: Any,
    ) -> Undefined {
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
            .as_::<Undefined>()
    }

    pub fn draw_indexed4(
        &self,
        index_count: Any,
        instance_count: Any,
        first_index: Any,
        base_vertex: Any,
        first_instance: Any,
    ) -> Undefined {
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
            .as_::<Undefined>()
    }
}
impl GPURenderPassEncoder {
    pub fn draw_indirect(&self, indirect_buffer: GPUBuffer, indirect_offset: Any) -> Undefined {
        self.inner
            .call(
                "drawIndirect",
                &[indirect_buffer.into(), indirect_offset.into()],
            )
            .as_::<Undefined>()
    }
}
impl GPURenderPassEncoder {
    pub fn draw_indexed_indirect(
        &self,
        indirect_buffer: GPUBuffer,
        indirect_offset: Any,
    ) -> Undefined {
        self.inner
            .call(
                "drawIndexedIndirect",
                &[indirect_buffer.into(), indirect_offset.into()],
            )
            .as_::<Undefined>()
    }
}
