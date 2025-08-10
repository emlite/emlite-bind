use super::*;

/// The GPUComputePassEncoder class.
/// [`GPUComputePassEncoder`](https://developer.mozilla.org/en-US/docs/Web/API/GPUComputePassEncoder)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct GPUComputePassEncoder {
    inner: Any,
}

impl FromVal for GPUComputePassEncoder {
    fn from_val(v: &Any) -> Self {
        GPUComputePassEncoder {
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

impl core::ops::Deref for GPUComputePassEncoder {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for GPUComputePassEncoder {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for GPUComputePassEncoder {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for GPUComputePassEncoder {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}

impl From<GPUComputePassEncoder> for Any {
    fn from(s: GPUComputePassEncoder) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&GPUComputePassEncoder> for Any {
    fn from(s: &GPUComputePassEncoder) -> Any {
        s.inner.clone().into()
    }
}

jsbind::utils::impl_dyn_cast!(GPUComputePassEncoder);

impl GPUComputePassEncoder {
    /// The setPipeline method.
    /// [`GPUComputePassEncoder.setPipeline`](https://developer.mozilla.org/en-US/docs/Web/API/GPUComputePassEncoder/setPipeline)
    pub fn set_pipeline(&self, pipeline: &GPUComputePipeline) -> Undefined {
        self.inner
            .call("setPipeline", &[pipeline.into()])
            .as_::<Undefined>()
    }
}
impl GPUComputePassEncoder {
    /// The dispatchWorkgroups method.
    /// [`GPUComputePassEncoder.dispatchWorkgroups`](https://developer.mozilla.org/en-US/docs/Web/API/GPUComputePassEncoder/dispatchWorkgroups)
    pub fn dispatch_workgroups0(&self, workgroup_count_x: &Any) -> Undefined {
        self.inner
            .call("dispatchWorkgroups", &[workgroup_count_x.into()])
            .as_::<Undefined>()
    }
    /// The dispatchWorkgroups method.
    /// [`GPUComputePassEncoder.dispatchWorkgroups`](https://developer.mozilla.org/en-US/docs/Web/API/GPUComputePassEncoder/dispatchWorkgroups)
    pub fn dispatch_workgroups1(
        &self,
        workgroup_count_x: &Any,
        workgroup_count_y: &Any,
    ) -> Undefined {
        self.inner
            .call(
                "dispatchWorkgroups",
                &[workgroup_count_x.into(), workgroup_count_y.into()],
            )
            .as_::<Undefined>()
    }
    /// The dispatchWorkgroups method.
    /// [`GPUComputePassEncoder.dispatchWorkgroups`](https://developer.mozilla.org/en-US/docs/Web/API/GPUComputePassEncoder/dispatchWorkgroups)
    pub fn dispatch_workgroups2(
        &self,
        workgroup_count_x: &Any,
        workgroup_count_y: &Any,
        workgroup_count_z: &Any,
    ) -> Undefined {
        self.inner
            .call(
                "dispatchWorkgroups",
                &[
                    workgroup_count_x.into(),
                    workgroup_count_y.into(),
                    workgroup_count_z.into(),
                ],
            )
            .as_::<Undefined>()
    }
}
impl GPUComputePassEncoder {
    /// The dispatchWorkgroupsIndirect method.
    /// [`GPUComputePassEncoder.dispatchWorkgroupsIndirect`](https://developer.mozilla.org/en-US/docs/Web/API/GPUComputePassEncoder/dispatchWorkgroupsIndirect)
    pub fn dispatch_workgroups_indirect(
        &self,
        indirect_buffer: &GPUBuffer,
        indirect_offset: &Any,
    ) -> Undefined {
        self.inner
            .call(
                "dispatchWorkgroupsIndirect",
                &[indirect_buffer.into(), indirect_offset.into()],
            )
            .as_::<Undefined>()
    }
}
impl GPUComputePassEncoder {
    /// The end method.
    /// [`GPUComputePassEncoder.end`](https://developer.mozilla.org/en-US/docs/Web/API/GPUComputePassEncoder/end)
    pub fn end(&self) -> Undefined {
        self.inner.call("end", &[]).as_::<Undefined>()
    }
}
impl GPUComputePassEncoder {
    /// Getter of the `label` attribute.
    /// [`GPUComputePassEncoder.label`](https://developer.mozilla.org/en-US/docs/Web/API/GPUComputePassEncoder/label)
    pub fn label(&self) -> JsString {
        self.inner.get("label").as_::<JsString>()
    }

    /// Setter of the `label` attribute.
    /// [`GPUComputePassEncoder.label`](https://developer.mozilla.org/en-US/docs/Web/API/GPUComputePassEncoder/label)
    pub fn set_label(&mut self, value: &JsString) {
        self.inner.set("label", value);
    }
}
impl GPUComputePassEncoder {
    /// The pushDebugGroup method.
    /// [`GPUComputePassEncoder.pushDebugGroup`](https://developer.mozilla.org/en-US/docs/Web/API/GPUComputePassEncoder/pushDebugGroup)
    pub fn push_debug_group(&self, group_label: &JsString) -> Undefined {
        self.inner
            .call("pushDebugGroup", &[group_label.into()])
            .as_::<Undefined>()
    }
}
impl GPUComputePassEncoder {
    /// The popDebugGroup method.
    /// [`GPUComputePassEncoder.popDebugGroup`](https://developer.mozilla.org/en-US/docs/Web/API/GPUComputePassEncoder/popDebugGroup)
    pub fn pop_debug_group(&self) -> Undefined {
        self.inner.call("popDebugGroup", &[]).as_::<Undefined>()
    }
}
impl GPUComputePassEncoder {
    /// The insertDebugMarker method.
    /// [`GPUComputePassEncoder.insertDebugMarker`](https://developer.mozilla.org/en-US/docs/Web/API/GPUComputePassEncoder/insertDebugMarker)
    pub fn insert_debug_marker(&self, marker_label: &JsString) -> Undefined {
        self.inner
            .call("insertDebugMarker", &[marker_label.into()])
            .as_::<Undefined>()
    }
}
impl GPUComputePassEncoder {
    /// The setBindGroup method.
    /// [`GPUComputePassEncoder.setBindGroup`](https://developer.mozilla.org/en-US/docs/Web/API/GPUComputePassEncoder/setBindGroup)
    pub fn set_bind_group(
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
