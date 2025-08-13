use super::*;




/// The GPURenderPassDescriptor dictionary.
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
    /// Getter of the `colorAttachments` attribute.
    pub fn color_attachments(&self) -> TypedArray<GPURenderPassColorAttachment> {
        self.inner.get("colorAttachments").as_::<TypedArray<GPURenderPassColorAttachment>>()
    }

    /// Setter of the `colorAttachments` attribute.
    pub fn set_color_attachments(&mut self, value: &TypedArray<GPURenderPassColorAttachment>) {
        self.inner.set("colorAttachments", value);
    }
}
impl GPURenderPassDescriptor {
    /// Getter of the `depthStencilAttachment` attribute.
    pub fn depth_stencil_attachment(&self) -> GPURenderPassDepthStencilAttachment {
        self.inner.get("depthStencilAttachment").as_::<GPURenderPassDepthStencilAttachment>()
    }

    /// Setter of the `depthStencilAttachment` attribute.
    pub fn set_depth_stencil_attachment(&mut self, value: &GPURenderPassDepthStencilAttachment) {
        self.inner.set("depthStencilAttachment", value);
    }
}
impl GPURenderPassDescriptor {
    /// Getter of the `occlusionQuerySet` attribute.
    pub fn occlusion_query_set(&self) -> GPUQuerySet {
        self.inner.get("occlusionQuerySet").as_::<GPUQuerySet>()
    }

    /// Setter of the `occlusionQuerySet` attribute.
    pub fn set_occlusion_query_set(&mut self, value: &GPUQuerySet) {
        self.inner.set("occlusionQuerySet", value);
    }
}
impl GPURenderPassDescriptor {
    /// Getter of the `timestampWrites` attribute.
    pub fn timestamp_writes(&self) -> GPURenderPassTimestampWrites {
        self.inner.get("timestampWrites").as_::<GPURenderPassTimestampWrites>()
    }

    /// Setter of the `timestampWrites` attribute.
    pub fn set_timestamp_writes(&mut self, value: &GPURenderPassTimestampWrites) {
        self.inner.set("timestampWrites", value);
    }
}
impl GPURenderPassDescriptor {
    /// Getter of the `maxDrawCount` attribute.
    pub fn max_draw_count(&self) -> Any {
        self.inner.get("maxDrawCount").as_::<Any>()
    }

    /// Setter of the `maxDrawCount` attribute.
    pub fn set_max_draw_count(&mut self, value: &Any) {
        self.inner.set("maxDrawCount", value);
    }
}
