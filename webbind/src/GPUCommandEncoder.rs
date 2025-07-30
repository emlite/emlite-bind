use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct GPURenderPassDescriptor {
    inner: Any,
}
impl FromVal for GPURenderPassDescriptor {
    fn from_val(v: &Any) -> Self {
        GPURenderPassDescriptor { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for GPURenderPassDescriptor {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for GPURenderPassDescriptor {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for GPURenderPassDescriptor {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for GPURenderPassDescriptor {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<GPURenderPassDescriptor> for Any {
    fn from(s: GPURenderPassDescriptor) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&GPURenderPassDescriptor> for Any {
    fn from(s: &GPURenderPassDescriptor) -> Any {
        s.inner.clone()
    }
}

impl GPURenderPassDescriptor {
    pub fn color_attachments(&self) -> TypedArray<Any> {
        self.inner.get("colorAttachments").as_::<TypedArray<Any>>()
    }

    pub fn set_color_attachments(&mut self, value: &TypedArray<Any>) {
        self.inner.set("colorAttachments", value);
    }
}
impl GPURenderPassDescriptor {
    pub fn depth_stencil_attachment(&self) -> Any {
        self.inner.get("depthStencilAttachment").as_::<Any>()
    }

    pub fn set_depth_stencil_attachment(&mut self, value: &Any) {
        self.inner.set("depthStencilAttachment", value);
    }
}
impl GPURenderPassDescriptor {
    pub fn occlusion_query_set(&self) -> GPUQuerySet {
        self.inner.get("occlusionQuerySet").as_::<GPUQuerySet>()
    }

    pub fn set_occlusion_query_set(&mut self, value: &GPUQuerySet) {
        self.inner.set("occlusionQuerySet", value);
    }
}
impl GPURenderPassDescriptor {
    pub fn timestamp_writes(&self) -> Any {
        self.inner.get("timestampWrites").as_::<Any>()
    }

    pub fn set_timestamp_writes(&mut self, value: &Any) {
        self.inner.set("timestampWrites", value);
    }
}
impl GPURenderPassDescriptor {
    pub fn max_draw_count(&self) -> Any {
        self.inner.get("maxDrawCount").as_::<Any>()
    }

    pub fn set_max_draw_count(&mut self, value: &Any) {
        self.inner.set("maxDrawCount", value);
    }
}
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct GPUComputePassDescriptor {
    inner: Any,
}
impl FromVal for GPUComputePassDescriptor {
    fn from_val(v: &Any) -> Self {
        GPUComputePassDescriptor { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for GPUComputePassDescriptor {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for GPUComputePassDescriptor {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for GPUComputePassDescriptor {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for GPUComputePassDescriptor {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<GPUComputePassDescriptor> for Any {
    fn from(s: GPUComputePassDescriptor) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&GPUComputePassDescriptor> for Any {
    fn from(s: &GPUComputePassDescriptor) -> Any {
        s.inner.clone()
    }
}

impl GPUComputePassDescriptor {
    pub fn timestamp_writes(&self) -> Any {
        self.inner.get("timestampWrites").as_::<Any>()
    }

    pub fn set_timestamp_writes(&mut self, value: &Any) {
        self.inner.set("timestampWrites", value);
    }
}
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct GPUTexelCopyBufferInfo {
    inner: Any,
}
impl FromVal for GPUTexelCopyBufferInfo {
    fn from_val(v: &Any) -> Self {
        GPUTexelCopyBufferInfo { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for GPUTexelCopyBufferInfo {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for GPUTexelCopyBufferInfo {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for GPUTexelCopyBufferInfo {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for GPUTexelCopyBufferInfo {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<GPUTexelCopyBufferInfo> for Any {
    fn from(s: GPUTexelCopyBufferInfo) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&GPUTexelCopyBufferInfo> for Any {
    fn from(s: &GPUTexelCopyBufferInfo) -> Any {
        s.inner.clone()
    }
}

impl GPUTexelCopyBufferInfo {
    pub fn buffer(&self) -> GPUBuffer {
        self.inner.get("buffer").as_::<GPUBuffer>()
    }

    pub fn set_buffer(&mut self, value: &GPUBuffer) {
        self.inner.set("buffer", value);
    }
}
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct GPUTexelCopyTextureInfo {
    inner: Any,
}
impl FromVal for GPUTexelCopyTextureInfo {
    fn from_val(v: &Any) -> Self {
        GPUTexelCopyTextureInfo { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for GPUTexelCopyTextureInfo {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for GPUTexelCopyTextureInfo {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for GPUTexelCopyTextureInfo {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for GPUTexelCopyTextureInfo {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<GPUTexelCopyTextureInfo> for Any {
    fn from(s: GPUTexelCopyTextureInfo) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&GPUTexelCopyTextureInfo> for Any {
    fn from(s: &GPUTexelCopyTextureInfo) -> Any {
        s.inner.clone()
    }
}

impl GPUTexelCopyTextureInfo {
    pub fn texture(&self) -> GPUTexture {
        self.inner.get("texture").as_::<GPUTexture>()
    }

    pub fn set_texture(&mut self, value: &GPUTexture) {
        self.inner.set("texture", value);
    }
}
impl GPUTexelCopyTextureInfo {
    pub fn mip_level(&self) -> Any {
        self.inner.get("mipLevel").as_::<Any>()
    }

    pub fn set_mip_level(&mut self, value: &Any) {
        self.inner.set("mipLevel", value);
    }
}
impl GPUTexelCopyTextureInfo {
    pub fn origin(&self) -> Any {
        self.inner.get("origin").as_::<Any>()
    }

    pub fn set_origin(&mut self, value: &Any) {
        self.inner.set("origin", value);
    }
}
impl GPUTexelCopyTextureInfo {
    pub fn aspect(&self) -> GPUTextureAspect {
        self.inner.get("aspect").as_::<GPUTextureAspect>()
    }

    pub fn set_aspect(&mut self, value: &GPUTextureAspect) {
        self.inner.set("aspect", value);
    }
}
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct GPUCommandBufferDescriptor {
    inner: Any,
}
impl FromVal for GPUCommandBufferDescriptor {
    fn from_val(v: &Any) -> Self {
        GPUCommandBufferDescriptor { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for GPUCommandBufferDescriptor {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for GPUCommandBufferDescriptor {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for GPUCommandBufferDescriptor {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for GPUCommandBufferDescriptor {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<GPUCommandBufferDescriptor> for Any {
    fn from(s: GPUCommandBufferDescriptor) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&GPUCommandBufferDescriptor> for Any {
    fn from(s: &GPUCommandBufferDescriptor) -> Any {
        s.inner.clone()
    }
}

/// The GPUCommandEncoder class.
/// [`GPUCommandEncoder`](https://developer.mozilla.org/en-US/docs/Web/API/GPUCommandEncoder)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct GPUCommandEncoder {
    inner: Any,
}
impl FromVal for GPUCommandEncoder {
    fn from_val(v: &Any) -> Self {
        GPUCommandEncoder {
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
impl core::ops::Deref for GPUCommandEncoder {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for GPUCommandEncoder {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for GPUCommandEncoder {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for GPUCommandEncoder {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<GPUCommandEncoder> for Any {
    fn from(s: GPUCommandEncoder) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&GPUCommandEncoder> for Any {
    fn from(s: &GPUCommandEncoder) -> Any {
        s.inner.clone().into()
    }
}
jsbind::utils::impl_dyn_cast!(GPUCommandEncoder);

impl GPUCommandEncoder {
    /// The beginRenderPass method.
    /// [`GPUCommandEncoder.beginRenderPass`](https://developer.mozilla.org/en-US/docs/Web/API/GPUCommandEncoder/beginRenderPass)
    pub fn begin_render_pass(&self, descriptor: &GPURenderPassDescriptor) -> GPURenderPassEncoder {
        self.inner
            .call("beginRenderPass", &[descriptor.into()])
            .as_::<GPURenderPassEncoder>()
    }
}
impl GPUCommandEncoder {
    /// The beginComputePass method.
    /// [`GPUCommandEncoder.beginComputePass`](https://developer.mozilla.org/en-US/docs/Web/API/GPUCommandEncoder/beginComputePass)
    pub fn begin_compute_pass0(&self) -> GPUComputePassEncoder {
        self.inner
            .call("beginComputePass", &[])
            .as_::<GPUComputePassEncoder>()
    }
    /// The beginComputePass method.
    /// [`GPUCommandEncoder.beginComputePass`](https://developer.mozilla.org/en-US/docs/Web/API/GPUCommandEncoder/beginComputePass)
    pub fn begin_compute_pass1(
        &self,
        descriptor: &GPUComputePassDescriptor,
    ) -> GPUComputePassEncoder {
        self.inner
            .call("beginComputePass", &[descriptor.into()])
            .as_::<GPUComputePassEncoder>()
    }
}
impl GPUCommandEncoder {
    /// The copyBufferToBuffer method.
    /// [`GPUCommandEncoder.copyBufferToBuffer`](https://developer.mozilla.org/en-US/docs/Web/API/GPUCommandEncoder/copyBufferToBuffer)
    pub fn copy_buffer_to_buffer0(
        &self,
        source: &GPUBuffer,
        source_offset: &Any,
        destination: &GPUBuffer,
        destination_offset: &Any,
    ) -> Undefined {
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
            .as_::<Undefined>()
    }
    /// The copyBufferToBuffer method.
    /// [`GPUCommandEncoder.copyBufferToBuffer`](https://developer.mozilla.org/en-US/docs/Web/API/GPUCommandEncoder/copyBufferToBuffer)
    pub fn copy_buffer_to_buffer1(
        &self,
        source: &GPUBuffer,
        source_offset: &Any,
        destination: &GPUBuffer,
        destination_offset: &Any,
        size: &Any,
    ) -> Undefined {
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
            .as_::<Undefined>()
    }
}
impl GPUCommandEncoder {
    /// The copyBufferToTexture method.
    /// [`GPUCommandEncoder.copyBufferToTexture`](https://developer.mozilla.org/en-US/docs/Web/API/GPUCommandEncoder/copyBufferToTexture)
    pub fn copy_buffer_to_texture(
        &self,
        source: &GPUTexelCopyBufferInfo,
        destination: &GPUTexelCopyTextureInfo,
        copy_size: &Any,
    ) -> Undefined {
        self.inner
            .call(
                "copyBufferToTexture",
                &[source.into(), destination.into(), copy_size.into()],
            )
            .as_::<Undefined>()
    }
}
impl GPUCommandEncoder {
    /// The copyTextureToBuffer method.
    /// [`GPUCommandEncoder.copyTextureToBuffer`](https://developer.mozilla.org/en-US/docs/Web/API/GPUCommandEncoder/copyTextureToBuffer)
    pub fn copy_texture_to_buffer(
        &self,
        source: &GPUTexelCopyTextureInfo,
        destination: &GPUTexelCopyBufferInfo,
        copy_size: &Any,
    ) -> Undefined {
        self.inner
            .call(
                "copyTextureToBuffer",
                &[source.into(), destination.into(), copy_size.into()],
            )
            .as_::<Undefined>()
    }
}
impl GPUCommandEncoder {
    /// The copyTextureToTexture method.
    /// [`GPUCommandEncoder.copyTextureToTexture`](https://developer.mozilla.org/en-US/docs/Web/API/GPUCommandEncoder/copyTextureToTexture)
    pub fn copy_texture_to_texture(
        &self,
        source: &GPUTexelCopyTextureInfo,
        destination: &GPUTexelCopyTextureInfo,
        copy_size: &Any,
    ) -> Undefined {
        self.inner
            .call(
                "copyTextureToTexture",
                &[source.into(), destination.into(), copy_size.into()],
            )
            .as_::<Undefined>()
    }
}
impl GPUCommandEncoder {
    /// The clearBuffer method.
    /// [`GPUCommandEncoder.clearBuffer`](https://developer.mozilla.org/en-US/docs/Web/API/GPUCommandEncoder/clearBuffer)
    pub fn clear_buffer0(&self, buffer: &GPUBuffer) -> Undefined {
        self.inner
            .call("clearBuffer", &[buffer.into()])
            .as_::<Undefined>()
    }
    /// The clearBuffer method.
    /// [`GPUCommandEncoder.clearBuffer`](https://developer.mozilla.org/en-US/docs/Web/API/GPUCommandEncoder/clearBuffer)
    pub fn clear_buffer1(&self, buffer: &GPUBuffer, offset: &Any) -> Undefined {
        self.inner
            .call("clearBuffer", &[buffer.into(), offset.into()])
            .as_::<Undefined>()
    }
    /// The clearBuffer method.
    /// [`GPUCommandEncoder.clearBuffer`](https://developer.mozilla.org/en-US/docs/Web/API/GPUCommandEncoder/clearBuffer)
    pub fn clear_buffer2(&self, buffer: &GPUBuffer, offset: &Any, size: &Any) -> Undefined {
        self.inner
            .call("clearBuffer", &[buffer.into(), offset.into(), size.into()])
            .as_::<Undefined>()
    }
}
impl GPUCommandEncoder {
    /// The resolveQuerySet method.
    /// [`GPUCommandEncoder.resolveQuerySet`](https://developer.mozilla.org/en-US/docs/Web/API/GPUCommandEncoder/resolveQuerySet)
    pub fn resolve_query_set(
        &self,
        query_set: &GPUQuerySet,
        first_query: &Any,
        query_count: &Any,
        destination: &GPUBuffer,
        destination_offset: &Any,
    ) -> Undefined {
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
            .as_::<Undefined>()
    }
}
impl GPUCommandEncoder {
    /// The finish method.
    /// [`GPUCommandEncoder.finish`](https://developer.mozilla.org/en-US/docs/Web/API/GPUCommandEncoder/finish)
    pub fn finish0(&self) -> GPUCommandBuffer {
        self.inner.call("finish", &[]).as_::<GPUCommandBuffer>()
    }
    /// The finish method.
    /// [`GPUCommandEncoder.finish`](https://developer.mozilla.org/en-US/docs/Web/API/GPUCommandEncoder/finish)
    pub fn finish1(&self, descriptor: &GPUCommandBufferDescriptor) -> GPUCommandBuffer {
        self.inner
            .call("finish", &[descriptor.into()])
            .as_::<GPUCommandBuffer>()
    }
}
impl GPUCommandEncoder {
    /// Getter of the `label` attribute.
    /// [`GPUCommandEncoder.label`](https://developer.mozilla.org/en-US/docs/Web/API/GPUCommandEncoder/label)
    pub fn label(&self) -> JsString {
        self.inner.get("label").as_::<JsString>()
    }

    /// Setter of the `label` attribute.
    /// [`GPUCommandEncoder.label`](https://developer.mozilla.org/en-US/docs/Web/API/GPUCommandEncoder/label)
    pub fn set_label(&mut self, value: &JsString) {
        self.inner.set("label", value);
    }
}
impl GPUCommandEncoder {
    /// The pushDebugGroup method.
    /// [`GPUCommandEncoder.pushDebugGroup`](https://developer.mozilla.org/en-US/docs/Web/API/GPUCommandEncoder/pushDebugGroup)
    pub fn push_debug_group(&self, group_label: &JsString) -> Undefined {
        self.inner
            .call("pushDebugGroup", &[group_label.into()])
            .as_::<Undefined>()
    }
}
impl GPUCommandEncoder {
    /// The popDebugGroup method.
    /// [`GPUCommandEncoder.popDebugGroup`](https://developer.mozilla.org/en-US/docs/Web/API/GPUCommandEncoder/popDebugGroup)
    pub fn pop_debug_group(&self) -> Undefined {
        self.inner.call("popDebugGroup", &[]).as_::<Undefined>()
    }
}
impl GPUCommandEncoder {
    /// The insertDebugMarker method.
    /// [`GPUCommandEncoder.insertDebugMarker`](https://developer.mozilla.org/en-US/docs/Web/API/GPUCommandEncoder/insertDebugMarker)
    pub fn insert_debug_marker(&self, marker_label: &JsString) -> Undefined {
        self.inner
            .call("insertDebugMarker", &[marker_label.into()])
            .as_::<Undefined>()
    }
}
