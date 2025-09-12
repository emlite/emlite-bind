use super::*;

/// The GPURenderBundleEncoder class.
/// [`GPURenderBundleEncoder`](https://developer.mozilla.org/en-US/docs/Web/API/GPURenderBundleEncoder)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct GPURenderBundleEncoder {
    inner: Any,
}

impl FromVal for GPURenderBundleEncoder {
    fn from_val(v: &Any) -> Self {
        GPURenderBundleEncoder {
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

impl core::ops::Deref for GPURenderBundleEncoder {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for GPURenderBundleEncoder {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for GPURenderBundleEncoder {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for GPURenderBundleEncoder {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}

impl From<GPURenderBundleEncoder> for Any {
    fn from(s: GPURenderBundleEncoder) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&GPURenderBundleEncoder> for Any {
    fn from(s: &GPURenderBundleEncoder) -> Any {
        s.inner.clone().into()
    }
}

jsbind::utils::impl_dyn_cast!(GPURenderBundleEncoder);

impl GPURenderBundleEncoder {
    /// Getter of the `label` attribute.
    /// [`GPURenderBundleEncoder.label`](https://developer.mozilla.org/en-US/docs/Web/API/GPURenderBundleEncoder/label)
    pub fn label(&self) -> JsString {
        self.inner.get("label").as_::<JsString>()
    }

    /// Setter of the `label` attribute.
    /// [`GPURenderBundleEncoder.label`](https://developer.mozilla.org/en-US/docs/Web/API/GPURenderBundleEncoder/label)
    pub fn set_label(&mut self, value: &JsString) {
        self.inner.set("label", value);
    }
}
impl GPURenderBundleEncoder {
    /// The finish method.
    /// [`GPURenderBundleEncoder.finish`](https://developer.mozilla.org/en-US/docs/Web/API/GPURenderBundleEncoder/finish)
    pub fn finish(&self) -> GPURenderBundle {
        self.inner.call("finish", &[]).as_::<GPURenderBundle>()
    }
}
impl GPURenderBundleEncoder {
    /// The finish method.
    /// [`GPURenderBundleEncoder.finish`](https://developer.mozilla.org/en-US/docs/Web/API/GPURenderBundleEncoder/finish)
    pub fn finish_with_descriptor(
        &self,
        descriptor: &GPURenderBundleDescriptor,
    ) -> GPURenderBundle {
        self.inner
            .call("finish", &[descriptor.into()])
            .as_::<GPURenderBundle>()
    }
}
impl GPURenderBundleEncoder {
    /// The pushDebugGroup method.
    /// [`GPURenderBundleEncoder.pushDebugGroup`](https://developer.mozilla.org/en-US/docs/Web/API/GPURenderBundleEncoder/pushDebugGroup)
    pub fn push_debug_group(&self, group_label: &JsString) -> Undefined {
        self.inner
            .call("pushDebugGroup", &[group_label.into()])
            .as_::<Undefined>()
    }
}
impl GPURenderBundleEncoder {
    /// The popDebugGroup method.
    /// [`GPURenderBundleEncoder.popDebugGroup`](https://developer.mozilla.org/en-US/docs/Web/API/GPURenderBundleEncoder/popDebugGroup)
    pub fn pop_debug_group(&self) -> Undefined {
        self.inner.call("popDebugGroup", &[]).as_::<Undefined>()
    }
}
impl GPURenderBundleEncoder {
    /// The insertDebugMarker method.
    /// [`GPURenderBundleEncoder.insertDebugMarker`](https://developer.mozilla.org/en-US/docs/Web/API/GPURenderBundleEncoder/insertDebugMarker)
    pub fn insert_debug_marker(&self, marker_label: &JsString) -> Undefined {
        self.inner
            .call("insertDebugMarker", &[marker_label.into()])
            .as_::<Undefined>()
    }
}
impl GPURenderBundleEncoder {
    /// The setBindGroup method.
    /// [`GPURenderBundleEncoder.setBindGroup`](https://developer.mozilla.org/en-US/docs/Web/API/GPURenderBundleEncoder/setBindGroup)
    pub fn set_bind_group(&self, index: &Any, bind_group: &GPUBindGroup) -> Undefined {
        self.inner
            .call("setBindGroup", &[index.into(), bind_group.into()])
            .as_::<Undefined>()
    }
}
impl GPURenderBundleEncoder {
    /// The setBindGroup method.
    /// [`GPURenderBundleEncoder.setBindGroup`](https://developer.mozilla.org/en-US/docs/Web/API/GPURenderBundleEncoder/setBindGroup)
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
impl GPURenderBundleEncoder {
    /// The setBindGroup method.
    /// [`GPURenderBundleEncoder.setBindGroup`](https://developer.mozilla.org/en-US/docs/Web/API/GPURenderBundleEncoder/setBindGroup)
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
impl GPURenderBundleEncoder {
    /// The setPipeline method.
    /// [`GPURenderBundleEncoder.setPipeline`](https://developer.mozilla.org/en-US/docs/Web/API/GPURenderBundleEncoder/setPipeline)
    pub fn set_pipeline(&self, pipeline: &GPURenderPipeline) -> Undefined {
        self.inner
            .call("setPipeline", &[pipeline.into()])
            .as_::<Undefined>()
    }
}
impl GPURenderBundleEncoder {
    /// The setIndexBuffer method.
    /// [`GPURenderBundleEncoder.setIndexBuffer`](https://developer.mozilla.org/en-US/docs/Web/API/GPURenderBundleEncoder/setIndexBuffer)
    pub fn set_index_buffer(&self, buffer: &GPUBuffer, index_format: &GPUIndexFormat) -> Undefined {
        self.inner
            .call("setIndexBuffer", &[buffer.into(), index_format.into()])
            .as_::<Undefined>()
    }
}
impl GPURenderBundleEncoder {
    /// The setIndexBuffer method.
    /// [`GPURenderBundleEncoder.setIndexBuffer`](https://developer.mozilla.org/en-US/docs/Web/API/GPURenderBundleEncoder/setIndexBuffer)
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
impl GPURenderBundleEncoder {
    /// The setIndexBuffer method.
    /// [`GPURenderBundleEncoder.setIndexBuffer`](https://developer.mozilla.org/en-US/docs/Web/API/GPURenderBundleEncoder/setIndexBuffer)
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
impl GPURenderBundleEncoder {
    /// The setVertexBuffer method.
    /// [`GPURenderBundleEncoder.setVertexBuffer`](https://developer.mozilla.org/en-US/docs/Web/API/GPURenderBundleEncoder/setVertexBuffer)
    pub fn set_vertex_buffer(&self, slot: &Any, buffer: &GPUBuffer) -> Undefined {
        self.inner
            .call("setVertexBuffer", &[slot.into(), buffer.into()])
            .as_::<Undefined>()
    }
}
impl GPURenderBundleEncoder {
    /// The setVertexBuffer method.
    /// [`GPURenderBundleEncoder.setVertexBuffer`](https://developer.mozilla.org/en-US/docs/Web/API/GPURenderBundleEncoder/setVertexBuffer)
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
impl GPURenderBundleEncoder {
    /// The setVertexBuffer method.
    /// [`GPURenderBundleEncoder.setVertexBuffer`](https://developer.mozilla.org/en-US/docs/Web/API/GPURenderBundleEncoder/setVertexBuffer)
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
impl GPURenderBundleEncoder {
    /// The draw method.
    /// [`GPURenderBundleEncoder.draw`](https://developer.mozilla.org/en-US/docs/Web/API/GPURenderBundleEncoder/draw)
    pub fn draw(&self, vertex_count: &Any) -> Undefined {
        self.inner
            .call("draw", &[vertex_count.into()])
            .as_::<Undefined>()
    }
}
impl GPURenderBundleEncoder {
    /// The draw method.
    /// [`GPURenderBundleEncoder.draw`](https://developer.mozilla.org/en-US/docs/Web/API/GPURenderBundleEncoder/draw)
    pub fn draw_with_instance_count(&self, vertex_count: &Any, instance_count: &Any) -> Undefined {
        self.inner
            .call("draw", &[vertex_count.into(), instance_count.into()])
            .as_::<Undefined>()
    }
}
impl GPURenderBundleEncoder {
    /// The draw method.
    /// [`GPURenderBundleEncoder.draw`](https://developer.mozilla.org/en-US/docs/Web/API/GPURenderBundleEncoder/draw)
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
impl GPURenderBundleEncoder {
    /// The draw method.
    /// [`GPURenderBundleEncoder.draw`](https://developer.mozilla.org/en-US/docs/Web/API/GPURenderBundleEncoder/draw)
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
impl GPURenderBundleEncoder {
    /// The drawIndexed method.
    /// [`GPURenderBundleEncoder.drawIndexed`](https://developer.mozilla.org/en-US/docs/Web/API/GPURenderBundleEncoder/drawIndexed)
    pub fn draw_indexed(&self, index_count: &Any) -> Undefined {
        self.inner
            .call("drawIndexed", &[index_count.into()])
            .as_::<Undefined>()
    }
}
impl GPURenderBundleEncoder {
    /// The drawIndexed method.
    /// [`GPURenderBundleEncoder.drawIndexed`](https://developer.mozilla.org/en-US/docs/Web/API/GPURenderBundleEncoder/drawIndexed)
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
impl GPURenderBundleEncoder {
    /// The drawIndexed method.
    /// [`GPURenderBundleEncoder.drawIndexed`](https://developer.mozilla.org/en-US/docs/Web/API/GPURenderBundleEncoder/drawIndexed)
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
impl GPURenderBundleEncoder {
    /// The drawIndexed method.
    /// [`GPURenderBundleEncoder.drawIndexed`](https://developer.mozilla.org/en-US/docs/Web/API/GPURenderBundleEncoder/drawIndexed)
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
impl GPURenderBundleEncoder {
    /// The drawIndexed method.
    /// [`GPURenderBundleEncoder.drawIndexed`](https://developer.mozilla.org/en-US/docs/Web/API/GPURenderBundleEncoder/drawIndexed)
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
impl GPURenderBundleEncoder {
    /// The drawIndirect method.
    /// [`GPURenderBundleEncoder.drawIndirect`](https://developer.mozilla.org/en-US/docs/Web/API/GPURenderBundleEncoder/drawIndirect)
    pub fn draw_indirect(&self, indirect_buffer: &GPUBuffer, indirect_offset: &Any) -> Undefined {
        self.inner
            .call(
                "drawIndirect",
                &[indirect_buffer.into(), indirect_offset.into()],
            )
            .as_::<Undefined>()
    }
}
impl GPURenderBundleEncoder {
    /// The drawIndexedIndirect method.
    /// [`GPURenderBundleEncoder.drawIndexedIndirect`](https://developer.mozilla.org/en-US/docs/Web/API/GPURenderBundleEncoder/drawIndexedIndirect)
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
