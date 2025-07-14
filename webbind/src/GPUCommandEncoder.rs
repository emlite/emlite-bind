use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct GPURenderPassDescriptor {
    inner: emlite::Val,
}
impl FromVal for GPURenderPassDescriptor {
    fn from_val(v: &emlite::Val) -> Self {
        GPURenderPassDescriptor { inner: v.clone() }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for GPURenderPassDescriptor {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for GPURenderPassDescriptor {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for GPURenderPassDescriptor {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for GPURenderPassDescriptor {
    fn as_mut(&mut self) -> &mut emlite::Val {
        &mut self.inner
    }
}
impl From<GPURenderPassDescriptor> for emlite::Val {
    fn from(s: GPURenderPassDescriptor) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl GPURenderPassDescriptor {
    pub fn color_attachments(&self) -> jsbind::Sequence<jsbind::Any> {
        self.inner
            .get("colorAttachments")
            .as_::<jsbind::Sequence<jsbind::Any>>()
    }

    pub fn set_color_attachments(&mut self, value: jsbind::Sequence<jsbind::Any>) {
        self.inner.set("colorAttachments", value);
    }
}
impl GPURenderPassDescriptor {
    pub fn depth_stencil_attachment(&self) -> jsbind::Any {
        self.inner
            .get("depthStencilAttachment")
            .as_::<jsbind::Any>()
    }

    pub fn set_depth_stencil_attachment(&mut self, value: jsbind::Any) {
        self.inner.set("depthStencilAttachment", value);
    }
}
impl GPURenderPassDescriptor {
    pub fn occlusion_query_set(&self) -> GPUQuerySet {
        self.inner.get("occlusionQuerySet").as_::<GPUQuerySet>()
    }

    pub fn set_occlusion_query_set(&mut self, value: GPUQuerySet) {
        self.inner.set("occlusionQuerySet", value);
    }
}
impl GPURenderPassDescriptor {
    pub fn timestamp_writes(&self) -> jsbind::Any {
        self.inner.get("timestampWrites").as_::<jsbind::Any>()
    }

    pub fn set_timestamp_writes(&mut self, value: jsbind::Any) {
        self.inner.set("timestampWrites", value);
    }
}
impl GPURenderPassDescriptor {
    pub fn max_draw_count(&self) -> jsbind::Any {
        self.inner.get("maxDrawCount").as_::<jsbind::Any>()
    }

    pub fn set_max_draw_count(&mut self, value: jsbind::Any) {
        self.inner.set("maxDrawCount", value);
    }
}
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct GPUComputePassDescriptor {
    inner: emlite::Val,
}
impl FromVal for GPUComputePassDescriptor {
    fn from_val(v: &emlite::Val) -> Self {
        GPUComputePassDescriptor { inner: v.clone() }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for GPUComputePassDescriptor {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for GPUComputePassDescriptor {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for GPUComputePassDescriptor {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for GPUComputePassDescriptor {
    fn as_mut(&mut self) -> &mut emlite::Val {
        &mut self.inner
    }
}
impl From<GPUComputePassDescriptor> for emlite::Val {
    fn from(s: GPUComputePassDescriptor) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl GPUComputePassDescriptor {
    pub fn timestamp_writes(&self) -> jsbind::Any {
        self.inner.get("timestampWrites").as_::<jsbind::Any>()
    }

    pub fn set_timestamp_writes(&mut self, value: jsbind::Any) {
        self.inner.set("timestampWrites", value);
    }
}
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct GPUTexelCopyBufferInfo {
    inner: emlite::Val,
}
impl FromVal for GPUTexelCopyBufferInfo {
    fn from_val(v: &emlite::Val) -> Self {
        GPUTexelCopyBufferInfo { inner: v.clone() }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for GPUTexelCopyBufferInfo {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for GPUTexelCopyBufferInfo {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for GPUTexelCopyBufferInfo {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for GPUTexelCopyBufferInfo {
    fn as_mut(&mut self) -> &mut emlite::Val {
        &mut self.inner
    }
}
impl From<GPUTexelCopyBufferInfo> for emlite::Val {
    fn from(s: GPUTexelCopyBufferInfo) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl GPUTexelCopyBufferInfo {
    pub fn buffer(&self) -> GPUBuffer {
        self.inner.get("buffer").as_::<GPUBuffer>()
    }

    pub fn set_buffer(&mut self, value: GPUBuffer) {
        self.inner.set("buffer", value);
    }
}
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct GPUTexelCopyTextureInfo {
    inner: emlite::Val,
}
impl FromVal for GPUTexelCopyTextureInfo {
    fn from_val(v: &emlite::Val) -> Self {
        GPUTexelCopyTextureInfo { inner: v.clone() }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for GPUTexelCopyTextureInfo {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for GPUTexelCopyTextureInfo {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for GPUTexelCopyTextureInfo {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for GPUTexelCopyTextureInfo {
    fn as_mut(&mut self) -> &mut emlite::Val {
        &mut self.inner
    }
}
impl From<GPUTexelCopyTextureInfo> for emlite::Val {
    fn from(s: GPUTexelCopyTextureInfo) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl GPUTexelCopyTextureInfo {
    pub fn texture(&self) -> GPUTexture {
        self.inner.get("texture").as_::<GPUTexture>()
    }

    pub fn set_texture(&mut self, value: GPUTexture) {
        self.inner.set("texture", value);
    }
}
impl GPUTexelCopyTextureInfo {
    pub fn mip_level(&self) -> jsbind::Any {
        self.inner.get("mipLevel").as_::<jsbind::Any>()
    }

    pub fn set_mip_level(&mut self, value: jsbind::Any) {
        self.inner.set("mipLevel", value);
    }
}
impl GPUTexelCopyTextureInfo {
    pub fn origin(&self) -> jsbind::Any {
        self.inner.get("origin").as_::<jsbind::Any>()
    }

    pub fn set_origin(&mut self, value: jsbind::Any) {
        self.inner.set("origin", value);
    }
}
impl GPUTexelCopyTextureInfo {
    pub fn aspect(&self) -> GPUTextureAspect {
        self.inner.get("aspect").as_::<GPUTextureAspect>()
    }

    pub fn set_aspect(&mut self, value: GPUTextureAspect) {
        self.inner.set("aspect", value);
    }
}
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct GPUCommandBufferDescriptor {
    inner: emlite::Val,
}
impl FromVal for GPUCommandBufferDescriptor {
    fn from_val(v: &emlite::Val) -> Self {
        GPUCommandBufferDescriptor { inner: v.clone() }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for GPUCommandBufferDescriptor {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for GPUCommandBufferDescriptor {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for GPUCommandBufferDescriptor {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for GPUCommandBufferDescriptor {
    fn as_mut(&mut self) -> &mut emlite::Val {
        &mut self.inner
    }
}
impl From<GPUCommandBufferDescriptor> for emlite::Val {
    fn from(s: GPUCommandBufferDescriptor) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct GPUCommandEncoder {
    inner: emlite::Val,
}
impl FromVal for GPUCommandEncoder {
    fn from_val(v: &emlite::Val) -> Self {
        GPUCommandEncoder {
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
impl core::ops::Deref for GPUCommandEncoder {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for GPUCommandEncoder {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for GPUCommandEncoder {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for GPUCommandEncoder {
    fn as_mut(&mut self) -> &mut emlite::Val {
        &mut self.inner
    }
}
impl From<GPUCommandEncoder> for emlite::Val {
    fn from(s: GPUCommandEncoder) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}
jsbind::utils::impl_dyn_cast!(GPUCommandEncoder);

impl GPUCommandEncoder {
    pub fn begin_render_pass(&self, descriptor: GPURenderPassDescriptor) -> GPURenderPassEncoder {
        self.inner
            .call("beginRenderPass", &[descriptor.into()])
            .as_::<GPURenderPassEncoder>()
    }
}
impl GPUCommandEncoder {
    pub fn begin_compute_pass0(&self) -> GPUComputePassEncoder {
        self.inner
            .call("beginComputePass", &[])
            .as_::<GPUComputePassEncoder>()
    }

    pub fn begin_compute_pass1(
        &self,
        descriptor: GPUComputePassDescriptor,
    ) -> GPUComputePassEncoder {
        self.inner
            .call("beginComputePass", &[descriptor.into()])
            .as_::<GPUComputePassEncoder>()
    }
}
impl GPUCommandEncoder {
    pub fn copy_buffer_to_buffer0(
        &self,
        source: GPUBuffer,
        source_offset: jsbind::Any,
        destination: GPUBuffer,
        destination_offset: jsbind::Any,
    ) -> jsbind::Undefined {
        self.inner
            .call(
                "copyBufferToBuffer",
                &[
                    source.into(),
                    source_offset.into(),
                    destination.into(),
                    destination_offset.into(),
                ],
            )
            .as_::<jsbind::Undefined>()
    }

    pub fn copy_buffer_to_buffer1(
        &self,
        source: GPUBuffer,
        source_offset: jsbind::Any,
        destination: GPUBuffer,
        destination_offset: jsbind::Any,
        size: jsbind::Any,
    ) -> jsbind::Undefined {
        self.inner
            .call(
                "copyBufferToBuffer",
                &[
                    source.into(),
                    source_offset.into(),
                    destination.into(),
                    destination_offset.into(),
                    size.into(),
                ],
            )
            .as_::<jsbind::Undefined>()
    }
}
impl GPUCommandEncoder {
    pub fn copy_buffer_to_texture(
        &self,
        source: GPUTexelCopyBufferInfo,
        destination: GPUTexelCopyTextureInfo,
        copy_size: jsbind::Any,
    ) -> jsbind::Undefined {
        self.inner
            .call(
                "copyBufferToTexture",
                &[source.into(), destination.into(), copy_size.into()],
            )
            .as_::<jsbind::Undefined>()
    }
}
impl GPUCommandEncoder {
    pub fn copy_texture_to_buffer(
        &self,
        source: GPUTexelCopyTextureInfo,
        destination: GPUTexelCopyBufferInfo,
        copy_size: jsbind::Any,
    ) -> jsbind::Undefined {
        self.inner
            .call(
                "copyTextureToBuffer",
                &[source.into(), destination.into(), copy_size.into()],
            )
            .as_::<jsbind::Undefined>()
    }
}
impl GPUCommandEncoder {
    pub fn copy_texture_to_texture(
        &self,
        source: GPUTexelCopyTextureInfo,
        destination: GPUTexelCopyTextureInfo,
        copy_size: jsbind::Any,
    ) -> jsbind::Undefined {
        self.inner
            .call(
                "copyTextureToTexture",
                &[source.into(), destination.into(), copy_size.into()],
            )
            .as_::<jsbind::Undefined>()
    }
}
impl GPUCommandEncoder {
    pub fn clear_buffer0(&self, buffer: GPUBuffer) -> jsbind::Undefined {
        self.inner
            .call("clearBuffer", &[buffer.into()])
            .as_::<jsbind::Undefined>()
    }

    pub fn clear_buffer1(&self, buffer: GPUBuffer, offset: jsbind::Any) -> jsbind::Undefined {
        self.inner
            .call("clearBuffer", &[buffer.into(), offset.into()])
            .as_::<jsbind::Undefined>()
    }

    pub fn clear_buffer2(
        &self,
        buffer: GPUBuffer,
        offset: jsbind::Any,
        size: jsbind::Any,
    ) -> jsbind::Undefined {
        self.inner
            .call("clearBuffer", &[buffer.into(), offset.into(), size.into()])
            .as_::<jsbind::Undefined>()
    }
}
impl GPUCommandEncoder {
    pub fn resolve_query_set(
        &self,
        query_set: GPUQuerySet,
        first_query: jsbind::Any,
        query_count: jsbind::Any,
        destination: GPUBuffer,
        destination_offset: jsbind::Any,
    ) -> jsbind::Undefined {
        self.inner
            .call(
                "resolveQuerySet",
                &[
                    query_set.into(),
                    first_query.into(),
                    query_count.into(),
                    destination.into(),
                    destination_offset.into(),
                ],
            )
            .as_::<jsbind::Undefined>()
    }
}
impl GPUCommandEncoder {
    pub fn finish0(&self) -> GPUCommandBuffer {
        self.inner.call("finish", &[]).as_::<GPUCommandBuffer>()
    }

    pub fn finish1(&self, descriptor: GPUCommandBufferDescriptor) -> GPUCommandBuffer {
        self.inner
            .call("finish", &[descriptor.into()])
            .as_::<GPUCommandBuffer>()
    }
}
impl GPUCommandEncoder {
    pub fn label(&self) -> jsbind::USVString {
        self.inner.get("label").as_::<jsbind::USVString>()
    }

    pub fn set_label(&mut self, value: jsbind::USVString) {
        self.inner.set("label", value);
    }
}
impl GPUCommandEncoder {
    pub fn push_debug_group(&self, group_label: jsbind::USVString) -> jsbind::Undefined {
        self.inner
            .call("pushDebugGroup", &[group_label.into()])
            .as_::<jsbind::Undefined>()
    }
}
impl GPUCommandEncoder {
    pub fn pop_debug_group(&self) -> jsbind::Undefined {
        self.inner
            .call("popDebugGroup", &[])
            .as_::<jsbind::Undefined>()
    }
}
impl GPUCommandEncoder {
    pub fn insert_debug_marker(&self, marker_label: jsbind::USVString) -> jsbind::Undefined {
        self.inner
            .call("insertDebugMarker", &[marker_label.into()])
            .as_::<jsbind::Undefined>()
    }
}
