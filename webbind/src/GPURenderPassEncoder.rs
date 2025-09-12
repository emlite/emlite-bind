use super::*;

/// The GPURenderPassEncoder class.
/// [`GPURenderPassEncoder`](https://developer.mozilla.org/en-US/docs/Web/API/GPURenderPassEncoder)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct GPURenderPassEncoder {
    inner: Any,
}

impl FromVal for GPURenderPassEncoder {
    fn from_val(v: &Any) -> Self {
        GPURenderPassEncoder {
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

impl core::ops::Deref for GPURenderPassEncoder {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for GPURenderPassEncoder {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for GPURenderPassEncoder {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for GPURenderPassEncoder {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}

impl From<GPURenderPassEncoder> for Any {
    fn from(s: GPURenderPassEncoder) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&GPURenderPassEncoder> for Any {
    fn from(s: &GPURenderPassEncoder) -> Any {
        s.inner.clone().into()
    }
}

jsbind::utils::impl_dyn_cast!(GPURenderPassEncoder);

impl GPURenderPassEncoder {
    /// Getter of the `label` attribute.
    /// [`GPURenderPassEncoder.label`](https://developer.mozilla.org/en-US/docs/Web/API/GPURenderPassEncoder/label)
    pub fn label(&self) -> JsString {
        self.inner.get("label").as_::<JsString>()
    }

    /// Setter of the `label` attribute.
    /// [`GPURenderPassEncoder.label`](https://developer.mozilla.org/en-US/docs/Web/API/GPURenderPassEncoder/label)
    pub fn set_label(&mut self, value: &JsString) {
        self.inner.set("label", value);
    }
}
impl GPURenderPassEncoder {
    /// The setViewport method.
    /// [`GPURenderPassEncoder.setViewport`](https://developer.mozilla.org/en-US/docs/Web/API/GPURenderPassEncoder/setViewport)
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
    /// The setScissorRect method.
    /// [`GPURenderPassEncoder.setScissorRect`](https://developer.mozilla.org/en-US/docs/Web/API/GPURenderPassEncoder/setScissorRect)
    pub fn set_scissor_rect(&self, x: &Any, y: &Any, width: &Any, height: &Any) -> Undefined {
        self.inner
            .call(
                "setScissorRect",
                &[x.into(), y.into(), width.into(), height.into()],
            )
            .as_::<Undefined>()
    }
}
impl GPURenderPassEncoder {
    /// The setBlendConstant method.
    /// [`GPURenderPassEncoder.setBlendConstant`](https://developer.mozilla.org/en-US/docs/Web/API/GPURenderPassEncoder/setBlendConstant)
    pub fn set_blend_constant(&self, color: &Any) -> Undefined {
        self.inner
            .call("setBlendConstant", &[color.into()])
            .as_::<Undefined>()
    }
}
impl GPURenderPassEncoder {
    /// The setStencilReference method.
    /// [`GPURenderPassEncoder.setStencilReference`](https://developer.mozilla.org/en-US/docs/Web/API/GPURenderPassEncoder/setStencilReference)
    pub fn set_stencil_reference(&self, reference: &Any) -> Undefined {
        self.inner
            .call("setStencilReference", &[reference.into()])
            .as_::<Undefined>()
    }
}
impl GPURenderPassEncoder {
    /// The beginOcclusionQuery method.
    /// [`GPURenderPassEncoder.beginOcclusionQuery`](https://developer.mozilla.org/en-US/docs/Web/API/GPURenderPassEncoder/beginOcclusionQuery)
    pub fn begin_occlusion_query(&self, query_index: &Any) -> Undefined {
        self.inner
            .call("beginOcclusionQuery", &[query_index.into()])
            .as_::<Undefined>()
    }
}
impl GPURenderPassEncoder {
    /// The endOcclusionQuery method.
    /// [`GPURenderPassEncoder.endOcclusionQuery`](https://developer.mozilla.org/en-US/docs/Web/API/GPURenderPassEncoder/endOcclusionQuery)
    pub fn end_occlusion_query(&self) -> Undefined {
        self.inner.call("endOcclusionQuery", &[]).as_::<Undefined>()
    }
}
impl GPURenderPassEncoder {
    /// The executeBundles method.
    /// [`GPURenderPassEncoder.executeBundles`](https://developer.mozilla.org/en-US/docs/Web/API/GPURenderPassEncoder/executeBundles)
    pub fn execute_bundles(&self, bundles: &TypedArray<GPURenderBundle>) -> Undefined {
        self.inner
            .call("executeBundles", &[bundles.into()])
            .as_::<Undefined>()
    }
}
impl GPURenderPassEncoder {
    /// The end method.
    /// [`GPURenderPassEncoder.end`](https://developer.mozilla.org/en-US/docs/Web/API/GPURenderPassEncoder/end)
    pub fn end(&self) -> Undefined {
        self.inner.call("end", &[]).as_::<Undefined>()
    }
}
impl GPURenderPassEncoder {
    /// The pushDebugGroup method.
    /// [`GPURenderPassEncoder.pushDebugGroup`](https://developer.mozilla.org/en-US/docs/Web/API/GPURenderPassEncoder/pushDebugGroup)
    pub fn push_debug_group(&self, group_label: &JsString) -> Undefined {
        self.inner
            .call("pushDebugGroup", &[group_label.into()])
            .as_::<Undefined>()
    }
}
impl GPURenderPassEncoder {
    /// The popDebugGroup method.
    /// [`GPURenderPassEncoder.popDebugGroup`](https://developer.mozilla.org/en-US/docs/Web/API/GPURenderPassEncoder/popDebugGroup)
    pub fn pop_debug_group(&self) -> Undefined {
        self.inner.call("popDebugGroup", &[]).as_::<Undefined>()
    }
}
impl GPURenderPassEncoder {
    /// The insertDebugMarker method.
    /// [`GPURenderPassEncoder.insertDebugMarker`](https://developer.mozilla.org/en-US/docs/Web/API/GPURenderPassEncoder/insertDebugMarker)
    pub fn insert_debug_marker(&self, marker_label: &JsString) -> Undefined {
        self.inner
            .call("insertDebugMarker", &[marker_label.into()])
            .as_::<Undefined>()
    }
}
impl GPURenderPassEncoder {
    /// The setBindGroup method.
    /// [`GPURenderPassEncoder.setBindGroup`](https://developer.mozilla.org/en-US/docs/Web/API/GPURenderPassEncoder/setBindGroup)
    pub fn set_bind_group(&self, index: &Any, bind_group: &GPUBindGroup) -> Undefined {
        self.inner
            .call("setBindGroup", &[index.into(), bind_group.into()])
            .as_::<Undefined>()
    }
}
impl GPURenderPassEncoder {
    /// The setBindGroup method.
    /// [`GPURenderPassEncoder.setBindGroup`](https://developer.mozilla.org/en-US/docs/Web/API/GPURenderPassEncoder/setBindGroup)
    pub fn set_bind_group_with_dynamic_offsets(
        &self,
        index: &Any,
        bind_group: &GPUBindGroup,
        dynamic_offsets: &TypedArray<Any>,
    ) -> Undefined {
        self.inner
            .call(
                "setBindGroup",
                &[index.into(), bind_group.into(), dynamic_offsets.into()],
            )
            .as_::<Undefined>()
    }
}
impl GPURenderPassEncoder {
    /// The setBindGroup method.
    /// [`GPURenderPassEncoder.setBindGroup`](https://developer.mozilla.org/en-US/docs/Web/API/GPURenderPassEncoder/setBindGroup)
    pub fn set_bind_group_with_index_and_bind_group_and_dynamic_offsets_data_and_dynamic_offsets_data_start_and_dynamic_offsets_data_length(
        &self,
        index: &Any,
        bind_group: &GPUBindGroup,
        dynamic_offsets_data: &Uint32Array,
        dynamic_offsets_data_start: &Any,
        dynamic_offsets_data_length: &Any,
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
    /// The setPipeline method.
    /// [`GPURenderPassEncoder.setPipeline`](https://developer.mozilla.org/en-US/docs/Web/API/GPURenderPassEncoder/setPipeline)
    pub fn set_pipeline(&self, pipeline: &GPURenderPipeline) -> Undefined {
        self.inner
            .call("setPipeline", &[pipeline.into()])
            .as_::<Undefined>()
    }
}
impl GPURenderPassEncoder {
    /// The setIndexBuffer method.
    /// [`GPURenderPassEncoder.setIndexBuffer`](https://developer.mozilla.org/en-US/docs/Web/API/GPURenderPassEncoder/setIndexBuffer)
    pub fn set_index_buffer(&self, buffer: &GPUBuffer, index_format: &GPUIndexFormat) -> Undefined {
        self.inner
            .call("setIndexBuffer", &[buffer.into(), index_format.into()])
            .as_::<Undefined>()
    }
}
impl GPURenderPassEncoder {
    /// The setIndexBuffer method.
    /// [`GPURenderPassEncoder.setIndexBuffer`](https://developer.mozilla.org/en-US/docs/Web/API/GPURenderPassEncoder/setIndexBuffer)
    pub fn set_index_buffer_with_offset(
        &self,
        buffer: &GPUBuffer,
        index_format: &GPUIndexFormat,
        offset: &Any,
    ) -> Undefined {
        self.inner
            .call(
                "setIndexBuffer",
                &[buffer.into(), index_format.into(), offset.into()],
            )
            .as_::<Undefined>()
    }
}
impl GPURenderPassEncoder {
    /// The setIndexBuffer method.
    /// [`GPURenderPassEncoder.setIndexBuffer`](https://developer.mozilla.org/en-US/docs/Web/API/GPURenderPassEncoder/setIndexBuffer)
    pub fn set_index_buffer_with_offset_and_size(
        &self,
        buffer: &GPUBuffer,
        index_format: &GPUIndexFormat,
        offset: &Any,
        size: &Any,
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
    /// The setVertexBuffer method.
    /// [`GPURenderPassEncoder.setVertexBuffer`](https://developer.mozilla.org/en-US/docs/Web/API/GPURenderPassEncoder/setVertexBuffer)
    pub fn set_vertex_buffer(&self, slot: &Any, buffer: &GPUBuffer) -> Undefined {
        self.inner
            .call("setVertexBuffer", &[slot.into(), buffer.into()])
            .as_::<Undefined>()
    }
}
impl GPURenderPassEncoder {
    /// The setVertexBuffer method.
    /// [`GPURenderPassEncoder.setVertexBuffer`](https://developer.mozilla.org/en-US/docs/Web/API/GPURenderPassEncoder/setVertexBuffer)
    pub fn set_vertex_buffer_with_offset(
        &self,
        slot: &Any,
        buffer: &GPUBuffer,
        offset: &Any,
    ) -> Undefined {
        self.inner
            .call(
                "setVertexBuffer",
                &[slot.into(), buffer.into(), offset.into()],
            )
            .as_::<Undefined>()
    }
}
impl GPURenderPassEncoder {
    /// The setVertexBuffer method.
    /// [`GPURenderPassEncoder.setVertexBuffer`](https://developer.mozilla.org/en-US/docs/Web/API/GPURenderPassEncoder/setVertexBuffer)
    pub fn set_vertex_buffer_with_offset_and_size(
        &self,
        slot: &Any,
        buffer: &GPUBuffer,
        offset: &Any,
        size: &Any,
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
    /// The draw method.
    /// [`GPURenderPassEncoder.draw`](https://developer.mozilla.org/en-US/docs/Web/API/GPURenderPassEncoder/draw)
    pub fn draw(&self, vertex_count: &Any) -> Undefined {
        self.inner
            .call("draw", &[vertex_count.into()])
            .as_::<Undefined>()
    }
}
impl GPURenderPassEncoder {
    /// The draw method.
    /// [`GPURenderPassEncoder.draw`](https://developer.mozilla.org/en-US/docs/Web/API/GPURenderPassEncoder/draw)
    pub fn draw_with_instance_count(&self, vertex_count: &Any, instance_count: &Any) -> Undefined {
        self.inner
            .call("draw", &[vertex_count.into(), instance_count.into()])
            .as_::<Undefined>()
    }
}
impl GPURenderPassEncoder {
    /// The draw method.
    /// [`GPURenderPassEncoder.draw`](https://developer.mozilla.org/en-US/docs/Web/API/GPURenderPassEncoder/draw)
    pub fn draw_with_instance_count_and_first_vertex(
        &self,
        vertex_count: &Any,
        instance_count: &Any,
        first_vertex: &Any,
    ) -> Undefined {
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
}
impl GPURenderPassEncoder {
    /// The draw method.
    /// [`GPURenderPassEncoder.draw`](https://developer.mozilla.org/en-US/docs/Web/API/GPURenderPassEncoder/draw)
    pub fn draw_with_instance_count_and_first_vertex_and_first_instance(
        &self,
        vertex_count: &Any,
        instance_count: &Any,
        first_vertex: &Any,
        first_instance: &Any,
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
    /// The drawIndexed method.
    /// [`GPURenderPassEncoder.drawIndexed`](https://developer.mozilla.org/en-US/docs/Web/API/GPURenderPassEncoder/drawIndexed)
    pub fn draw_indexed(&self, index_count: &Any) -> Undefined {
        self.inner
            .call("drawIndexed", &[index_count.into()])
            .as_::<Undefined>()
    }
}
impl GPURenderPassEncoder {
    /// The drawIndexed method.
    /// [`GPURenderPassEncoder.drawIndexed`](https://developer.mozilla.org/en-US/docs/Web/API/GPURenderPassEncoder/drawIndexed)
    pub fn draw_indexed_with_instance_count(
        &self,
        index_count: &Any,
        instance_count: &Any,
    ) -> Undefined {
        self.inner
            .call("drawIndexed", &[index_count.into(), instance_count.into()])
            .as_::<Undefined>()
    }
}
impl GPURenderPassEncoder {
    /// The drawIndexed method.
    /// [`GPURenderPassEncoder.drawIndexed`](https://developer.mozilla.org/en-US/docs/Web/API/GPURenderPassEncoder/drawIndexed)
    pub fn draw_indexed_with_instance_count_and_first_index(
        &self,
        index_count: &Any,
        instance_count: &Any,
        first_index: &Any,
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
}
impl GPURenderPassEncoder {
    /// The drawIndexed method.
    /// [`GPURenderPassEncoder.drawIndexed`](https://developer.mozilla.org/en-US/docs/Web/API/GPURenderPassEncoder/drawIndexed)
    pub fn draw_indexed_with_instance_count_and_first_index_and_base_vertex(
        &self,
        index_count: &Any,
        instance_count: &Any,
        first_index: &Any,
        base_vertex: &Any,
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
}
impl GPURenderPassEncoder {
    /// The drawIndexed method.
    /// [`GPURenderPassEncoder.drawIndexed`](https://developer.mozilla.org/en-US/docs/Web/API/GPURenderPassEncoder/drawIndexed)
    pub fn draw_indexed_with_instance_count_and_first_index_and_base_vertex_and_first_instance(
        &self,
        index_count: &Any,
        instance_count: &Any,
        first_index: &Any,
        base_vertex: &Any,
        first_instance: &Any,
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
    /// The drawIndirect method.
    /// [`GPURenderPassEncoder.drawIndirect`](https://developer.mozilla.org/en-US/docs/Web/API/GPURenderPassEncoder/drawIndirect)
    pub fn draw_indirect(&self, indirect_buffer: &GPUBuffer, indirect_offset: &Any) -> Undefined {
        self.inner
            .call(
                "drawIndirect",
                &[indirect_buffer.into(), indirect_offset.into()],
            )
            .as_::<Undefined>()
    }
}
impl GPURenderPassEncoder {
    /// The drawIndexedIndirect method.
    /// [`GPURenderPassEncoder.drawIndexedIndirect`](https://developer.mozilla.org/en-US/docs/Web/API/GPURenderPassEncoder/drawIndexedIndirect)
    pub fn draw_indexed_indirect(
        &self,
        indirect_buffer: &GPUBuffer,
        indirect_offset: &Any,
    ) -> Undefined {
        self.inner
            .call(
                "drawIndexedIndirect",
                &[indirect_buffer.into(), indirect_offset.into()],
            )
            .as_::<Undefined>()
    }
}
