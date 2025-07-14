use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct GPUComputePassEncoder {
    inner: emlite::Val,
}
impl FromVal for GPUComputePassEncoder {
    fn from_val(v: &emlite::Val) -> Self {
        GPUComputePassEncoder {
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
impl core::ops::Deref for GPUComputePassEncoder {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for GPUComputePassEncoder {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for GPUComputePassEncoder {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for GPUComputePassEncoder {
    fn as_mut(&mut self) -> &mut emlite::Val {
        &mut self.inner
    }
}
impl From<GPUComputePassEncoder> for emlite::Val {
    fn from(s: GPUComputePassEncoder) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}
jsbind::utils::impl_dyn_cast!(GPUComputePassEncoder);

impl GPUComputePassEncoder {
    pub fn set_pipeline(&self, pipeline: GPUComputePipeline) -> jsbind::Undefined {
        self.inner
            .call("setPipeline", &[pipeline.into()])
            .as_::<jsbind::Undefined>()
    }
}
impl GPUComputePassEncoder {
    pub fn dispatch_workgroups0(&self, workgroup_count_x: jsbind::Any) -> jsbind::Undefined {
        self.inner
            .call("dispatchWorkgroups", &[workgroup_count_x.into()])
            .as_::<jsbind::Undefined>()
    }

    pub fn dispatch_workgroups1(
        &self,
        workgroup_count_x: jsbind::Any,
        workgroup_count_y: jsbind::Any,
    ) -> jsbind::Undefined {
        self.inner
            .call(
                "dispatchWorkgroups",
                &[workgroup_count_x.into(), workgroup_count_y.into()],
            )
            .as_::<jsbind::Undefined>()
    }

    pub fn dispatch_workgroups2(
        &self,
        workgroup_count_x: jsbind::Any,
        workgroup_count_y: jsbind::Any,
        workgroup_count_z: jsbind::Any,
    ) -> jsbind::Undefined {
        self.inner
            .call(
                "dispatchWorkgroups",
                &[
                    workgroup_count_x.into(),
                    workgroup_count_y.into(),
                    workgroup_count_z.into(),
                ],
            )
            .as_::<jsbind::Undefined>()
    }
}
impl GPUComputePassEncoder {
    pub fn dispatch_workgroups_indirect(
        &self,
        indirect_buffer: GPUBuffer,
        indirect_offset: jsbind::Any,
    ) -> jsbind::Undefined {
        self.inner
            .call(
                "dispatchWorkgroupsIndirect",
                &[indirect_buffer.into(), indirect_offset.into()],
            )
            .as_::<jsbind::Undefined>()
    }
}
impl GPUComputePassEncoder {
    pub fn end(&self) -> jsbind::Undefined {
        self.inner.call("end", &[]).as_::<jsbind::Undefined>()
    }
}
impl GPUComputePassEncoder {
    pub fn label(&self) -> jsbind::USVString {
        self.inner.get("label").as_::<jsbind::USVString>()
    }

    pub fn set_label(&mut self, value: jsbind::USVString) {
        self.inner.set("label", value);
    }
}
impl GPUComputePassEncoder {
    pub fn push_debug_group(&self, group_label: jsbind::USVString) -> jsbind::Undefined {
        self.inner
            .call("pushDebugGroup", &[group_label.into()])
            .as_::<jsbind::Undefined>()
    }
}
impl GPUComputePassEncoder {
    pub fn pop_debug_group(&self) -> jsbind::Undefined {
        self.inner
            .call("popDebugGroup", &[])
            .as_::<jsbind::Undefined>()
    }
}
impl GPUComputePassEncoder {
    pub fn insert_debug_marker(&self, marker_label: jsbind::USVString) -> jsbind::Undefined {
        self.inner
            .call("insertDebugMarker", &[marker_label.into()])
            .as_::<jsbind::Undefined>()
    }
}
impl GPUComputePassEncoder {
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
